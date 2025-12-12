// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod video_capture;

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::net::UdpSocket;
use std::time::Duration;
use tauri::{State, Manager};
use video_capture::TelloVideoCapture;

// Drone state structures
#[derive(Default, Clone, Serialize, Deserialize)]
struct DroneState {
    connected: bool,
    flying: bool,
    battery: i32,
    temperature: i32,
    height: i32,
    pitch: i32,
    roll: i32,
    yaw: i32,
    speed: i32,
    video_active: bool,
}

#[derive(Default)]
struct AppState {
    drone: Arc<Mutex<DroneState>>,
    command_socket: Arc<Mutex<Option<UdpSocket>>>,
    state_socket: Arc<Mutex<Option<UdpSocket>>>,
    video_capture: Arc<Mutex<Option<TelloVideoCapture>>>,
}

// Command/Response types
#[derive(Serialize, Deserialize, Debug)]
struct CommandResult {
    success: bool,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct RCControl {
    left_right: i32,
    forward_back: i32,
    up_down: i32,
    yaw: i32,
}

#[derive(Serialize, Deserialize)]
struct TelemetryData {
    battery: i32,
    temperature: i32,
    height: i32,
    pitch: i32,
    roll: i32,
    yaw: i32,
    tof: Option<i32>,
}

// Face recognition types
#[derive(Serialize, Deserialize, Clone)]
struct FaceModel {
    name: String,
    algorithm: String,
    action: String,
    sample_count: i32,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize, Deserialize)]
struct RecognitionResult {
    success: bool,
    name: Option<String>,
    confidence: f32,
    bbox: Option<BoundingBox>,
}

#[derive(Serialize, Deserialize, Clone)]
struct BoundingBox {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

// Tauri commands

#[tauri::command]
async fn connect_drone(state: State<'_, AppState>) -> Result<CommandResult, String> {
    // Check network configuration first
    println!("[Connect] 🌐 Checking network configuration...");
    match std::process::Command::new("ipconfig").output() {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if !output_str.contains("192.168.10.") {
                eprintln!("[Connect] ⚠️  WARNING: Not connected to Tello WiFi network!");
                eprintln!("[Connect] 💡 Please connect to TELLO-XXXXXX WiFi first");
                eprintln!("[Connect] 💡 Your IP should be 192.168.10.x, not 192.168.1.x");
                return Err("Not connected to Tello WiFi network. Please connect to TELLO-XXXXXX WiFi and try again.".to_string());
            } else {
                println!("[Connect] ✅ Detected Tello network (192.168.10.x)");
            }
        }
        Err(_) => {
            println!("[Connect] ⚠️  Could not verify network - proceeding anyway");
        }
    }
    
    // First, clear any existing socket
    *state.command_socket.lock().unwrap() = None;
    
    // Small delay to ensure socket is released
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    println!("[Connect] Creating new UDP socket on 0.0.0.0:8889...");
    
    let socket = match UdpSocket::bind("0.0.0.0:8889") {
        Ok(s) => {
            println!("[Connect] ✅ Socket bound successfully");
            s
        }
        Err(e) => {
            eprintln!("[Connect] ❌ Failed to bind socket: {}", e);
            return Err(format!("Failed to bind socket: {}. Port 8889 may be in use. Close other Tello apps and try again.", e));
        }
    };
    
