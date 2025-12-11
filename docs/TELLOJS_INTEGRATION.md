# tellojs-typescript Integration

## Overview

We're using the `tellojs-typescript` library to control the Tello drone directly from the Svelte frontend. This eliminates the need for a complex Rust backend for drone communication.

## Installation

```bash
npm install tellojs-typescript
```

## Architecture

### Tello Controller (`src/lib/tello.ts`)
Wrapper class around tellojs-typescript that provides:
- Connection management
- Flight control commands
- Telemetry reading
- Video stream handling
- State stream handling

### Integration Points

1. **DroneControl.svelte** - Uses `tello.connect()`, `tello.takeoff()`, etc.
2. **Telemetry.svelte** - Subscribes to state stream for real-time updates
3. **VideoFeed.svelte** - Uses video stream for live feed
4. **Tricks.svelte** - Uses flip commands

## API Reference

### Connection
```typescript
await tello.connect();  // Connect to drone
await tello.disconnect();  // Disconnect
```

### Flight Control
```typescript
await tello.takeoff();  // Takeoff
await tello.land();  // Land
await tello.emergency();  // Emergency stop
await tello.stop();  // Stop all movement
```

### Movement
```typescript
await tello.moveUp(20);  // Move up 20cm
await tello.moveDown(20);  // Move down 20cm
await tello.moveLeft(20);  // Move left 20cm
await tello.moveRight(20);  // Move right 20cm
await tello.moveForward(20);  // Move forward 20cm
await tello.moveBackward(20);  // Move backward 20cm
```

### Rotation
```typescript
await tello.rotateClockwise(90);  // Rotate 90° clockwise
await tello.rotateCounterClockwise(90);  // Rotate 90° counter-clockwise
```

### RC Control (Manual Flight)
```typescript
// Parameters: x (left/right), y (forward/back), z (up/down), yaw (rotation)
// Range: -100 to 100
await tello.sendRC(0, 50, 0, 0);  // Move forward at 50% speed
await tello.sendRC(0, 0, 0, 0);  // Stop all movement
```

### Flips
```typescript
await tello.flipForward();
await tello.flipBackward();
await tello.flipLeft();
await tello.flipRight();
```

### Telemetry
```typescript
const battery = await tello.getBattery();  // Returns number (0-100)
const height = await tello.getHeight();  // Returns height in cm
const temp = await tello.getTemperature();  // Returns temperature in °C
const attitude = await tello.getAttitude();  // Returns {pitch, roll, yaw}
const speed = await tello.getSpeed();  // Returns speed
const tof = await tello.getTOF();  // Returns Time of Flight distance
```

### Settings
```typescript
await tello.setSpeed(50);  // Set speed (10-100 cm/s)
await tello.setWifiSSID('ssid', 'password');  // Change WiFi
```

### State Stream
Real-time state updates via EventEmitter:

```typescript
const stateStream = tello.getStateStream();

stateStream.on('message', (state: string) => {
  // Parse state string
  // Format: "pitch:0;roll:0;yaw:0;vgx:0;vgy:0;vgz:0;templ:54;temph:56;tof:10;h:0;bat:87;baro:28.66;time:0;agx:5.00;agy:-26.00;agz:-1001.00;"
  const parts = state.split(';');
  // Process each part...
});
```

### Video Stream
Live video feed via EventEmitter:

```typescript
const videoStream = await tello.startVideoStream();

videoStream.on('data', (frame: Buffer) => {
  // Process H.264 video frame
  // Decode and display on canvas
});

await tello.stopVideoStream();
```

## State Message Format

The state stream sends messages in this format:
```
pitch:X;roll:X;yaw:X;vgx:X;vgy:X;vgz:X;templ:X;temph:X;tof:X;h:X;bat:X;baro:X;time:X;agx:X;agy:X;agz:X;
```

### Fields
- `pitch` - Pitch angle (-90 to 90)
- `roll` - Roll angle (-90 to 90)
- `yaw` - Yaw angle (-180 to 180)
- `vgx` - X-axis speed (-100 to 100)
- `vgy` - Y-axis speed (-100 to 100)
- `vgz` - Z-axis speed (-100 to 100)
- `templ` - Lowest temperature (°C)
- `temph` - Highest temperature (°C)
- `tof` - Time of Flight distance (cm)
- `h` - Height (cm)
- `bat` - Battery (0-100)
- `baro` - Barometer (m)
- `time` - Motor time
- `agx` - X-axis acceleration
- `agy` - Y-axis acceleration
- `agz` - Z-axis acceleration

