# Quick Start Guide

Get up and running with the Tello Drone Control application in minutes.

## Prerequisites Checklist

- [ ] Python 3.11+ installed
- [ ] Node.js 20+ installed  
- [ ] Rust installed (from rustup.rs)
- [ ] Git installed

## 5-Minute Setup

### Step 1: Install Python Dependencies (2 min)

```bash
cd PythonApp
pip install -r requirements.txt
```

### Step 2: Install Node Dependencies (2 min)

```bash
cd ../SvelteTelloApp
npm install
```

### Step 3: Run the App (1 min)

```bash
npm run tauri:dev
```

That's it! The application should launch.

## Connecting Your Drone

1. Power on your Tello drone
2. Connect your computer to the drone's WiFi network
   - Network name: `TELLO-XXXXXX`
   - No password required
3. In the app, click "Connect"
4. Wait for connection confirmation
5. Start flying!

## Basic Flight Controls

- **Takeoff**: Click the "Takeoff" button
- **Land**: Click the "Land" button
- **Move**: Use the directional arrows
- **Rotate**: Use the rotation buttons

## Face Recognition Quick Start

1. Enter a name in the "Train New Person" field
2. Click "Train"
3. The drone will capture multiple photos
4. Wait for training to complete
5. Click "Start Recognition" to test

## Troubleshooting

### App won't start
```bash
# Verify installations
python --version    # Should be 3.11+
node --version      # Should be 20+
rustc --version     # Should show Rust version
```

### Drone won't connect
- Check WiFi connection to drone
- Restart the drone
- Restart the application

### Build errors
```bash
# Clear and reinstall
cd SvelteTelloApp
rm -rf node_modules package-lock.json
npm install
```

## Next Steps

- Read the full [README.md](../README.md)
- Check out [SETUP.md](SETUP.md) for detailed setup
- Explore [ARCHITECTURE.md](ARCHITECTURE.md) to understand the system

## Need Help?

1. Check the troubleshooting section in [SETUP.md](SETUP.md)
2. Review the [Architecture](ARCHITECTURE.md) documentation
3. Open an issue on GitHub