    socket.set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| format!("Failed to set timeout: {}", e))?;
    
    println!("[Connect] Sending 'command' to 192.168.10.1:8889...");
    
    // Send command mode with retries
    for attempt in 1..=3 {
        match socket.send_to(b"command", "192.168.10.1:8889") {
            Ok(bytes) => {
                println!("[Connect] Attempt {}: Sent {} bytes", attempt, bytes);
            }
            Err(e) => {
                eprintln!("[Connect] ❌ Send error: {}", e);
                return Err(format!("Failed to send command: {}", e));
            }
        }
        
        // Wait for response
        let mut buf = [0u8; 1024];
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let response = String::from_utf8_lossy(&buf[..size]);
                let trimmed = response.trim();
                
                println!("[Connect] Received from {}: '{}' ({} bytes)", addr, trimmed, size);
                
                // Accept "ok" or "OK" as valid response
                if trimmed.eq_ignore_ascii_case("ok") {
                    *state.command_socket.lock().unwrap() = Some(socket);
                    
                    let mut drone = state.drone.lock().unwrap();
                    drone.connected = true;
                    drone.speed = 50;
                    
                    println!("[Connect] ✅ Successfully connected!");
                    
                    return Ok(CommandResult {
                        success: true,
                        message: "Connected to drone".to_string(),
                    });
                } else {
                    println!("[Connect] ⚠️ Unexpected response: '{}', retrying...", trimmed);
                    if attempt < 3 {
                        tokio::time::sleep(Duration::from_millis(300)).await;
                    }
                }
            }
            Err(e) => {
                println!("[Connect] ⚠️ Attempt {} timeout: {}", attempt, e);
                if attempt < 3 {
                    tokio::time::sleep(Duration::from_millis(300)).await;
                } else {
                    return Err(format!("No response from drone after {} attempts. Make sure:\n1. Drone is powered on\n2. You're connected to TELLO-XXXXXX WiFi\n3. No other apps are using the drone", attempt));
                }
            }
        }
    }
    
    Err("Failed to get valid response from drone. Check WiFi connection.".to_string())
}

