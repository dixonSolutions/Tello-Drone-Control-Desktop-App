// Theme definitions with multiple color schemes
export type ThemeName = 'fedora' | 'ocean' | 'sunset' | 'forest' | 'galaxy' | 'rose' | 'dark' | 'light';

export interface Theme {
  name: ThemeName;
  displayName: string;
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

export const themes: Record<ThemeName, Theme> = {
  fedora: {
    name: 'fedora',
    displayName: 'Fedora Blue',
    primary: '#3c6eb4',
    primaryHover: '#2f5d99',
    primaryActive: '#264c7d',
    accent: '#3B82F6',
    background: '#f5f7fa',
    surface: '#ffffff',
    surfaceHover: '#f8fafc',
    text: '#0f1b2a',
    textMuted: '#64748b',
    border: '#e2e8f0',
    shadow: 'rgba(0, 0, 0, 0.1)',
    success: '#10b981',
    warning: '#f59e0b',
    error: '#ef4444',
    info: '#3b82f6',
  },
  ocean: {
    name: 'ocean',
    displayName: 'Ocean Teal',
    primary: '#0891b2',
    primaryHover: '#0e7490',
    primaryActive: '#155e75',
    accent: '#06b6d4',
    background: '#ecfeff',
    surface: '#ffffff',
    surfaceHover: '#f0fdff',
    text: '#0c4a6e',
    textMuted: '#64748b',
    border: '#cffafe',
    shadow: 'rgba(6, 182, 212, 0.1)',
    success: '#14b8a6',
    warning: '#f59e0b',
    error: '#ef4444',
    info: '#0ea5e9',
  },
  sunset: {
    name: 'sunset',
    displayName: 'Sunset Orange',
    primary: '#ea580c',
    primaryHover: '#c2410c',
    primaryActive: '#9a3412',
    accent: '#f97316',
    background: '#fff7ed',
    surface: '#ffffff',
    surfaceHover: '#fffbf5',
    text: '#7c2d12',
    textMuted: '#78716c',
    border: '#fed7aa',
    shadow: 'rgba(249, 115, 22, 0.1)',
    success: '#10b981',
    warning: '#fbbf24',
    error: '#dc2626',
    info: '#3b82f6',
  },
  forest: {
    name: 'forest',
    displayName: 'Forest Green',
    primary: '#16a34a',
    primaryHover: '#15803d',
    primaryActive: '#166534',
    accent: '#22c55e',
    background: '#f0fdf4',
    surface: '#ffffff',
    surfaceHover: '#f7fef9',
    text: '#14532d',
    textMuted: '#64748b',
    border: '#bbf7d0',
    shadow: 'rgba(34, 197, 94, 0.1)',
    success: '#22c55e',
    warning: '#f59e0b',
    error: '#ef4444',
    info: '#3b82f6',
  },
  galaxy: {
    name: 'galaxy',
    displayName: 'Purple Galaxy',
    primary: '#7c3aed',
    primaryHover: '#6d28d9',
    primaryActive: '#5b21b6',
    accent: '#8b5cf6',
    background: '#faf5ff',
    surface: '#ffffff',
    surfaceHover: '#fdf9ff',
    text: '#4c1d95',
    textMuted: '#64748b',
    border: '#e9d5ff',
    shadow: 'rgba(139, 92, 246, 0.1)',
    success: '#10b981',
    warning: '#f59e0b',
    error: '#ef4444',
    info: '#8b5cf6',
  },
  rose: {
    name: 'rose',
    displayName: 'Rose Pink',
    primary: '#e11d48',
    primaryHover: '#be123c',
    primaryActive: '#9f1239',
    accent: '#f43f5e',
    background: '#fff1f2',
    surface: '#ffffff',
    surfaceHover: '#fff5f7',
    text: '#881337',
    textMuted: '#64748b',
    border: '#fecdd3',
    shadow: 'rgba(244, 63, 94, 0.1)',
    success: '#10b981',
    warning: '#f59e0b',
    error: '#dc2626',
    info: '#3b82f6',
  },
  dark: {
    name: 'dark',
    displayName: 'Dark',
    primary: '#3b82f6',
    primaryHover: '#2563eb',
    primaryActive: '#1d4ed8',
    accent: '#60a5fa',
    background: '#0f172a',
    surface: '#1e293b',
    surfaceHover: '#334155',
    text: '#f1f5f9',
    textMuted: '#94a3b8',
    border: '#334155',
    shadow: 'rgba(0, 0, 0, 0.5)',
    success: '#10b981',
    warning: '#f59e0b',
    error: '#ef4444',
    info: '#3b82f6',
  },
  light: {
    name: 'light',
    displayName: 'Light',
    primary: '#2563eb',
    primaryHover: '#1d4ed8',
    primaryActive: '#1e40af',
    accent: '#3b82f6',
    background: '#ffffff',
    surface: '#f8fafc',
    surfaceHover: '#f1f5f9',
    text: '#0f172a',
    textMuted: '#64748b',
    border: '#e2e8f0',
    shadow: 'rgba(0, 0, 0, 0.1)',
    success: '#10b981',
    warning: '#f59e0b',
    error: '#ef4444',
    info: '#3b82f6',
  },
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