## Important Notes

### Connection
1. **WiFi Connection Required**: Computer must be connected to Tello WiFi (TELLO-XXXXXX)
2. **UDP Ports**: The library uses:
   - 8889 for commands
   - 8890 for state
   - 11111 for video

### Movement Commands
- **Distance**: 20-500 cm
- **Speed**: 10-100 cm/s
- **Angle**: 1-360 degrees

### RC Control
- **Range**: -100 to 100 for all axes
- **Continuous**: Send RC commands continuously for smooth flight
- **Stop**: Send (0,0,0,0) to stop all movement

### Flips
- **Requirements**: Battery > 50%, sufficient height, open space
- **Risk**: Can cause crashes if space is insufficient

### Video Stream
- **Format**: H.264 encoded
- **Resolution**: 960x720
- **Frame Rate**: ~30 FPS
- **Decoding**: Requires H.264 decoder (TODO: implement)

## TODO Implementation

### Video Display
The video stream provides raw H.264 frames. To display:

1. **Option 1**: Use Broadway.js (H.264 decoder in JavaScript)
```bash
npm install broadway
```

2. **Option 2**: Use FFmpeg.wasm
```bash
npm install @ffmpeg/ffmpeg
```

3. **Option 3**: Server-side decoding via Tauri Rust backend

### Face Recognition
Integrate face recognition with video stream:
1. Decode video frames
2. Pass frames to face detection algorithm
3. Draw bounding boxes on canvas
4. Trigger actions based on recognition

### Free Fly Mode
Port Python algorithms:
1. Edge detection on video frames
2. Optical flow calculation
3. Autonomous navigation logic
4. Send RC commands based on analysis

## Example Usage

### Basic Flight
```typescript
import { tello } from '$lib/tello';

// Connect
await tello.connect();

// Takeoff
await tello.takeoff();

// Wait 3 seconds
await new Promise(r => setTimeout(r, 3000));

// Move forward 50cm
await tello.moveForward(50);

// Rotate 360°
await tello.rotateClockwise(360);

// Land
await tello.land();

// Disconnect
await tello.disconnect();
```

### RC Control (Smooth Movement)
```typescript
import { tello } from '$lib/tello';

// Takeoff first
await tello.takeoff();

// Fly forward for 2 seconds
const interval = setInterval(() => {
  tello.sendRC(0, 50, 0, 0);  // Forward at 50% speed
}, 50);  // Send every 50ms

setTimeout(() => {
  clearInterval(interval);
  tello.sendRC(0, 0, 0, 0);  // Stop
  tello.land();
}, 2000);
```

### Real-time Telemetry
```typescript
import { tello } from '$lib/tello';

await tello.connect();

const stateStream = tello.getStateStream();

stateStream.on('message', (state: string) => {
  const stateObj = parseState(state);  // Helper function
  
  console.log(`Battery: ${stateObj.bat}%`);
  console.log(`Height: ${stateObj.h}cm`);
  console.log(`Temperature: ${(stateObj.templ + stateObj.temph) / 2}°C`);
});

function parseState(state: string) {
  const obj: any = {};
  state.split(';').forEach(part => {
    const [key, value] = part.split(':');
    if (key && value) obj[key] = parseFloat(value);
  });
  return obj;
}
```

## Troubleshooting

### "Connection failed"
- Not connected to Tello WiFi
- Another app is controlling the drone
- Drone is off or out of range

### "Command timeout"
- Weak WiFi signal
- Drone is busy processing previous command
- Drone is in error state (try emergency stop)

### Video stream not working
- Ensure `streamon` command was successful
- Check UDP port 11111 isn't blocked by firewall
- Video decoder not implemented yet (see TODO)

### State stream no data
- Connection not established
- State stream not bound (call `tello.getStateStream()`)
- UDP port 8890 blocked

## Resources

- [tellojs-typescript GitHub](https://github.com/RelatedTitle/tellojs)
- [Tello SDK Documentation](https://dl-cdn.ryzerobotics.com/downloads/Tello/Tello%20SDK%202.0%20User%20Guide.pdf)
- [Node.js dgram](https://nodejs.org/api/dgram.html)
