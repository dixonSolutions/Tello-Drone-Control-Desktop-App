import { writable } from 'svelte/store';

export interface FaceModel {
  name: string;
  algorithm: 'lbph' | 'orb' | 'mp';
  action: 'none' | 'takeoff' | 'land' | 'up' | 'down' | 'left' | 'right' | 'forward' | 'back';
  threshold?: number;
  sampleCount?: number;
  createdAt: string;
  updatedAt: string;
}

export interface RecognitionState {
  active: boolean;
  currentModel: string | null;
  recognizedName: string | null;
  confidence: number;
  bbox: { x: number; y: number; width: number; height: number } | null;
  lastSeen: number;
  following: boolean;
}

function createModelsStore() {
  const { subscribe, set, update } = writable<FaceModel[]>([]);

  return {
    subscribe,
    set,
    addModel: (model: FaceModel) => update(models => [...models, model]),
    updateModel: (name: string, updates: Partial<FaceModel>) => 
      update(models => models.map(m => m.name === name ? { ...m, ...updates } : m)),
    removeModel: (name: string) => update(models => models.filter(m => m.name !== name)),
    refresh: async () => {
      // This will be implemented to call Tauri backend
      // const models = await invoke('list_face_models');
      // set(models);
    },
  };
}

function createRecognitionStore() {
  const { subscribe, set, update } = writable<RecognitionState>({
    active: false,
    currentModel: null,
    recognizedName: null,
    confidence: 0,
    bbox: null,
    lastSeen: 0,
    following: false,
  });

  return {
    subscribe,
    start: (modelName: string) => update(state => ({ 
      ...state, 
      active: true, 
      currentModel: modelName 
    })),
    stop: () => set({
      active: false,
      currentModel: null,
      recognizedName: null,
      confidence: 0,
      bbox: null,
      lastSeen: 0,
      following: false,
    }),
    updateRecognition: (name: string | null, confidence: number, bbox: RecognitionState['bbox']) =>
      update(state => ({
        ...state,
        recognizedName: name,
        confidence,
        bbox,
        lastSeen: name ? Date.now() : state.lastSeen,
      })),
    setFollowing: (following: boolean) => update(state => ({ ...state, following })),
  };
}

export const modelsStore = createModelsStore();
export const recognitionStore = createRecognitionStore();
