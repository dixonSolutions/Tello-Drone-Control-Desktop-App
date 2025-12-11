// Simple theme system: Dark or Light base with color accent palettes
export type BaseTheme = 'dark' | 'light';
export type AccentColor = 'blue' | 'teal' | 'green' | 'purple' | 'orange' | 'red';
export type ThemeName = `${BaseTheme}-${AccentColor}`;

export interface Theme {
  name: ThemeName;
  displayName: string;
  baseTheme: BaseTheme;
  accentColor: AccentColor;
  primary: string;
  primaryHover: string;
  primaryActive: string;
  accent: string;
  background: string;
  surface: string;
  surfaceHover: string;
  text: string;
  textMuted: string;
  border: string;
  shadow: string;
  success: string;
  warning: string;
  error: string;
  info: string;
}

const accentColors = {
  blue: { primary: '#3b82f6', hover: '#2563eb', active: '#1d4ed8' },
  teal: { primary: '#14b8a6', hover: '#0d9488', active: '#0f766e' },
  green: { primary: '#22c55e', hover: '#16a34a', active: '#15803d' },
  purple: { primary: '#a855f7', hover: '#9333ea', active: '#7e22ce' },
  orange: { primary: '#f97316', hover: '#ea580c', active: '#c2410c' },
  red: { primary: '#ef4444', hover: '#dc2626', active: '#b91c1c' },
};

function createTheme(base: BaseTheme, accent: AccentColor): Theme {
  const isDark = base === 'dark';
  const accentDef = accentColors[accent];
  
  return {
    name: `${base}-${accent}` as ThemeName,
    displayName: `${base === 'dark' ? 'Dark' : 'Light'} ${accent.charAt(0).toUpperCase() + accent.slice(1)}`,
    baseTheme: base,
    accentColor: accent,
    primary: accentDef.primary,
    primaryHover: accentDef.hover,
    primaryActive: accentDef.active,
    accent: accentDef.primary,
    background: isDark ? '#0f172a' : '#ffffff',
    surface: isDark ? '#1e293b' : '#f8fafc',
    surfaceHover: isDark ? '#334155' : '#f1f5f9',
    text: isDark ? '#f1f5f9' : '#0f172a',
    textMuted: isDark ? '#94a3b8' : '#64748b',
    border: isDark ? '#334155' : '#e2e8f0',
    shadow: isDark ? 'rgba(0, 0, 0, 0.5)' : 'rgba(0, 0, 0, 0.1)',
    success: '#10b981',
    warning: '#f59e0b',
    error: '#ef4444',
    info: accentDef.primary,
  };
}

export const themes: Record<ThemeName, Theme> = {
  'dark-blue': createTheme('dark', 'blue'),
  'dark-teal': createTheme('dark', 'teal'),
  'dark-green': createTheme('dark', 'green'),
  'dark-purple': createTheme('dark', 'purple'),
  'dark-orange': createTheme('dark', 'orange'),
  'dark-red': createTheme('dark', 'red'),
  'light-blue': createTheme('light', 'blue'),
  'light-teal': createTheme('light', 'teal'),
  'light-green': createTheme('light', 'green'),
  'light-purple': createTheme('light', 'purple'),
  'light-orange': createTheme('light', 'orange'),
  'light-red': createTheme('light', 'red'),
};

export function applyTheme(theme: Theme) {
  const root = document.documentElement;
  
  // Apply CSS custom properties
  root.style.setProperty('--color-primary', theme.primary);
  root.style.setProperty('--color-primary-hover', theme.primaryHover);
  root.style.setProperty('--color-primary-active', theme.primaryActive);
  root.style.setProperty('--color-accent', theme.accent);
  root.style.setProperty('--color-background', theme.background);
  root.style.setProperty('--color-surface', theme.surface);
  root.style.setProperty('--color-surface-hover', theme.surfaceHover);
  root.style.setProperty('--color-text', theme.text);
  root.style.setProperty('--color-text-muted', theme.textMuted);
  root.style.setProperty('--color-border', theme.border);
  root.style.setProperty('--color-shadow', theme.shadow);
  root.style.setProperty('--color-success', theme.success);
  root.style.setProperty('--color-warning', theme.warning);
  root.style.setProperty('--color-error', theme.error);
  root.style.setProperty('--color-info', theme.info);
}
