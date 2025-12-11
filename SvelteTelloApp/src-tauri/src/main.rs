// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::net::UdpSocket;
use std::time::{Duration, SystemTime};
use tauri::{State, Manager};

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
    video_socket: Arc<Mutex<Option<UdpSocket>>>,
}

// Command/Response types
#[derive(Serialize, Deserialize)]
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
    let socket = UdpSocket::bind("0.0.0.0:8889")
        .map_err(|e| format!("Failed to bind socket: {}", e))?;
    
    socket.set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| format!("Failed to set timeout: {}", e))?;
    
    // Send command mode
    socket.send_to(b"command", "192.168.10.1:8889")
        .map_err(|e| format!("Failed to send command: {}", e))?;
    
    // Wait for response
    let mut buf = [0u8; 1024];
    match socket.recv_from(&mut buf) {
        Ok((size, _)) => {
            let response = String::from_utf8_lossy(&buf[..size]);
            if response.trim() == "ok" {
                *state.command_socket.lock().unwrap() = Some(socket);
                
                let mut drone = state.drone.lock().unwrap();
                drone.connected = true;
                drone.speed = 50;
                
                Ok(CommandResult {
                    success: true,
                    message: "Connected to drone".to_string(),
                })
            } else {
                Err(format!("Unexpected response: {}", response))
            }
        }
        Err(e) => Err(format!("Connection timeout: {}", e)),
    }
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
    *state.video_socket.lock().unwrap() = None;
    
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
async fn start_video_stream(state: State<'_, AppState>) -> Result<CommandResult, String> {
    send_command(state.clone(), "streamon".to_string()).await
}

#[tauri::command]
async fn stop_video_stream(state: State<'_, AppState>) -> Result<CommandResult, String> {
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
