# Quick Start Guide

## Connect and Fly in 5 Steps

### Step 1: Prerequisites Check
- [ ] Tello drone is charged
- [ ] Computer has WiFi
- [ ] Node.js 18+ installed
- [ ] Rust installed

### Step 2: Install (First Time Only)
```bash
cd SvelteTelloApp
npm install
```

### Step 3: Start the App
```bash
npm run tauri:dev
```

**Note**: First launch takes 5-10 minutes to compile Rust. Be patient!

### Step 4: Connect to Drone
1. Turn on your Tello drone (wait for blinking yellow light)
2. On your computer:
   - **Windows**: Settings → Network → WiFi → Connect to TELLO-XXXXXX
   - **macOS**: WiFi menu → Select TELLO-XXXXXX
   - **Linux**: Network settings → Connect to TELLO-XXXXXX
3. In the app, click **Connect** button
4. Wait for "Connected to drone" message

### Step 5: Fly!
1. Click **Takeoff** (drone will hover ~1m high)
2. Use directional arrows to move
3. Click **Land** when done

## Visual Guide

```
┌─────────────────────────────────────────────┐
│  Tello Drone Control      [Theme] [Status]  │
├─────────────────────────────────────────────┤
│                                             │
│  ┌──────────────┐  ┌──────────────────────┐│
│  │              │  │                      ││
│  │   Controls   │  │    Live Video        ││
│  │              │  │                      ││
│  │  [Takeoff]   │  │  [■ No stream]       ││
│  │  [ Land  ]   │  │                      ││
│  │              │  │                      ││
│  │    ↑         │  └──────────────────────┘│
│  │  ← ○ →       │                          │
│  │    ↓         │  Battery: 95%            │
│  │              │  Height:  0cm            │
│  └──────────────┘  Temp:    20°C           │
│                                             │
└─────────────────────────────────────────────┘
```

## Common Issues

### "Connection failed"
- **Problem**: Not connected to Tello WiFi
- **Solution**: Check WiFi settings, ensure TELLO-XXXXXX is connected

### "Takeoff failed: battery too low"
- **Problem**: Battery under 15%
- **Solution**: Charge the drone

### "Video stream inactive"
- **Problem**: Haven't started video yet
- **Solution**: Click "Start Stream" button under video feed

### App won't start
- **Problem**: Missing dependencies
- **Solution**: Run `npm install` again

### Rust compilation errors
- **Problem**: Rust not installed or outdated
- **Solution**: Install/update Rust:
  ```bash
  # Windows
  winget install Rustlang.Rustup
  
  # macOS/Linux
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## Keyboard Shortcuts (Coming Soon)

| Key | Action |
|-----|--------|
| Space | Takeoff/Land |
| W | Forward |
| S | Backward |
| A | Left |
| D | Right |
| Q | Rotate Left |
| E | Rotate Right |
| R | Up |
| F | Down |
| Esc | Emergency Stop |

## Safety Tips

1. **Always fly in open spaces** - At least 3m clearance
2. **Check battery first** - Don't fly under 20%
3. **Have clear landing zone** - Remove obstacles
4. **Emergency stop ready** - Red button always accessible
5. **Respect flight limits** - Don't exceed 10m height indoors

## Feature Tour

### Controls Tab
Basic flight controls and connection management.

### Modes Tab
- **Free Fly**: Autonomous navigation
- Face following (when recognition active)

### Tricks Tab
Perform flips! Requires:
- Battery > 30%
- Drone flying
- Open space (2m+ clearance)

### Models Tab
Train face recognition:
1. Enter your name
2. Click "Start Training"
3. Capture 20 samples
4. Click "Finish Training"
5. Start recognition to detect faces

### Settings Tab
- Adjust flight speeds
- Configure battery warnings
- Enable/disable features

## Theme Selector
Click palette icon (top-right) to choose from 8 themes:
- Fedora Blue (default)
- Ocean Teal
- Sunset Orange
- Forest Green
- Purple Galaxy
- Rose Pink
- Dark
- Light

## Video Features

### Live Stream
1. Connect to drone
2. Click "Start Stream"
3. View live camera feed

### Capture Photos
1. Start stream
2. Click camera icon
3. Photo saved to Pictures folder

### Record Video
1. Start stream
2. Click "Start Recording"
3. Timer shows recording duration
4. Click "Stop Recording" to save

## Getting Help

- Read `README.md` for full documentation
- Check `DEVELOPMENT.md` for technical details
- Review `IMPLEMENTATION_STATUS.md` for feature list
- Open GitHub issues for bugs

## Next Steps

Once comfortable with basic flight:
1. Try different themes
2. Train face recognition models
3. Experiment with Free Fly mode
4. Perform tricks (safely!)
5. Adjust settings to your preference

## Enjoy Flying! 🚁

Remember: Safety first, fun second!
