export interface RecognitionResult {
  success: boolean;
  name: string | null;
  confidence: number;
  bbox: BoundingBox | null;
}

export interface BoundingBox {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface TrainingProgress {
  stage: 'capturing' | 'processing' | 'training' | 'complete' | 'error';
  progress: number; // 0-100
  message: string;
  samplesCollected?: number;
}

export interface ModelMetadata {
  name: string;
  algorithm: 'lbph' | 'orb' | 'mp';
  sampleCount: number;
  createdAt: string;
  updatedAt: string;
  threshold?: number;
  faceSize?: [number, number];
  label?: number;
}

export interface FaceModel {
  name: string;
  algorithm: 'lbph' | 'orb' | 'mp';
  action: string;
  sampleCount: number;
  createdAt: string;
  updatedAt: string;
}
