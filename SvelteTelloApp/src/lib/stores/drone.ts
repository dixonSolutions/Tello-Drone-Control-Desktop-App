import { writable, derived } from 'svelte/store';

export interface DroneState {
  connected: boolean;
  flying: boolean;
  battery: number;
  temperature: number;
  height: number; // cm
  pitch: number;  // degrees
  roll: number;   // degrees
  yaw: number;    // degrees
  speed: number;  // current speed setting (50 or 100)
  videoActive: boolean;
}

export interface ConnectionStatus {
  status: 'disconnected' | 'connecting' | 'connected' | 'error';
  message: string;
  lastUpdate: number;
}

function createDroneStore() {
  const { subscribe, set, update } = writable<DroneState>({
    connected: false,
    flying: false,
    battery: 0,
    temperature: 0,
    height: 0,
    pitch: 0,
    roll: 0,
    yaw: 0,
    speed: 50,
    videoActive: false,
  });

  return {
    subscribe,
    update,
    setConnected: (connected: boolean) => update(state => ({ ...state, connected })),
    setFlying: (flying: boolean) => update(state => ({ ...state, flying })),
    setBattery: (battery: number) => update(state => ({ ...state, battery })),
    setTemperature: (temperature: number) => update(state => ({ ...state, temperature })),
    setHeight: (height: number) => update(state => ({ ...state, height })),
    setAttitude: (pitch: number, roll: number, yaw: number) => 
      update(state => ({ ...state, pitch, roll, yaw })),
    setSpeed: (speed: number) => update(state => ({ ...state, speed })),
    setVideoActive: (videoActive: boolean) => update(state => ({ ...state, videoActive })),
    reset: () => set({
      connected: false,
      flying: false,
      battery: 0,
      temperature: 0,
      height: 0,
      pitch: 0,
      roll: 0,
      yaw: 0,
      speed: 50,
      videoActive: false,
    }),
  };
}

function createConnectionStore() {
  const { subscribe, set, update } = writable<ConnectionStatus>({
    status: 'disconnected',
    message: 'Not connected',
    lastUpdate: Date.now(),
  });

  return {
    subscribe,
    setStatus: (status: ConnectionStatus['status'], message: string) => {
      set({ status, message, lastUpdate: Date.now() });
    },
  };
}

export const droneStore = createDroneStore();
export const connectionStore = createConnectionStore();

// Derived store for battery warning
export const batteryWarning = derived(
  droneStore,
  $drone => $drone.battery > 0 && $drone.battery < 15
);
