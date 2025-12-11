<script lang="ts">
  import { themeStore } from '$lib/stores/theme';
  import { themes, type ThemeName, type BaseTheme } from '$lib/themes';
  import Button from './ui/button/Button.svelte';
  import { MoreVertical, Check } from 'lucide-svelte';
  
  let showThemePicker = false;
  let currentTheme: ThemeName = 'dark-blue';
  let currentBase: BaseTheme = 'dark';
  
  themeStore.subscribe((theme) => {
    currentTheme = theme.name;
    currentBase = theme.baseTheme;
  });
  
  function selectTheme(themeName: ThemeName) {
    console.log('[ThemeSelector] User selected theme:', themeName);
    themeStore.setTheme(themeName);
    showThemePicker = false;
  }
  
  function togglePicker() {
    showThemePicker = !showThemePicker;
    console.log('[ThemeSelector] Theme picker toggled:', showThemePicker);
  }
  
  // Group themes by base (dark/light)
  const darkThemes = Object.values(themes).filter(t => t.baseTheme === 'dark');
  const lightThemes = Object.values(themes).filter(t => t.baseTheme === 'light');
</script>

<div class="relative">
  <Button on:click={togglePicker} variant="ghost" size="icon" class="theme-text">
    <MoreVertical class="h-5 w-5" />
    <span class="sr-only">Customize theme</span>
  </Button>
  
  {#if showThemePicker}
    <div class="absolute right-0 mt-2 w-80 rounded-lg theme-surface shadow-2xl border theme-border z-50 animate-slide-up">
      <div class="p-4">
        <div class="flex items-center justify-between mb-4">
          <p class="text-sm font-bold theme-text">Customize Color Theme</p>
          <button 
            on:click={() => (showThemePicker = false)}
            class="theme-text-muted hover:theme-text transition-colors"
          >
            ✕
          </button>
        </div>
        
        <!-- Dark Themes -->
        <div class="mb-4">
          <p class="text-xs font-semibold theme-text-muted mb-2">DARK MODE</p>
          <div class="grid grid-cols-3 gap-2">
            {#each darkThemes as theme}
              <button
                on:click={() => selectTheme(theme.name)}
                class="flex flex-col items-center gap-2 p-3 rounded-md transition-all border"
                style="background-color: {currentTheme === theme.name ? 'var(--color-primary)' : 'transparent'}; border-color: {currentTheme === theme.name ? theme.primary : 'var(--color-border)'}"
              >
                <div 
                  class="w-10 h-10 rounded-full border-2"
                  style="background-color: {theme.primary}; border-color: {theme.primary}"
                />
                <span class="text-xs font-medium" style="color: {currentTheme === theme.name ? 'var(--color-text)' : 'var(--color-text-muted)'}">
                  {theme.accentColor.charAt(0).toUpperCase() + theme.accentColor.slice(1)}
                </span>
                {#if currentTheme === theme.name}
                  <Check class="h-4 w-4 absolute top-1 right-1" style="color: {theme.primary}" />
                {/if}
              </button>
            {/each}
          </div>
        </div>
        
        <!-- Light Themes -->
        <div>
          <p class="text-xs font-semibold theme-text-muted mb-2">LIGHT MODE</p>
          <div class="grid grid-cols-3 gap-2">
            {#each lightThemes as theme}
              <button
                on:click={() => selectTheme(theme.name)}
                class="flex flex-col items-center gap-2 p-3 rounded-md transition-all border relative"
                style="background-color: {currentTheme === theme.name ? 'var(--color-primary)' : 'transparent'}; border-color: {currentTheme === theme.name ? theme.primary : 'var(--color-border)'}"
              >
                <div 
                  class="w-10 h-10 rounded-full border-2"
                  style="background-color: {theme.primary}; border-color: {theme.primary}"
                />
                <span class="text-xs font-medium" style="color: {currentTheme === theme.name ? 'var(--color-text)' : 'var(--color-text-muted)'}">
                  {theme.accentColor.charAt(0).toUpperCase() + theme.accentColor.slice(1)}
                </span>
                {#if currentTheme === theme.name}
                  <Check class="h-4 w-4 absolute top-1 right-1" style="color: {theme.primary}" />
                {/if}
              </button>
            {/each}
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

{#if showThemePicker}
  <button
    class="fixed inset-0 z-40"
    on:click={() => (showThemePicker = false)}
    aria-label="Close theme picker"
  />
{/if}
