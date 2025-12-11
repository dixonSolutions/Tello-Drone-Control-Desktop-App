# Tello Drone Control App

A modern, cross-platform desktop application for controlling DJI Tello drones, built with Svelte and Tauri.

## Features

### Flight Control
- Connect/disconnect to Tello drone via WiFi
- Takeoff and landing
- Directional movement controls (forward, back, left, right, up, down)
- Rotation controls (yaw)
- Emergency stop
- Speed adjustment (normal/fast modes)

### Real-time Telemetry
- Battery level monitoring with warnings
- Height tracking
- Temperature monitoring
- Attitude display (pitch, roll, yaw)
- Connection status indicator

### Video Streaming
- Live video feed from drone camera
- Picture capture
- Video recording with timer
- Recognition overlay support

### Face Recognition
- Train custom face recognition models
- Sample collection interface
- Multiple algorithm support (LBPH, ORB, MediaPipe)
- Live face detection and recognition
- Action triggering on recognition

### Autonomous Modes
- Free Fly mode with obstacle avoidance
- Edge detection and optical flow navigation
- Debug information display

### Tricks & Flips
- Flips in all directions (forward, back, left, right)
- Safety checks before execution
- Visual directional controls

### Themes
8 beautiful color themes to choose from:
- **Fedora Blue** - Classic professional blue
- **Ocean Teal** - Calm aquatic tones
- **Sunset Orange** - Warm vibrant hues
- **Forest Green** - Natural earth tones
- **Purple Galaxy** - Deep space vibes
- **Rose Pink** - Soft elegant pink
- **Dark** - Classic dark mode
- **Light** - Clean light mode

### Settings
- Configurable flight speeds
- Battery warning level adjustment
- Auto-land on low battery
- Keyboard controls toggle
- Persistent settings storage

## Prerequisites

- Node.js 18+ and npm
- Rust 1.70+
- DJI Tello or Tello EDU drone
- WiFi connection to drone

## Installation

1. **Clone the repository**
```bash
cd SvelteTelloApp
```

2. **Install dependencies**
```bash
npm install
```

3. **Run development server**
```bash
npm run tauri:dev
```

This will:
- Start the Vite dev server
- Compile the Rust backend
- Open the desktop application

Note: First run may take 5-10 minutes while Rust compiles.

## Building for Production

```bash
npm run tauri:build
```

This creates an installer/executable in `src-tauri/target/release/bundle/`

## Usage

1. **Connect to Drone**
   - Turn on your Tello drone
   - Connect your computer to the Tello WiFi network (TELLO-XXXXXX)
   - Click "Connect" in the app

2. **Flying**
   - Click "Takeoff" to start flying
   - Use the directional controls to move
   - Click "Land" when done

3. **Video**
   - Click "Start Stream" to view live video
   - Use camera icon to capture pictures
   - Start recording to save video

4. **Face Recognition**
   - Train models by entering a name and capturing samples
   - Start recognition to detect trained faces
   - Configure actions to trigger on recognition

5. **Autonomous Mode**
   - Takeoff first
   - Switch to "Modes" tab
   - Click "Start Free Fly" for autonomous navigation

6. **Tricks**
   - Ensure drone is flying with adequate battery
   - Go to "Tricks" tab
   - Click directional arrows to perform flips

## Project Structure

```
SvelteTelloApp/
├── src/                     # Svelte frontend
│   ├── lib/
│   │   ├── components/     # UI components
│   │   ├── stores/         # State management
│   │   ├── types/          # TypeScript definitions
│   │   ├── themes.ts       # Theme system
│   │   └── constants.ts    # Configuration
│   ├── App.svelte          # Main component
│   └── main.ts             # Entry point
├── src-tauri/              # Rust backend
│   ├── src/
│   │   └── main.rs         # Drone control & Tauri commands
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
└── docs/                   # Documentation
```

## Technology Stack

- **Frontend**: Svelte 4, TypeScript, Tailwind CSS
- **UI Components**: shadcn-svelte
- **Desktop Framework**: Tauri 1.5
- **Drone Control**: tellojs-typescript
- **Backend**: Rust with Tokio async runtime (for file operations)
- **Build Tool**: Vite

## Development

### Running Tests
```bash
npm run test
```

### Linting
```bash
npm run lint
```

### Type Checking
```bash
npm run check
```

## Troubleshooting

### Cannot connect to drone
- Ensure you're connected to the Tello WiFi network
- Check that no other app is controlling the drone
- Try restarting the drone

### Video stream not working
- Make sure you've started the stream
- Check that firewall isn't blocking UDP port 11111
- Restart the stream

### Rust compilation errors
- Ensure Rust is installed: `rustup --version`
- Update Rust: `rustup update`
- Clean build: `npm run tauri build -- --clean`

## Safety

- Always fly in an open area
- Keep battery above 15%
- Have a clear landing zone
- Emergency stop is always available
- Drone will auto-land on critical battery

## License

This project is for educational purposes.

## Credits

- Original Python implementation
- DJI for the Tello drone
- Tauri and Svelte communities

## Contributing

Contributions welcome! Please read CONTRIBUTING.md first.

## Support

For issues and questions, please open a GitHub issue.