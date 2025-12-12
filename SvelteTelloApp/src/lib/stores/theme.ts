import { writable } from 'svelte/store';
import { themes, applyTheme, type ThemeName, type Theme } from '$lib/themes';

function createThemeStore() {
  // Migrate old theme names to new format
  const migrateThemeName = (oldName: string | null): ThemeName => {
    if (!oldName) return 'dark-blue';
    
    // Migration map from old to new
    const migration: Record<string, ThemeName> = {
      'dark': 'dark-blue',
      'light': 'light-blue',
      'fedora': 'dark-blue',
      'ocean': 'dark-teal',
      'sunset': 'light-orange',
      'forest': 'dark-green',
      'galaxy': 'dark-purple',
      'rose': 'light-red',
    };
    
    return migration[oldName] || 'dark-blue';
  };
  
  const savedTheme = typeof window !== 'undefined' 
    ? localStorage.getItem('theme')
    : null;
  
  const themeName = migrateThemeName(savedTheme);
  
  const { subscribe, set, update } = writable<Theme>(themes[themeName]);

  return {
    subscribe,
    setTheme: (themeName: ThemeName) => {
      const theme = themes[themeName];
      set(theme);
      applyTheme(theme);
      if (typeof window !== 'undefined') {
        localStorage.setItem('theme', themeName);
      }
    },
    init: () => {
      const theme = themes[themeName];
      if (theme) {
        applyTheme(theme);
      } else {
        const defaultTheme = themes['dark-blue'];
        applyTheme(defaultTheme);
      }
    }
  };
}

export const themeStore = createThemeStore();
