# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### To Be Implemented
- Python backend integration
- Video streaming from drone
- Face recognition functionality
- Keyboard shortcuts for drone control
- Settings page
- Flight path recording
- Autonomous flight modes

## [0.1.0] - 2025-12-11

### Added - Initial Monorepo Setup

#### Infrastructure
- Created monorepo structure with separate project folders
- Added GitHub Actions CI/CD workflow for multi-platform builds
- Created comprehensive `.gitignore` for monorepo
- Set up documentation folder with multiple guides

#### SvelteTelloApp (New Desktop Application)
- Initialized Svelte 4 + TypeScript project with Vite
- Integrated Tauri 1.5 for cross-platform desktop support
- Configured Tailwind CSS 3.4 for styling
- Set up PostCSS for CSS processing
- Implemented shadcn-svelte UI component library:
  - Button component with variants (default, destructive, outline, secondary, ghost, link)
  - Card components (Card, CardHeader, CardTitle, CardContent)
  - Input component with styling
  - Label component for forms
- Created dark and light theme support with mode-watcher
- Implemented CSS custom properties for theming

#### UI Components
- **DroneControl.svelte**: Flight control interface
  - Connection management UI
  - Directional control buttons
  - Telemetry display (battery, temperature)
  - Takeoff/land buttons
  - Rotation controls
- **VideoFeed.svelte**: Video stream display component
  - Canvas-based video rendering
  - Placeholder for no video state
- **FaceRecognition.svelte**: Face recognition interface
  - Person name input for training
  - Training button with loading state
  - Recognition trigger button
  - Recognition result display
- **ThemeToggle.svelte**: Theme switcher
  - Sun/moon icon toggle
  - Smooth transitions between themes

#### Backend (Rust/Tauri)
- Set up Tauri project structure
- Implemented command handlers (stubs for implementation):
  - `connect_drone`: Establish drone connection
  - `disconnect_drone`: Close drone connection
  - `send_drone_command`: Send flight commands
  - `get_drone_status`: Retrieve drone telemetry
  - `train_face_model`: Train face recognition model
  - `start_face_recognition`: Perform face recognition
- Configured state management with Mutex for thread safety
- Set up Tokio for async operations
- Created Tauri configuration for multi-platform builds

#### Python Backend
- Created `requirements.txt` with dependencies:
  - djitellopy >= 2.4.0
  - opencv-python >= 4.8.0
  - opencv-contrib-python >= 4.8.0
  - numpy >= 1.24.0
  - Pillow >= 10.0.0
  - pygame >= 2.5.0
- Preserved existing Python codebase intact

#### Documentation
- **README.md**: Main monorepo overview and quick reference
- **SvelteTelloApp/README.md**: App-specific documentation
- **docs/ARCHITECTURE.md**: System architecture and data flow
- **docs/DEVELOPMENT.md**: Development guide and best practices
- **docs/MIGRATION.md**: Angular to Svelte migration guide
- **docs/PROJECT_STATUS.md**: Current status and roadmap
- **docs/QUICKSTART.md**: 5-minute setup guide
- **docs/SETUP.md**: Detailed setup instructions
- **docs/SUMMARY.md**: Complete summary of what was created

#### CI/CD
- Python linting job with flake8
- Multi-platform Tauri build job:
  - Ubuntu (Linux builds: .deb, .AppImage)
  - Windows (builds: .msi, .exe)
  - macOS (builds: .dmg, .app)
- Artifact upload for build outputs
- Automated checks on push and pull requests

#### Configuration Files
- `package.json`: Node dependencies and scripts
- `vite.config.ts`: Vite configuration with Tauri integration
- `tsconfig.json`: TypeScript configuration
- `tailwind.config.js`: Tailwind CSS with shadcn theming
- `postcss.config.js`: PostCSS configuration
- `svelte.config.js`: Svelte preprocessor configuration
- `Cargo.toml`: Rust dependencies
- `tauri.conf.json`: Tauri application configuration

### Project Structure
```
TelloDrone/
├── .github/workflows/monorepo-ci.yml
├── docs/ (8 documentation files)
├── PythonApp/ (existing, with requirements.txt added)
├── InstallFrontEnd/ (existing, unchanged)
├── SvelteTelloApp/ (new, 30+ files)
├── .gitignore
└── README.md
```

### Technology Stack Versions
- Svelte: 4.2.8
- TypeScript: 5.3.3
- Vite: 5.0.11
- Tauri: 1.5.4
- Tailwind CSS: 3.4.1
- bits-ui: 0.11.8
- mode-watcher: 0.2.2
- lucide-svelte: 0.303.0
- Rust: Latest stable

### Notes
- This release establishes the foundation and structure
- Python backend integration is pending
- Tauri commands are implemented as stubs
- Video streaming requires implementation
- Face recognition needs Python bridge
- Icons need to be generated

---

## Release Notes Format

### Added
For new features.

### Changed
For changes in existing functionality.

### Deprecated
For soon-to-be removed features.

### Removed
For now removed features.

### Fixed
For any bug fixes.

### Security
In case of vulnerabilities.

