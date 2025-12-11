# Tello Drone Control - Monorepo

A comprehensive monorepo for controlling Tello drones with multiple client implementations and a Python backend.

## Project Structure

This monorepo contains multiple independent projects:

```
TelloDrone/
├── PythonApp/              # Python backend for drone control
├── SvelteTelloApp/         # Modern Svelte + Tauri desktop app
├── InstallFrontEnd/        # Legacy Angular frontend
└── .github/workflows/      # CI/CD workflows
```

## Projects

### 1. PythonApp

Python-based backend application for drone control, video streaming, and face recognition.

**Key Features:**
- Direct communication with Tello drone via UDP
- Face recognition using OpenCV
- Video streaming capabilities
- Face training and model management

**Setup:**
```bash
cd PythonApp
pip install -r requirements.txt
python main.py
```

### 2. SvelteTelloApp (Recommended)

Modern desktop application built with Svelte, TypeScript, and Tauri.

**Key Features:**
- Cross-platform desktop app (Windows, macOS, Linux)
- Modern UI with shadcn-svelte components
- Dark and light theme support
- Real-time drone control interface
- Integrated face recognition

**Setup:**
```bash
cd SvelteTelloApp
npm install
npm run tauri:dev
```

**Build:**
```bash
npm run tauri:build
```

For detailed information, see [SvelteTelloApp/README.md](SvelteTelloApp/README.md)

### 3. InstallFrontEnd (Legacy)

Legacy Angular-based frontend application.

**Setup:**
```bash
cd InstallFrontEnd/TelloDroneFrontEnd
npm install
npm start
```

## CI/CD

The monorepo uses GitHub Actions for continuous integration and deployment:

- **Python App**: Linting and testing
- **Svelte Tauri App**: Multi-platform builds (Windows, macOS, Linux)

Workflow file: `.github/workflows/monorepo-ci.yml`

## Development Workflow

### Prerequisites

- **Python 3.11+** for PythonApp
- **Node.js 20+** and npm for frontend applications
- **Rust** (latest stable) for Tauri builds

### Getting Started

1. Clone the repository:
```bash
git clone <your-repo-url>
cd TelloDrone
```

2. Choose your project and follow its setup instructions above.

### Running Multiple Projects

Each project runs independently. You can run multiple projects simultaneously:

```bash
# Terminal 1: Python backend
cd PythonApp
python main.py

# Terminal 2: Svelte app
cd SvelteTelloApp
npm run tauri:dev
```

## Architecture

### Communication Flow

```
Tello Drone <--> PythonApp <--> SvelteTelloApp (Desktop UI)
     UDP           API/WS         Tauri/Svelte
```

- **Drone Communication**: UDP protocol (ports 8889, 8890, 11111)
- **Backend to Frontend**: WebSocket for video, REST API for commands
- **Face Recognition**: Python OpenCV + LBPH

## Build and Deployment

### Building the Svelte Desktop App

The Svelte + Tauri app can be built for multiple platforms:

```bash
cd SvelteTelloApp
npm run tauri:build
```

Artifacts will be in `SvelteTelloApp/src-tauri/target/release/bundle/`

### CI/CD Artifacts

GitHub Actions automatically builds the app for:
- Windows (.msi, .exe)
- macOS (.dmg, .app)
- Linux (.deb, .AppImage)

## Technology Stack

### Frontend
- **Svelte 4** - Reactive UI framework
- **TypeScript** - Type safety
- **Tailwind CSS** - Utility-first CSS
- **shadcn-svelte** - UI component library
- **Tauri 1.5** - Desktop application framework

### Backend
- **Python 3.11+** - Core backend language
- **OpenCV** - Computer vision and face recognition
- **djitellopy** - Tello drone SDK

### DevOps
- **GitHub Actions** - CI/CD
- **Vite** - Frontend build tool
- **Rust** - Tauri backend

## Contributing

1. Create a feature branch
2. Make your changes
3. Run tests and linting
4. Submit a pull request

## License

[Your License Here]

## Troubleshooting

### Tauri Build Issues

**Linux**: Install required dependencies
```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
```

**macOS**: Install Xcode Command Line Tools
```bash
xcode-select --install
```

### Python Dependencies

If you encounter import errors, ensure all dependencies are installed:
```bash
cd PythonApp
pip install -r requirements.txt
```

### Drone Connection Issues

- Ensure your computer is connected to the Tello drone's WiFi network
- Check that no firewall is blocking UDP ports 8889, 8890, 11111
- Verify the drone is powered on and in range

## Roadmap

- [ ] Implement WebSocket video streaming
- [ ] Add autonomous flight modes
- [ ] Enhance face recognition accuracy
- [ ] Add gesture control
- [ ] Multi-drone support
- [ ] Cloud recording and analytics

