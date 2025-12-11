# Quick Reference Card

## 🚀 Essential Commands

### Development
```bash
# Start Svelte + Tauri app
cd SvelteTelloApp
npm run tauri:dev

# Start Python backend
cd PythonApp
python main.py
```

### Building
```bash
# Build desktop app
cd SvelteTelloApp
npm run tauri:build
```

### Checking
```bash
# Type check Svelte
npm run check

# Check Rust
cd src-tauri && cargo check

# Lint Python
cd PythonApp
flake8 . --exclude=__pycache__,venv,face_training
```

## 📁 Important Paths

| Path | Description |
|------|-------------|
| `SvelteTelloApp/src/` | Svelte components |
| `SvelteTelloApp/src-tauri/src/` | Rust backend |
| `SvelteTelloApp/src/lib/components/` | UI components |
| `SvelteTelloApp/src/lib/components/ui/` | shadcn components |
| `PythonApp/` | Python backend |
| `docs/` | Documentation |

## 🎨 Key Files

| File | Purpose |
|------|---------|
| `App.svelte` | Main app component |
| `app.css` | Themes + Tailwind |
| `main.rs` | Tauri commands |
| `tailwind.config.js` | Styling config |
| `tauri.conf.json` | App configuration |

## 📦 Installed Packages

### Frontend
- svelte 4.2.8
- typescript 5.3.3
- vite 5.0.11
- tailwindcss 3.4.1
- @tauri-apps/api 1.5.3
- bits-ui 0.11.8
- mode-watcher 0.2.2
- lucide-svelte 0.303.0

### Backend
- tauri 1.5.4
- tokio (async runtime)
- serde (serialization)

## 🎯 Component Import Paths

```typescript
// Main components
import DroneControl from '$lib/components/DroneControl.svelte';
import VideoFeed from '$lib/components/VideoFeed.svelte';
import FaceRecognition from '$lib/components/FaceRecognition.svelte';
import ThemeToggle from '$lib/components/ThemeToggle.svelte';

// UI components
import Button from '$lib/components/ui/button/Button.svelte';
import Card from '$lib/components/ui/card/Card.svelte';
import Input from '$lib/components/ui/input/Input.svelte';
import Label from '$lib/components/ui/label/Label.svelte';

// Utils
import { cn } from '$lib/utils';
```

## 🎨 Tailwind Classes

### Colors
```css
bg-background text-foreground
bg-primary text-primary-foreground
bg-secondary text-secondary-foreground
bg-muted text-muted-foreground
bg-accent text-accent-foreground
bg-destructive text-destructive-foreground
```

### Card
```html
<div class="rounded-lg border bg-card text-card-foreground shadow-sm">
```

### Button
```html
<button class="bg-primary text-primary-foreground hover:bg-primary/90">
```

## 🔧 Tauri Commands

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// Connect to drone
await invoke('connect_drone');

// Send command
await invoke('send_drone_command', { command: 'takeoff' });

// Get status
const status = await invoke('get_drone_status');

// Train face model
await invoke('train_face_model', { name: 'John' });

// Recognize face
const result = await invoke('start_face_recognition');
```

## 🎨 Theme Variables

### CSS Custom Properties
```css
/* Use in components */
background: hsl(var(--background));
color: hsl(var(--foreground));
border: hsl(var(--border));
```

### Toggle Theme
```typescript
import { toggleMode } from 'mode-watcher';
toggleMode(); // Switch between light/dark
```

## 📚 Documentation Links

| Doc | Quick Access |
|-----|--------------|
| Quick Start | [docs/QUICKSTART.md](QUICKSTART.md) |
| Setup | [docs/SETUP.md](SETUP.md) |
| Architecture | [docs/ARCHITECTURE.md](ARCHITECTURE.md) |
| Development | [docs/DEVELOPMENT.md](DEVELOPMENT.md) |
| Migration | [docs/MIGRATION.md](MIGRATION.md) |
| Status | [docs/PROJECT_STATUS.md](PROJECT_STATUS.md) |

## 🐛 Troubleshooting Quick Fixes

### Port in use
```bash
# Kill process on port 1420
# Windows
netstat -ano | findstr :1420
taskkill /PID <PID> /F

# Linux/Mac
lsof -ti:1420 | xargs kill -9
```

### Hot reload not working
```bash
rm -rf node_modules/.vite
npm run tauri:dev
```

### Rust compilation errors
```bash
cd src-tauri
cargo clean
cargo build
```

### TypeScript errors
```bash
npm run check
```

## 📊 Project Stats

- **Projects**: 3 (Python, Svelte, Angular)
- **Components**: 11 Svelte components
- **Docs**: 8 markdown files
- **Languages**: TypeScript, Rust, Python
- **Lines**: 5,500+ total

## 🎯 Development Workflow

1. **Start**: `npm run tauri:dev`
2. **Edit**: Make changes in `src/`
3. **Save**: Auto-reload in dev
4. **Test**: Check functionality
5. **Build**: `npm run tauri:build`

## 🔑 Git Workflow

```bash
# Create branch
git checkout -b feature/my-feature

# Stage changes
git add .

# Commit
git commit -m "feat: add new feature"

# Push
git push origin feature/my-feature
```

## 📱 App Shortcuts

| Action | Shortcut |
|--------|----------|
| Toggle Theme | Click sun/moon icon |
| Connect Drone | Click "Connect" button |
| Takeoff | Click "Takeoff" button |
| Land | Click "Land" button |

## 🌐 Ports

| Service | Port |
|---------|------|
| Vite Dev Server | 1420 |
| Drone Command | 8889 |
| Drone State | 8890 |
| Drone Video | 11111 |

## 📦 Build Outputs

| Platform | Location |
|----------|----------|
| Windows | `src-tauri/target/release/bundle/msi/` |
| macOS | `src-tauri/target/release/bundle/dmg/` |
| Linux | `src-tauri/target/release/bundle/deb/` |

---

**Keep this card handy for quick reference!** 📌

