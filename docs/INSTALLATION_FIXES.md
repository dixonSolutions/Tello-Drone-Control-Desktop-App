# Installation Issues & Fixes

## Issue 1: Rust/Cargo Not Found

The error "program not found" means Rust is not installed on your system.

### Install Rust on Windows

#### Option 1: Using Rustup Installer (Recommended)
1. Download Rust installer from: https://rustup.rs/
2. Run the installer (rustup-init.exe)
3. Follow the prompts (usually just press Enter to accept defaults)
4. **Important**: Restart your terminal/PowerShell after installation
5. Verify installation:
   ```powershell
   rustc --version
   cargo --version
   ```

#### Option 2: Using winget (Windows Package Manager)
```powershell
winget install Rustlang.Rustup
```

### Additional Windows Requirements

You may also need Visual Studio C++ Build Tools:

1. Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
2. Install "Desktop development with C++" workload
3. Or install via winget:
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools --silent --override "--wait --add Microsoft.VisualStudio.Workload.VCTools"
   ```

### After Installing Rust

1. **Close and reopen PowerShell** (important!)
2. Navigate back to the project:
   ```powershell
   cd "C:\Users\Rat Rad\Projects\SideProjects\TelloDrone\SvelteTelloApp"
   ```

## Issue 2: Peer Dependency Conflict (Fixed)

The Svelte vite plugin version was incompatible with Svelte 4.

**Fix Applied**: Changed `@sveltejs/vite-plugin-svelte` from `^4.0.0` to `^3.0.2`

### Reinstall Dependencies

After I've fixed the package.json:

```powershell
# Remove old node_modules and lock file
Remove-Item node_modules -Recurse -Force
Remove-Item package-lock.json -Force

# Clean install
npm install
```

## Issue 3: Security Vulnerabilities

You have 7 moderate severity vulnerabilities. To fix:

```powershell
# Check what they are
npm audit

# Fix them (may cause breaking changes)
npm audit fix

# Or force fix all
npm audit fix --force
```

## Complete Setup Process

Here's the full process to get everything working:

### Step 1: Install Rust
```powershell
# Download and run rustup from https://rustup.rs/
# OR use winget
winget install Rustlang.Rustup

# Close and reopen PowerShell
exit
```

### Step 2: Clean and Reinstall (New PowerShell Window)
```powershell
cd "C:\Users\Rat Rad\Projects\SideProjects\TelloDrone\SvelteTelloApp"

# Remove old installation
Remove-Item node_modules -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item package-lock.json -Force -ErrorAction SilentlyContinue

# Install with fixed dependencies
npm install

# Fix vulnerabilities
npm audit fix
```

### Step 3: Verify Rust Installation
```powershell
rustc --version    # Should show: rustc 1.x.x
cargo --version    # Should show: cargo 1.x.x
```

### Step 4: Run the App
```powershell
npm run tauri:dev
```

## Expected First Run

The first time you run `tauri:dev`, it will:
1. Download Rust dependencies (this takes a few minutes)
2. Compile the Rust backend (also takes a few minutes)
3. Start the Vite dev server
4. Launch the Tauri window

**Be patient on first run - it can take 5-10 minutes!**

## Troubleshooting

### "cargo: command not found" after installing Rust
- **Solution**: Close and reopen PowerShell
- Rust adds to PATH, but PowerShell needs restart to see it

### "linker link.exe not found"
- **Solution**: Install Visual Studio Build Tools
- Rust needs a C++ compiler on Windows

### Port 1420 already in use
```powershell
# Kill process using port 1420
netstat -ano | findstr :1420
# Note the PID, then:
taskkill /PID <PID> /F
```

### Still having issues?
```powershell
# Check environment
rustc --version
node --version
npm --version

# Clean everything
cd SvelteTelloApp
Remove-Item node_modules -Recurse -Force
Remove-Item package-lock.json -Force
Remove-Item src-tauri\target -Recurse -Force

# Fresh install
npm install
npm run tauri:dev
```

## Quick Commands Reference

```powershell
# Check installations
rustc --version
cargo --version
node --version
npm --version

# Clean install
Remove-Item node_modules -Recurse -Force
Remove-Item package-lock.json -Force
npm install

# Run development
npm run tauri:dev

# Build for production
npm run tauri:build
```

## Next Steps

Once Rust is installed and you've reinstalled dependencies:

1. Run `npm run tauri:dev`
2. Wait for the first compile (5-10 minutes)
3. The app window should open
4. You'll see the Tello Drone Control interface

The app will have hot reload enabled, so any changes you make to Svelte files will update automatically!

