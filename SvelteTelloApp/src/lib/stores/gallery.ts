import { writable } from 'svelte/store';

export interface CapturedImage {
  id: string;
  filename: string;
  path: string;
  timestamp: number;
  size: number; // bytes
  thumbnail?: string; // base64 or path
}

function createGalleryStore() {
  const { subscribe, set, update } = writable<CapturedImage[]>([]);

  return {
    subscribe,
    set,
    addImage: (image: CapturedImage) => {
      console.log('[GalleryStore] Adding image:', image.filename);
      update(images => [image, ...images]);
    },
    removeImage: (id: string) => {
      console.log('[GalleryStore] Removing image:', id);
      update(images => images.filter(img => img.id !== id));
    },
    clear: () => set([]),
  };
}

export const galleryStore = createGalleryStore();

