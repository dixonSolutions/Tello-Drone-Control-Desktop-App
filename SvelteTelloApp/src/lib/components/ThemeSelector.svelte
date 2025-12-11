<script lang="ts">
  import { themeStore } from '$lib/stores/theme';
  import { themes, type ThemeName } from '$lib/themes';
  import Button from './ui/button/Button.svelte';
  import { MoreVertical, Check } from 'lucide-svelte';
  
  let showThemePicker = false;
  let currentTheme: ThemeName = 'dark';
  
  themeStore.subscribe((theme) => {
    currentTheme = theme.name;
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
</script>

<div class="relative">
  <Button on:click={togglePicker} variant="ghost" size="icon" class="theme-text">
    <MoreVertical class="h-5 w-5" />
    <span class="sr-only">Customize theme</span>
  </Button>
  
  {#if showThemePicker}
    <div class="absolute right-0 mt-2 w-72 rounded-lg theme-surface shadow-2xl border theme-border z-50 animate-slide-up">
      <div class="p-4">
        <div class="flex items-center justify-between mb-4">
          <p class="text-sm font-bold theme-text">Customize Color Theme</p>
          <button 
            on:click={() => (showThemePicker = false)}
            class="theme-text-muted hover:theme-text"
          >
            ✕
          </button>
        </div>
        
        <div class="space-y-2">
          {#each Object.values(themes) as theme}
            <button
              on:click={() => selectTheme(theme.name)}
              class="w-full flex items-center gap-3 px-3 py-3 rounded-md transition-colors theme-surface-hover"
              style="background-color: {currentTheme === theme.name ? 'var(--color-primary)' : 'transparent'}"
            >
              <div 
                class="w-8 h-8 rounded-full border-2 theme-border flex-shrink-0"
                style="background-color: {theme.primary}"
              />
              <span class="flex-1 text-left text-sm font-medium" style="color: {currentTheme === theme.name ? 'white' : 'var(--color-text)'}">
                {theme.displayName}
              </span>
              {#if currentTheme === theme.name}
                <Check class="h-5 w-5" style="color: white" />
              {/if}
            </button>
          {/each}
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
