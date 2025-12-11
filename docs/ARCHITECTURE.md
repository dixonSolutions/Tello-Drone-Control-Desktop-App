# Architecture Overview

This document describes the architecture of the TelloDrone monorepo and how different components interact.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     User Interface Layer                     │
│                                                               │
│  ┌────────────────────────────────────────────────────────┐ │
│  │         SvelteTelloApp (Desktop UI)                    │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐ │ │
│  │  │   Svelte 4   │  │ Tailwind CSS │  │  shadcn-ui  │ │ │
│  │  │  TypeScript  │  │              │  │             │ │ │
│  │  └──────────────┘  └──────────────┘  └─────────────┘ │ │
│  │                                                        │ │
│  │  Components:                                          │ │
│  │  - DroneControl    - VideoFeed                        │ │
│  │  - FaceRecognition - ThemeToggle                      │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ Tauri API
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Application Layer (Rust)                   │
│                                                               │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                 Tauri Backend (Rust)                   │ │
│  │                                                        │ │
│  │  - Drone State Management                             │ │
│  │  - Command Processing                                 │ │
│  │  - IPC with Python Services                           │ │
│  │  - File System Access                                 │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ IPC/WebSocket/REST
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Service Layer (Python)                     │
│                                                               │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                    PythonApp                           │ │
│  │                                                        │ │
│  │  ┌────────────────┐  ┌────────────────┐              │ │
│  │  │ Drone Control  │  │ Face Recognition│              │ │
│  │  │   (djitellopy) │  │    (OpenCV)     │              │ │
│  │  └────────────────┘  └────────────────┘              │ │
│  │                                                        │ │
│  │  ┌────────────────┐  ┌────────────────┐              │ │
│  │  │ Video Stream   │  │ Model Training  │              │ │
│  │  │   Processing   │  │    (LBPH)       │              │ │
│  │  └────────────────┘  └────────────────┘              │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ UDP Protocol
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     Hardware Layer                           │
│                                                               │
│                      Tello Drone                             │
│                                                               │
│  - Flight Control     - Video Camera                         │
│  - Sensors           - WiFi Module                           │
└─────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. SvelteTelloApp (Frontend)

**Technology Stack:**
- Svelte 4 for reactive UI
- TypeScript for type safety
- Tailwind CSS for styling
- shadcn-svelte for UI components
- mode-watcher for theme management

**Key Components:**

#### DroneControl.svelte
- Manages drone connection state
- Sends flight commands (takeoff, land, move)
- Displays drone telemetry (battery, temperature)
- Provides manual control interface

#### VideoFeed.svelte
- Displays live video stream from drone
- Renders video on HTML canvas
- Handles video stream lifecycle

#### FaceRecognition.svelte
- Interface for training face recognition models
- Triggers face recognition from drone camera
- Displays recognition results

#### ThemeToggle.svelte
- Switches between light and dark themes
- Uses CSS variables for seamless transitions

### 2. Tauri Backend (Rust)

**Purpose:** Bridge between UI and Python services

**Key Functions:**

```rust
// Drone Commands
connect_drone()         // Establish connection with drone
disconnect_drone()      // Close connection
send_drone_command()    // Send flight commands
get_drone_status()      // Retrieve telemetry

// Face Recognition
train_face_model()      // Train new face model
start_face_recognition() // Start recognition process
```

**State Management:**
- DroneState: Tracks connection, battery, temperature
- Thread-safe using Mutex
- Async operations using Tokio

### 3. PythonApp (Backend Services)

**Technology Stack:**
- djitellopy for Tello SDK
- OpenCV for computer vision
- LBPH for face recognition

**Key Modules:**

#### main.py
- Entry point for drone control
- Manages drone connection
- Handles command processing

#### face_training/train_face_model.py
- Captures training images
- Trains LBPH face recognition model
- Saves models to disk

#### face_training/run_face_action.py
- Loads trained models
- Performs real-time face recognition
- Returns recognition results

#### tell_video.py
- Streams video from drone
- Processes video frames
- Provides video feed to frontend

### 4. Tello Drone (Hardware)

**Communication:**
- UDP Command Port: 8889
- UDP State Port: 8890
- UDP Video Port: 11111

**Capabilities:**
- Flight control (takeoff, land, move, rotate)
- HD video streaming (720p)
- Sensors (battery, temperature, altitude, speed)

## Data Flow

### Drone Control Flow

```
User Action (UI)
    ↓
DroneControl.svelte (invoke Tauri command)
    ↓
Tauri Backend (send_drone_command)
    ↓
PythonApp (UDP command)
    ↓
Tello Drone (execute command)
    ↓
Tello Drone (send state)
    ↓
PythonApp (receive state)
    ↓
Tauri Backend (update state)
    ↓
DroneControl.svelte (display updated state)
```

### Video Streaming Flow

```
Tello Drone (UDP video stream)
    ↓
PythonApp (receive H.264 stream)
    ↓
PythonApp (decode frames)
    ↓
WebSocket/IPC (send frames)
    ↓
Tauri Backend (forward frames)
    ↓
VideoFeed.svelte (render on canvas)
```

### Face Recognition Flow

```
User Input (name to train)
    ↓
FaceRecognition.svelte
    ↓
Tauri Backend
    ↓
PythonApp (train_face_model.py)
    ↓
Capture images from drone
    ↓
Train LBPH model
    ↓
Save model to disk
    ↓
Return success
    ↓
Update UI
```

## Communication Protocols

### Tauri Commands (Frontend ↔ Rust)

```typescript
// Invoke from Svelte
await invoke('connect_drone');
await invoke('send_drone_command', { command: 'takeoff' });
const status = await invoke('get_drone_status');
```

### IPC (Rust ↔ Python)

**Options for implementation:**
1. WebSocket server in Python, client in Rust
2. REST API in Python, HTTP client in Rust
3. Named pipes / Unix sockets
4. gRPC for high-performance communication

### UDP (Python ↔ Drone)

```python
# Send command
sock.sendto(b'takeoff', ('192.168.10.1', 8889))

# Receive state
data, addr = sock.recvfrom(1518)

# Receive video
video_data, addr = video_sock.recvfrom(2048)
```

## State Management

### Frontend State (Svelte)

```typescript
let connected = false;       // Connection status
let battery = 0;            // Battery percentage
let temperature = 0;        // Temperature in Celsius
let recognizedPerson = '';  // Last recognized person
```

### Backend State (Rust)

```rust
struct DroneState {
    connected: Mutex<bool>,
    battery: Mutex<i32>,
    temperature: Mutex<i32>,
}
```

### Python State

```python
# Global drone instance
tello = Tello()

# Connection state
is_connected = False

# Face recognition models
models = {}  # name -> LBPH model
```

## Security Considerations

1. **Local Network Only**: Drone communication is limited to local WiFi
2. **No Cloud Dependencies**: All processing happens locally
3. **File System Isolation**: Tauri restricts file access
4. **Input Validation**: All commands validated before sending

## Performance Considerations

1. **Video Streaming**: H.264 compression for efficient bandwidth usage
2. **Face Recognition**: LBPH algorithm for fast recognition
3. **Async Operations**: Non-blocking I/O in Rust and Python
4. **Hot Reload**: Vite dev server for fast development

## Future Enhancements

1. **WebRTC**: For lower-latency video streaming
2. **WebSocket**: Replace HTTP polling with WebSocket
3. **Multi-Drone**: Support controlling multiple drones
4. **Cloud Sync**: Optional cloud storage for training data
5. **Gesture Control**: Use computer vision for gesture-based control

