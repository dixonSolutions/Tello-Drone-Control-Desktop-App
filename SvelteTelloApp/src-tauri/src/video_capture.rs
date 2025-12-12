// Video packet receiver for Tello
// Receives UDP H.264 stream from port 11111 and provides latest packet

use std::sync::{Arc, Mutex};
use std::net::UdpSocket;
use std::time::Duration;
use std::thread;

pub struct TelloVideoCapture {
    running: Arc<Mutex<bool>>,
    latest_packet: Arc<Mutex<Option<Vec<u8>>>>,
    receiver_thread: Option<thread::JoinHandle<()>>,
}

impl TelloVideoCapture {
    pub fn new() -> Self {
        Self {
            running: Arc::new(Mutex::new(false)),
            latest_packet: Arc::new(Mutex::new(None)),
            receiver_thread: None,
        }
    }

    pub fn start(&mut self) {
        let running = Arc::clone(&self.running);
        let latest_packet = Arc::clone(&self.latest_packet);
        
        *running.lock().unwrap() = true;
        
        let handle = thread::spawn(move || {
            println!("[TelloVideo] 🎬 Starting UDP receiver on port 11111...");
            
            let socket = match UdpSocket::bind("0.0.0.0:11111") {
                Ok(s) => {
                    println!("[TelloVideo] ✅ Bound to port 11111");
                    s
                }
                Err(e) => {
                    eprintln!("[TelloVideo] ❌ Failed to bind: {}", e);
                    eprintln!("[TelloVideo] 💡 Tip: Make sure no other app is using port 11111");
                    return;
                }
            };
            
            // Send a "kick-start" packet to the drone's video port
            // Some Tello versions need this to start streaming
            println!("[TelloVideo] 📤 Sending video stream kick-start packet...");
            if let Err(e) = socket.send_to(&[0x01], "192.168.10.1:11111") {
                eprintln!("[TelloVideo] ⚠️ Could not send kick-start: {}", e);
            } else {
                println!("[TelloVideo] ✅ Kick-start sent");
            }
            
            socket.set_read_timeout(Some(Duration::from_millis(500))).ok();
            socket.set_nonblocking(false).ok();
            
            let mut buf = vec![0u8; 65536]; // 64KB buffer
            let mut packet_count = 0;
            let mut wait_count = 0;
            
            println!("[TelloVideo] 🎯 Waiting for H.264 packets from drone...");
            println!("[TelloVideo] 💡 Drone may take a few seconds to start streaming");
            
            while *running.lock().unwrap() {
                match socket.recv(&mut buf) {
                    Ok(size) => {
                        if packet_count == 0 {
                            println!("[TelloVideo] 🎉 First packet received! ({} bytes)", size);
                            println!("[TelloVideo] 📦 First 16 bytes: {:02x?}", &buf[..16.min(size)]);
                        }
                        packet_count += 1;
                        wait_count = 0; // Reset wait counter on successful receive
                        
                        // Store raw H.264 packet
                        let packet = buf[..size].to_vec();
                        *latest_packet.lock().unwrap() = Some(packet);
                        
                        if packet_count % 30 == 0 {
                            println!("[TelloVideo] 📊 {} packets received", packet_count);
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock 
                           || e.kind() == std::io::ErrorKind::TimedOut => {
                        // Normal timeout, continue waiting
                        wait_count += 1;
                        if wait_count % 10 == 0 {
                            println!("[TelloVideo] ⏳ Still waiting for packets... ({} attempts)", wait_count);
                        }
                        continue;
                    }
                    Err(e) if e.raw_os_error() == Some(10060) => {
                        // Windows-specific timeout error (WSAETIMEDOUT)
                        // This is normal when no data is available, keep waiting
                        wait_count += 1;
                        if wait_count == 1 {
                            println!("[TelloVideo] ⏳ Waiting for drone to start streaming...");
                        } else if wait_count % 10 == 0 {
                            println!("[TelloVideo] ⏳ Still waiting... ({} attempts, ~{} seconds)", 
                                     wait_count, wait_count / 2);
                        }
                        continue;
                    }
                    Err(e) => {
                        eprintln!("[TelloVideo] ❌ Unexpected error: {} (kind: {:?}, os: {:?})", 
                                  e, e.kind(), e.raw_os_error());
                        // Don't break immediately, might be transient
                        wait_count += 1;
                        if wait_count > 60 {
                            eprintln!("[TelloVideo] ❌ Too many errors, stopping");
                            break;
                        }
                        continue;
                    }
                }
            }
            
            println!("[TelloVideo] 🛑 Video receiver stopped ({} packets total)", packet_count);
        });
        
        self.receiver_thread = Some(handle);
    }

    pub fn get_packet(&self) -> Option<Vec<u8>> {
        self.latest_packet.lock().unwrap().clone()
    }

    pub fn stop(&mut self) {
        *self.running.lock().unwrap() = false;
        
        if let Some(handle) = self.receiver_thread.take() {
            handle.join().ok();
        }
    }
}

impl Drop for TelloVideoCapture {
    fn drop(&mut self) {
        self.stop();
    }
}

