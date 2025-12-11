import { writable } from 'svelte/store';
import { themes, applyTheme, type ThemeName, type Theme } from '$lib/themes';

function createThemeStore() {
  // Load saved theme from localStorage or default to 'dark'
  const savedTheme = (typeof window !== 'undefined' 
    ? localStorage.getItem('theme') as ThemeName 
    : null) || 'dark';
  
  console.log('[Theme] Loading saved theme:', savedTheme);
  
  const { subscribe, set, update } = writable<Theme>(themes[savedTheme]);

  return {
    subscribe,
    setTheme: (themeName: ThemeName) => {
      console.log('[Theme] Switching to:', themeName);
      const theme = themes[themeName];
      set(theme);
      applyTheme(theme);
      if (typeof window !== 'undefined') {
        localStorage.setItem('theme', themeName);
        console.log('[Theme] Saved theme to localStorage:', themeName);
      }
    },
    init: () => {
      console.log('[Theme] Initializing theme system...');
      const theme = themes[savedTheme];
      applyTheme(theme);
      console.log('[Theme] Applied theme:', savedTheme);
    }
  };
}

export const themeStore = createThemeStore();
