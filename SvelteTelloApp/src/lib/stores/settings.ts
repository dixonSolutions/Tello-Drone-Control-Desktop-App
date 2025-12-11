import { writable } from 'svelte/store';

export interface AppSettings {
  videoQuality: 'auto' | 'low' | 'medium' | 'high';
  recordingDirectory: string;
  pictureDirectory: string;
  enableKeyboardControls: boolean;
  fastModeSpeed: number;
  normalModeSpeed: number;
  freeFlyEnabled: boolean;
  autoLandOnLowBattery: boolean;
  batteryWarningLevel: number;
}

function createSettingsStore() {
  const defaultSettings: AppSettings = {
    videoQuality: 'auto',
    recordingDirectory: '~/Videos/Tello',
    pictureDirectory: '~/Pictures/Tello',
    enableKeyboardControls: true,
    fastModeSpeed: 100,
    normalModeSpeed: 50,
    freeFlyEnabled: true,
    autoLandOnLowBattery: true,
    batteryWarningLevel: 15,
  };

  // Load from localStorage if available
  const saved = typeof window !== 'undefined' 
    ? localStorage.getItem('appSettings') 
    : null;
  
  const initial = saved ? { ...defaultSettings, ...JSON.parse(saved) } : defaultSettings;

  const { subscribe, set, update } = writable<AppSettings>(initial);

  return {
    subscribe,
    update: (updates: Partial<AppSettings>) => {
      update(settings => {
        const newSettings = { ...settings, ...updates };
        if (typeof window !== 'undefined') {
          localStorage.setItem('appSettings', JSON.stringify(newSettings));
        }
        return newSettings;
      });
    },
    reset: () => set(defaultSettings),
  };
}

export const settingsStore = createSettingsStore();