#[tauri::command]
async fn disconnect_drone(state: State<'_, AppState>) -> Result<CommandResult, String> {
    // Land if flying
    let is_flying = {
        let drone = state.drone.lock().unwrap();
        drone.flying
    };
    
    if is_flying {
        let _ = send_command(state.clone(), "land".to_string()).await;
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    
    *state.command_socket.lock().unwrap() = None;
    *state.state_socket.lock().unwrap() = None;
    *state.video_capture.lock().unwrap() = None;
    
    let mut drone = state.drone.lock().unwrap();
    drone.connected = false;
    drone.flying = false;
    drone.video_active = false;
    
    Ok(CommandResult {
        success: true,
        message: "Disconnected".to_string(),
    })
}

#[tauri::command]
async fn send_command(state: State<'_, AppState>, command: String) -> Result<CommandResult, String> {
    let socket = state.command_socket.lock().unwrap();
    
    if socket.is_none() {
        return Err("Not connected to drone".to_string());
    }
    
    let sock = socket.as_ref().unwrap();
    
    sock.send_to(command.as_bytes(), "192.168.10.1:8889")
        .map_err(|e| format!("Send failed: {}", e))?;
    
    let mut buf = [0u8; 1024];
    match sock.recv_from(&mut buf) {
        Ok((size, _)) => {
            let response = String::from_utf8_lossy(&buf[..size]);
            Ok(CommandResult {
                success: response.trim() == "ok",
                message: response.trim().to_string(),
            })
        }
        Err(e) => Err(format!("Command timeout: {}", e)),
    }
}

#[tauri::command]
async fn takeoff(state: State<'_, AppState>) -> Result<CommandResult, String> {
    let result = send_command(state.clone(), "takeoff".to_string()).await?;
    
    if result.success {
        let mut drone = state.drone.lock().unwrap();
        drone.flying = true;
    }
    
    Ok(result)
}

#[tauri::command]
async fn land(state: State<'_, AppState>) -> Result<CommandResult, String> {
    let result = send_command(state.clone(), "land".to_string()).await?;
    
    if result.success {
        let mut drone = state.drone.lock().unwrap();
        drone.flying = false;
    }
    
    Ok(result)
}

#[tauri::command]
async fn emergency(state: State<'_, AppState>) -> Result<CommandResult, String> {
    let result = send_command(state.clone(), "emergency".to_string()).await?;
    
    let mut drone = state.drone.lock().unwrap();
    drone.flying = false;
    
    Ok(result)
}

#[tauri::command]
async fn send_rc_control(
    state: State<'_, AppState>,
    left_right: i32,
    forward_back: i32,
    up_down: i32,
    yaw: i32,
) -> Result<(), String> {
    let command = format!("rc {} {} {} {}", left_right, forward_back, up_down, yaw);
    let _ = send_command(state, command).await;
    Ok(())
}

#[tauri::command]
async fn set_speed(state: State<'_, AppState>, speed: i32) -> Result<CommandResult, String> {
    let result = send_command(state.clone(), format!("speed {}", speed)).await?;
    
    if result.success {
        let mut drone = state.drone.lock().unwrap();
        drone.speed = speed;
    }
    
    Ok(result)
}

#[tauri::command]
async fn flip(state: State<'_, AppState>, direction: String) -> Result<CommandResult, String> {
    send_command(state, format!("flip {}", direction)).await
}

#[tauri::command]
async fn get_drone_state(state: State<'_, AppState>) -> Result<DroneState, String> {
    let drone = state.drone.lock().unwrap();
    Ok(drone.clone())
}

#[tauri::command]
async fn get_battery(state: State<'_, AppState>) -> Result<i32, String> {
    let result = send_command(state.clone(), "battery?".to_string()).await?;
    
    if let Ok(battery) = result.message.trim().parse::<i32>() {
        let mut drone = state.drone.lock().unwrap();
        drone.battery = battery;
        Ok(battery)
    } else {
        Err("Failed to parse battery".to_string())
    }
}

#[tauri::command]
async fn get_telemetry(state: State<'_, AppState>) -> Result<TelemetryData, String> {
    // Query multiple telemetry values
    let socket = state.command_socket.lock().unwrap();
    
    if socket.is_none() {
        return Err("Not connected".to_string());
    }
    
    let mut telemetry = TelemetryData {
        battery: 0,
        temperature: 0,
        height: 0,
        pitch: 0,
        roll: 0,
        yaw: 0,
        tof: None,
    };
    
    // Helper to query single value
    let query = |cmd: &str| -> Option<i32> {
        let sock = socket.as_ref().unwrap();
        sock.send_to(cmd.as_bytes(), "192.168.10.1:8889").ok()?;
        let mut buf = [0u8; 1024];
        let (size, _) = sock.recv_from(&mut buf).ok()?;
        let response = String::from_utf8_lossy(&buf[..size]);
        response.trim().parse::<i32>().ok()
    };
    
    telemetry.battery = query("battery?").unwrap_or(0);
    telemetry.temperature = query("temp?").unwrap_or(0);
    telemetry.height = query("height?").unwrap_or(0);
    telemetry.pitch = query("pitch?").unwrap_or(0);
    telemetry.roll = query("roll?").unwrap_or(0);
    telemetry.yaw = query("yaw?").unwrap_or(0);
    telemetry.tof = query("tof?");
    
    // Update state
    let mut drone = state.drone.lock().unwrap();
    drone.battery = telemetry.battery;
    drone.temperature = telemetry.temperature;
    drone.height = telemetry.height;
    drone.pitch = telemetry.pitch;
    drone.roll = telemetry.roll;
    drone.yaw = telemetry.yaw;
    
    Ok(telemetry)
}

#[tauri::command]
async fn start_video_stream(state: State<'_, AppState>, app_handle: tauri::AppHandle) -> Result<CommandResult, String> {
    println!("[VideoStream] 🎥 Starting video stream...");
    
    // Send streamon command
    println!("[VideoStream] 📡 Sending 'streamon' command to drone...");
    let result = send_command(state.clone(), "streamon".to_string()).await?;
    
    println!("[VideoStream] ✅ streamon response: {:?}", result);
    
    if result.success {
        println!("[VideoStream] 🚀 Starting video capture...");
        
        // Create and start video capture (like Python's TelloVideo)
        let mut capture = TelloVideoCapture::new();
        capture.start();
        
        // Store capture in state
        *state.video_capture.lock().unwrap() = Some(capture);
        
        // Start frame polling thread (like Python's update_frame loop)
        let video_capture = Arc::clone(&state.video_capture);
        std::thread::spawn(move || {
            println!("[PacketForwarder] 🔄 Starting H.264 packet forwarding at 30fps...");
            let mut packet_num = 0;
            let mut no_packet_count = 0;
            
            loop {
                std::thread::sleep(Duration::from_millis(33)); // ~30fps
                
                let packet_data = {
                    let capture_lock = video_capture.lock().unwrap();
                    if let Some(ref capture) = *capture_lock {
                        capture.get_packet()
                    } else {
                        println!("[PacketForwarder] ⚠️ Capture is None, stopping...");
                        break; // Capture stopped
                    }
                };
                
                if let Some(h264_bytes) = packet_data {
                    if packet_num == 0 {
                        println!("[PacketForwarder] 🎉 First packet ready! ({} bytes)", h264_bytes.len());
                        println!("[PacketForwarder] 📦 First 16 bytes: {:02x?}", &h264_bytes[..16.min(h264_bytes.len())]);
                    }
                    
                    packet_num += 1;
                    no_packet_count = 0;
                    
                    // Encode to base64 and emit as raw H.264
                    use base64::{Engine as _, engine::general_purpose};
                    let encoded = general_purpose::STANDARD.encode(&h264_bytes);
                    
                    if let Err(e) = app_handle.emit_all("video-packet", encoded) {
                        eprintln!("[PacketForwarder] ❌ Failed to emit packet: {}", e);
                    }
                    
                    if packet_num % 30 == 0 {
                        println!("[PacketForwarder] 📊 {} packets forwarded", packet_num);
                    }
                } else {
                    no_packet_count += 1;
                    if no_packet_count == 1 {
                        println!("[PacketForwarder] ⏳ Waiting for first packet from UDP receiver...");
                    } else if no_packet_count % 30 == 0 {
                        println!("[PacketForwarder] ⚠️ Still no packets after {} checks (~{} seconds)", 
                                 no_packet_count, no_packet_count / 30);
                    }
                }
            }
            
            println!("[PacketForwarder] 🛑 Packet forwarding stopped");
        });
        
        println!("[VideoStream] ✅ Video capture and polling started");
    } else {
        println!("[VideoStream] ❌ streamon command failed: {}", result.message);
    }
    
    Ok(result)
}

#[tauri::command]
async fn stop_video_stream(state: State<'_, AppState>) -> Result<CommandResult, String> {
    println!("[VideoStream] 🛑 Stopping video stream...");
    
    // Stop video capture (this will trigger Drop, cleaning up thread)
    *state.video_capture.lock().unwrap() = None;
    
    // Send streamoff command
    send_command(state, "streamoff".to_string()).await
}

#[tauri::command]
async fn set_video_bitrate(state: State<'_, AppState>, bitrate: i32) -> Result<CommandResult, String> {
    send_command(state, format!("setbitrate {}", bitrate)).await
}

// Face recognition commands

#[tauri::command]
async fn list_face_models() -> Result<Vec<FaceModel>, String> {
    // TODO: Implement by calling Python script or listing model files
    Ok(vec![])
}

#[tauri::command]
async fn train_face_model(name: String) -> Result<CommandResult, String> {
    // TODO: Call Python training script
    println!("Training face model for: {}", name);
    Ok(CommandResult {
        success: true,
        message: format!("Training started for {}", name),
    })
}

#[tauri::command]
async fn delete_face_model(name: String) -> Result<CommandResult, String> {
    // TODO: Delete model files
    println!("Deleting face model: {}", name);
    Ok(CommandResult {
        success: true,
        message: format!("Deleted model {}", name),
    })
}

#[tauri::command]
async fn start_face_recognition(model_name: String) -> Result<CommandResult, String> {
    // TODO: Start recognition loop
    println!("Starting face recognition with model: {}", model_name);
    Ok(CommandResult {
        success: true,
        message: "Recognition started".to_string(),
    })
}

#[tauri::command]
async fn stop_face_recognition() -> Result<CommandResult, String> {
    // TODO: Stop recognition
    Ok(CommandResult {
        success: true,
        message: "Recognition stopped".to_string(),
    })
}

#[tauri::command]
async fn open_images_folder() -> Result<CommandResult, String> {
    use std::process::Command;
    
    // Get the standard pictures directory
    let pictures_dir = dirs::picture_dir()
        .ok_or("Could not find pictures directory")?;
    
    let tello_dir = pictures_dir.join("Tello");
    
    // Create directory if it doesn't exist
    std::fs::create_dir_all(&tello_dir)
        .map_err(|e| format!("Failed to create directory: {}", e))?;
    
    // Open folder with system default file manager
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&tello_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&tello_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&tello_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    
    Ok(CommandResult {
        success: true,
        message: format!("Opened folder: {:?}", tello_dir),
    })
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            connect_drone,
            disconnect_drone,
            send_command,
            takeoff,
            land,
            emergency,
            send_rc_control,
            set_speed,
            flip,
            get_drone_state,
            get_battery,
            get_telemetry,
            start_video_stream,
            stop_video_stream,
            set_video_bitrate,
            list_face_models,
            train_face_model,
            delete_face_model,
            start_face_recognition,
            stop_face_recognition,
            open_images_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
