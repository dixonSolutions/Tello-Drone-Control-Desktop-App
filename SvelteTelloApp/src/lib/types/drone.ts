// TypeScript types for Tello drone

export interface TelloCommand {
  type: 'control' | 'query' | 'set';
  command: string;
  params?: Record<string, any>;
}

export interface TelloResponse {
  success: boolean;
  data?: any;
  error?: string;
}

export interface RCControl {
  leftRight: number;    // -100 to 100
  forwardBack: number;  // -100 to 100
  upDown: number;       // -100 to 100
  yaw: number;          // -100 to 100
}

export interface TelloState {
  pitch: number;
  roll: number;
  yaw: number;
  speedX: number;
  speedY: number;
  speedZ: number;
  temperature: { low: number; high: number };
  tof: number;  // Time of Flight distance in cm
  height: number; // cm
  battery: number; // percentage
  barometer: number;
  time: number;
  acceleration: { x: number; y: number; z: number };
}

export interface TelemetryData {
  battery: number;
  temperature: number;
  height: number;
  pitch: number;
  roll: number;
  yaw: number;
  tof: number | null;
}

export type VideoQuality = 'auto' | 'low' | 'medium' | 'high';

export interface FlightLog {
  timestamp: number;
  event: string;
  data?: any;
}

export type DroneMode = 'manual' | 'free_fly' | 'face_follow' | 'tricks';

export interface FreeFlyDebug {
  mode: 'scan' | 'move' | 'evade';
  edgeDensity: number;
  lapVar: number;
  looming: boolean;
  divergence: number;
  tofCm: number | null;
}
