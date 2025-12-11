# Monorepo Setup Guide

This guide will help you set up the TelloDrone monorepo from scratch.

## Prerequisites

### For All Projects
- Git
- A code editor (VS Code recommended)

### For PythonApp
- Python 3.11 or higher
- pip (Python package manager)

### For SvelteTelloApp
- Node.js 20 or higher
- npm (comes with Node.js)
- Rust (latest stable version)
  - Install from: https://rustup.rs/
  - On Windows, you may also need Visual Studio Build Tools

### Platform-Specific Requirements

#### Windows
- Visual Studio 2022 Build Tools (for Rust/Tauri)
- WebView2 (usually pre-installed on Windows 10/11)

#### macOS
- Xcode Command Line Tools: `xcode-select --install`

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y \
  libgtk-3-dev \
  libwebkit2gtk-4.0-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf
```

## Initial Setup

### 1. Clone the Repository

```bash
git clone <your-repository-url>
cd TelloDrone
```

### 2. Set Up Python Backend

```bash
cd PythonApp
pip install -r requirements.txt
```

**Verify installation:**
```bash
python -c "import cv2; import djitellopy; print('Python setup successful!')"
```

### 3. Set Up Svelte + Tauri App

```bash
cd ../SvelteTelloApp
npm install
```

**Verify installation:**
```bash
npm run check
```

## Running the Applications

### Python Backend

```bash
cd PythonApp
python main.py
```

### Svelte Desktop App (Development)

```bash
cd SvelteTelloApp
npm run tauri:dev
```

This will:
1. Start the Vite dev server on port 1420
2. Launch the Tauri window with hot reload enabled

### Building for Production

```bash
cd SvelteTelloApp
npm run tauri:build
```

Build artifacts will be in:
- Windows: `src-tauri/target/release/bundle/msi/`
- macOS: `src-tauri/target/release/bundle/dmg/`
- Linux: `src-tauri/target/release/bundle/deb/` and `bundle/appimage/`

## Development Workflow

### Working on the Svelte App

1. Start development mode:
```bash
cd SvelteTelloApp
npm run tauri:dev
```

2. Make changes to Svelte files in `src/`
3. Changes will hot-reload automatically
4. For Rust changes in `src-tauri/src/`, the app will rebuild and restart

### Working on Python Backend

1. Start the Python app:
```bash
cd PythonApp
python main.py
```

2. Make changes to Python files
3. Restart the application to see changes

## Testing

### Python Linting
```bash
cd PythonApp
pip install flake8
flake8 . --exclude=__pycache__,venv,face_training
```

### Svelte Type Checking
```bash
cd SvelteTelloApp
npm run check
```

## CI/CD

The monorepo includes a GitHub Actions workflow that:
- Runs Python linting
- Builds the Svelte app for Windows, macOS, and Linux
- Creates build artifacts

Workflow location: `.github/workflows/monorepo-ci.yml`

## Troubleshooting

### Tauri: "cargo not found"
Install Rust from https://rustup.rs/ and restart your terminal.

### Tauri: Build fails on Linux
Make sure you have all the required dependencies:
```bash
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
```

### Python: OpenCV import error
```bash
pip install --upgrade opencv-python opencv-contrib-python
```

### npm: Permission errors
On Linux/macOS, avoid using `sudo` with npm. Configure npm to use a different directory:
```bash
mkdir ~/.npm-global
npm config set prefix '~/.npm-global'
echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.profile
source ~/.profile
```

### Drone won't connect
1. Make sure you're connected to the Tello WiFi network
2. Verify the drone is powered on
3. Check firewall isn't blocking UDP ports 8889, 8890, 11111

## Next Steps

1. Read the main [README.md](../README.md) for project overview
2. Check [SvelteTelloApp/README.md](../SvelteTelloApp/README.md) for app-specific details
3. Explore the codebase and make your first contribution

## Useful Commands Reference

### Python
```bash
# Install dependencies
pip install -r requirements.txt

# Run main application
python main.py

# Run face training
cd face_training
python train_face_model.py
```

### Svelte/Tauri
```bash
# Install dependencies
npm install

# Development mode
npm run tauri:dev

# Build for production
npm run tauri:build

# Run Vite dev server only
npm run dev

# Type check
npm run check
```

### Git
```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Stage changes
git add .

# Commit
git commit -m "Description of changes"

# Push
git push origin feature/your-feature-name
```

