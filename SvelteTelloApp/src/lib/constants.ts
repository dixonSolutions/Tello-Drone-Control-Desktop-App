// Constants for drone control
export const DRONE_IP = '192.168.10.1';
export const COMMAND_PORT = 8889;
export const STATE_PORT = 8890;
export const VIDEO_PORT = 11111;

// Speed settings
export const SPEED_NORMAL = 50;
export const SPEED_FAST = 100;

// RC control limits
export const RC_MIN = -100;
export const RC_MAX = 100;

// Altitude limits for autonomous flight
export const ALTITUDE_MIN = 60;  // cm
export const ALTITUDE_MAX = 120; // cm

// Battery thresholds
export const BATTERY_CRITICAL = 15;
export const BATTERY_WARNING = 30;

// Free Fly tuning parameters
export const FREE_FLY_PARAMS = {
  EDGE_LOW: 28.0,
  EDGE_HIGH: 42.0,
  EDGE_BLOCK: 55.0,
  FLOW_LOOMING: 2.2,
  FLOW_DIVERGENCE: 0.30,
  MOVE_FB: 14,
  MOVE_UD: 18,
  MOVE_YW: 28,
  NUDGE_T: 0.45,
  FORWARD_CLEAR_FRAMES: 4,
  TEXTURE_MIN: 25.0,
};

// Face recognition
export const RECOGNITION_HOLD_TIME = 0.2; // seconds before triggering action
export const FACE_MIN_SIZE = 60; // pixels

// Video
export const VIDEO_FPS = 30;
export const VIDEO_WIDTH = 960;
export const VIDEO_HEIGHT = 720;

// Timeouts
export const CONNECTION_TIMEOUT = 5000; // ms
export const COMMAND_TIMEOUT = 3000; // ms
export const TELEMETRY_UPDATE_INTERVAL = 1000; // ms
export const VIDEO_STALL_TIMEOUT = 8000; // ms
