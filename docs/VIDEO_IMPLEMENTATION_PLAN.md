# Video Stream Implementation Plan

## Current Status
The video render loop is active and the canvas is displaying, but **no actual video frames from the drone are being decoded or displayed**. The current implementation only shows a placeholder animation.

## The Problem
1. Tello drone sends H.264 encoded video over UDP port 11111
2. Our Rust backend sends `streamon` command but doesn't listen for or decode the video data
3. The frontend canvas just draws placeholders - no real frames are received

## How Python App Does It
The Python app uses `djitellopy` which:
1. Sends `streamon` command to drone
2. Listens on UDP port 11111 for H.264 packets
3. Uses `av` (PyAV) or `opencv` to decode H.264 frames
4. Returns decoded BGR frames via `get_frame_read()`
5. UI updates every 30ms with `video.get_frame()`

## Solution Options

### Option 1: Full Rust H.264 Decoding (Complex, Best Performance)
**Required Crates:**
- `ffmpeg-next` or `openh264` for H.264 decoding
- `tokio` for async UDP listener
- Base64 encoding to send frames to frontend

**Steps:**
1. Add video listener thread in Rust that binds to UDP 0.0.0.0:11111
2. Collect H.264 NAL units from UDP packets  
3. Feed to H.264 decoder
4. Convert decoded YUV/RGB frames to PNG/JPEG
5. Emit Tauri events with base64 image data
6. Frontend receives events and draws to canvas

**Pros:** Fast, native, no external dependencies
**Cons:** Complex H.264 handling, large binary size

### Option 2: Hybrid Python Bridge (Easier, Slower)
**Steps:**
1. Keep Python video handling code
2. Start Python subprocess from Rust
3. Python decodes video and writes frames to shared memory or temp files
4. Rust reads and forwards to frontend

**Pros:** Reuses working Python code
**Cons:** Slower, requires Python runtime, complex IPC

### Option 3: WebRTC/RTSP Proxy (Production Quality)
**Steps:**
1. Create Rust UDP-to-WebRTC bridge
2. Convert H.264 stream to WebRTC
3. Frontend uses native `<video>` element

**Pros:** Smooth playback, browser-native
**Cons:** Very complex, overkill for this project

## Recommended Approach: Option 1 (Simplified)

### Implementation Steps

#### 1. Update Cargo.toml
```toml
[dependencies]
opencv = { version = "0.88", features = ["videoio", "imgcodecs"] }
base64 = "0.21"
```

#### 2. Create Video Receiver Thread
```rust
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

fn start_video_receiver(app_handle: tauri::AppHandle) {
    std::thread::spawn(move || {
        let socket = UdpSocket::bind("0.0.0.0:11111").expect("Couldn't bind to video port");
        let mut buf = vec![0u8; 2048];
        
        loop {
            match socket.recv(&mut buf) {
                Ok(size) => {
                    // H.264 packet received
                    // TODO: Decode and emit to frontend
                    app_handle.emit_all("video-frame", &buf[..size]).ok();
                }
                Err(e) => eprintln!("Video receive error: {}", e),
            }
        }
    });
}
```

#### 3. Frontend Receives Frames
```typescript
import { listen } from '@tauri-apps/api/event';

listen('video-frame', (event) => {
  // Draw frame to canvas
  const imageData = event.payload;
  // ... decode and render
});
```

## Why It's Not Working Now
The current Rust code:
- ✅ Sends `streamon` command successfully
- ❌ Never listens on UDP port 11111
- ❌ Never decodes H.264 packets
- ❌ Never forwards frames to frontend

The frontend:
- ✅ Has canvas ready
- ✅ Has render loop
- ❌ Never receives actual frame data
- ❌ Just draws placeholder animation

## Next Steps
1. Add UDP video listener on port 11111 in Rust
2. Implement basic H.264 decoding (or use OpenCV's VideoCapture)
3. Emit decoded frames as base64 PNG to frontend
4. Update frontend to receive and draw real frames
5. Add frame rate limiting and buffering

## Temporary Workaround
For now, the app will continue to show the placeholder animation when "streaming". This at least demonstrates the UI flow and all other controls work properly.

