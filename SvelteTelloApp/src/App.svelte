<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { Toaster } from 'svelte-sonner';
  import { themeStore } from './lib/stores/theme';
  import { droneStore } from './lib/stores/drone';
  import LoadingScreen from './lib/components/LoadingScreen.svelte';
  import ConnectingOverlay from './lib/components/ConnectingOverlay.svelte';
  import ReconnectScreen from './lib/components/ReconnectScreen.svelte';
  import ThemeSelector from './lib/components/ThemeSelector.svelte';
  import DroneControl from './lib/components/DroneControl.svelte';
  import VideoFeed from './lib/components/VideoFeed.svelte';
  import Telemetry from './lib/components/Telemetry.svelte';
  import FaceRecognition from './lib/components/FaceRecognition.svelte';
  import FreeFly from './lib/components/FreeFly.svelte';
  import Tricks from './lib/components/Tricks.svelte';
  import Settings from './lib/components/Settings.svelte';
  import Recording from './lib/components/Recording.svelte';
  import ConnectionStatus from './lib/components/ConnectionStatus.svelte';
  import Gallery from './lib/components/Gallery.svelte';
  
  let activeTab: 'controls' | 'modes' | 'tricks' | 'models' | 'gallery' | 'settings' = 'controls';
  let appReady = false;
  
  onMount(() => {
    themeStore.init();
    
    setTimeout(() => {
      appReady = true;
    }, 1000);
  });
  
  function changeTab(tab: typeof activeTab) {
    activeTab = tab;
  }
</script>

<!-- Overlays and Notifications -->
<LoadingScreen />
<ConnectingOverlay />
<ReconnectScreen />
<Toaster position="bottom-right" duration={3000} />

{#if appReady}
  <!-- NO SCROLL LAYOUT - Everything fits on screen like Python app -->
  <main class="h-screen flex flex-col overflow-hidden" style="background-color: var(--color-background)" transition:fade={{ duration: 300 }}>
    
    <!-- Compact Header - Fixed height -->
    <header class="border-b flex-shrink-0" style="border-color: var(--color-border); background-color: var(--color-surface); height: 50px">
      <div class="h-full px-4 flex items-center justify-between">
        <div class="flex items-center gap-3">
          <h1 class="text-base font-bold" style="color: var(--color-text)">Tello Control</h1>
          <ConnectionStatus />
        </div>
        <div class="flex items-center gap-3">
          <Telemetry />
          <ThemeSelector />
        </div>
      </div>
    </header>

    <div class="flex-1 flex flex-col min-h-0">
      <!-- Video Feed - Fixed 55% of remaining height -->
      <div class="flex-shrink-0" style="height: 55%">
        <VideoFeed />
      </div>

      <!-- Controls Area - Remaining 45% -->
      <div class="flex-shrink-0 border-t flex flex-col" style="height: 45%; border-color: var(--color-border); background-color: var(--color-surface)">
        <!-- Tabs - Compact -->
        <div class="flex gap-1 px-3 py-1.5 border-b flex-shrink-0" style="border-color: var(--color-border)">
          <button
            class="px-3 py-1 rounded text-xs font-semibold transition-colors whitespace-nowrap"
            style="background-color: {activeTab === 'controls' ? 'var(--color-primary)' : 'transparent'}; color: {activeTab === 'controls' ? 'var(--color-text)' : 'var(--color-text-muted)'}"
            on:click={() => changeTab('controls')}
          >
            Controls
          </button>
          <button
            class="px-3 py-1 rounded text-xs font-semibold transition-colors whitespace-nowrap"
            style="background-color: {activeTab === 'modes' ? 'var(--color-primary)' : 'transparent'}; color: {activeTab === 'modes' ? 'var(--color-text)' : 'var(--color-text-muted)'}"
            on:click={() => changeTab('modes')}
          >
            Modes
          </button>
          <button
            class="px-3 py-1 rounded text-xs font-semibold transition-colors whitespace-nowrap"
            style="background-color: {activeTab === 'tricks' ? 'var(--color-primary)' : 'transparent'}; color: {activeTab === 'tricks' ? 'var(--color-text)' : 'var(--color-text-muted)'}"
            on:click={() => changeTab('tricks')}
          >
            Tricks
          </button>
          <button
            class="px-3 py-1 rounded text-xs font-semibold transition-colors whitespace-nowrap"
            style="background-color: {activeTab === 'models' ? 'var(--color-primary)' : 'transparent'}; color: {activeTab === 'models' ? 'var(--color-text)' : 'var(--color-text-muted)'}"
            on:click={() => changeTab('models')}
          >
            Models
          </button>
          <button
            class="px-3 py-1 rounded text-xs font-semibold transition-colors whitespace-nowrap"
            style="background-color: {activeTab === 'gallery' ? 'var(--color-primary)' : 'transparent'}; color: {activeTab === 'gallery' ? 'var(--color-text)' : 'var(--color-text-muted)'}"
            on:click={() => changeTab('gallery')}
          >
            Gallery
          </button>
          <button
            class="px-3 py-1 rounded text-xs font-semibold transition-colors whitespace-nowrap"
            style="background-color: {activeTab === 'settings' ? 'var(--color-primary)' : 'transparent'}; color: {activeTab === 'settings' ? 'var(--color-text)' : 'var(--color-text-muted)'}"
            on:click={() => changeTab('settings')}
          >
            Settings
          </button>
        </div>

        <!-- Tab Content - Scrollable if needed -->
        <div class="flex-1 overflow-y-auto px-3 py-2">
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2">
            {#if activeTab === 'controls'}
              <DroneControl />
              <Recording />
            {:else if activeTab === 'modes'}
              <FreeFly />
            {:else if activeTab === 'tricks'}
              <Tricks />
            {:else if activeTab === 'models'}
              <FaceRecognition />
            {:else if activeTab === 'gallery'}
              <Gallery />
            {:else if activeTab === 'settings'}
              <Settings />
            {/if}
          </div>
        </div>
      </div>
    </div>
  </main>
{/if}

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
  }
</style>
