// Tauri API wrapper for drone commands
import { invoke } from '@tauri-apps/api/tauri';
import type { TelloCommand, TelloResponse, RCControl, TelemetryData } from '$lib/types/drone';
import type { FaceModel, RecognitionResult } from '$lib/types/recognition';

export class TelloAPI {
  // Connection
  static async connect(): Promise<{ success: boolean; message: string }> {
    return await invoke('connect_drone');
  }

  static async disconnect(): Promise<{ success: boolean; message: string }> {
    return await invoke('disconnect_drone');
  }

  // Flight Control
  static async takeoff(): Promise<{ success: boolean; message: string }> {
    return await invoke('takeoff');
  }

  static async land(): Promise<{ success: boolean; message: string }> {
    return await invoke('land');
  }

  static async emergency(): Promise<{ success: boolean; message: string }> {
    return await invoke('emergency');
  }

  static async sendRC(leftRight: number, forwardBack: number, upDown: number, yaw: number): Promise<void> {
    return await invoke('send_rc_control', {
      leftRight,
      forwardBack,
      upDown,
      yaw,
    });
  }

  static async setSpeed(speed: number): Promise<{ success: boolean; message: string }> {
    return await invoke('set_speed', { speed });
  }

  // Tricks
  static async flip(direction: 'f' | 'b' | 'l' | 'r'): Promise<{ success: boolean; message: string }> {
    return await invoke('flip', { direction });
  }

  // Telemetry
  static async getDroneState(): Promise<any> {
    return await invoke('get_drone_state');
  }

  static async getBattery(): Promise<number> {
    return await invoke('get_battery');
  }

  static async getTelemetry(): Promise<TelemetryData> {
    return await invoke('get_telemetry');
  }

  // Video
  static async startVideoStream(): Promise<{ success: boolean; message: string }> {
    return await invoke('start_video_stream');
  }

  static async stopVideoStream(): Promise<{ success: boolean; message: string }> {
    return await invoke('stop_video_stream');
  }

  static async setVideoBitrate(bitrate: number): Promise<{ success: boolean; message: string }> {
    return await invoke('set_video_bitrate', { bitrate });
  }

  // Face Recognition
  static async listFaceModels(): Promise<FaceModel[]> {
    return await invoke('list_face_models');
  }

  static async trainFaceModel(name: string): Promise<{ success: boolean; message: string }> {
    return await invoke('train_face_model', { name });
  }

  static async deleteFaceModel(name: string): Promise<{ success: boolean; message: string }> {
    return await invoke('delete_face_model', { name });
  }

  static async startFaceRecognition(modelName: string): Promise<{ success: boolean; message: string }> {
    return await invoke('start_face_recognition', { modelName });
  }

  static async stopFaceRecognition(): Promise<{ success: boolean; message: string }> {
    return await invoke('stop_face_recognition');
  }

  // Raw command (for advanced use)
  static async sendCommand(command: string): Promise<{ success: boolean; message: string }> {
    return await invoke('send_command', { command });
  }
}

