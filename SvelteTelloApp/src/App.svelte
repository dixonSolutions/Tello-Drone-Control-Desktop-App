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
  import SkeletonLoader from './lib/components/SkeletonLoader.svelte';
  
  let activeTab: 'controls' | 'modes' | 'tricks' | 'models' | 'settings' = 'controls';
  let appReady = false;
  let componentsLoaded = false;
  
  onMount(() => {
    console.log('='.repeat(60));
    console.log('🚁 TELLO DRONE CONTROL APP STARTING');
    console.log('='.repeat(60));
    console.log('[App] Mounting main application...');
    console.log('[App] Current timestamp:', new Date().toISOString());
    
    themeStore.init();
    console.log('[App] Theme system initialized');
    
    // Simulate component loading
    setTimeout(() => {
      componentsLoaded = true;
      console.log('[App] Components loaded');
    }, 500);
    
    setTimeout(() => {
      appReady = true;
      console.log('[App] Application ready!');
      console.log('='.repeat(60));
    }, 1000);
    
    // Log drone state changes
    const unsubscribe = droneStore.subscribe(state => {
      console.log('[App] Drone state updated:', {
        connected: state.connected,
        flying: state.flying,
        battery: state.battery,
        height: state.height
      });
    });
    
    return unsubscribe;
  });
  
  function changeTab(tab: typeof activeTab) {
    console.log('[App] Switching to tab:', tab);
    activeTab = tab;
  }
</script>

<!-- Overlays and Notifications -->
<LoadingScreen />
<ConnectingOverlay />
<ReconnectScreen />
<Toaster position="top-right" duration={3000} />

{#if appReady}
  <main class="min-h-screen flex flex-col" style="background-color: var(--color-background)" transition:fade={{ duration: 300 }}>
    <!-- Header -->
    <header class="border-b theme-border" style="background-color: var(--color-surface)">
      <div class="container flex h-16 items-center justify-between py-4 px-6">
        <div class="flex items-center gap-4">
          <h1 class="text-2xl font-bold theme-text">Tello Drone Control</h1>
          <ConnectionStatus />
        </div>
        <div class="flex items-center gap-3">
          <Telemetry />
          <div class="h-8 w-px theme-border" style="background-color: var(--color-border)"></div>
          <ThemeSelector />
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <div class="flex-1 container py-6 px-6">
      <!-- Tabs Navigation -->
      <div class="mb-6 flex gap-2 theme-surface rounded-lg p-2 shadow-sm border theme-border">
        <button
          class="px-6 py-2 rounded-md font-semibold transition-all"
          style="background-color: {activeTab === 'controls' ? 'var(--color-primary)' : 'transparent'}; color: {activeTab === 'controls' ? 'white' : 'var(--color-text-muted)'}"
          on:click={() => changeTab('controls')}
        >
          Controls
        </button>
        <button
          class="px-6 py-2 rounded-md font-semibold transition-all"
          style="background-color: {activeTab === 'modes' ? 'var(--color-primary)' : 'transparent'}; color: {activeTab === 'modes' ? 'white' : 'var(--color-text-muted)'}"
          on:click={() => changeTab('modes')}
        >
          Modes
        </button>
        <button
          class="px-6 py-2 rounded-md font-semibold transition-all"
          style="background-color: {activeTab === 'tricks' ? 'var(--color-primary)' : 'transparent'}; color: {activeTab === 'tricks' ? 'white' : 'var(--color-text-muted)'}"
          on:click={() => changeTab('tricks')}
        >
          Tricks
        </button>
        <button
          class="px-6 py-2 rounded-md font-semibold transition-all"
          style="background-color: {activeTab === 'models' ? 'var(--color-primary)' : 'transparent'}; color: {activeTab === 'models' ? 'white' : 'var(--color-text-muted)'}"
          on:click={() => changeTab('models')}
        >
          Models
        </button>
        <button
          class="px-6 py-2 rounded-md font-semibold transition-all"
          style="background-color: {activeTab === 'settings' ? 'var(--color-primary)' : 'transparent'}; color: {activeTab === 'settings' ? 'white' : 'var(--color-text-muted)'}"
          on:click={() => changeTab('settings')}
        >
          Settings
        </button>
      </div>

      <!-- Content Grid -->
      <div class="grid gap-6 lg:grid-cols-2">
        <!-- Left Panel - Controls -->
        <div class="space-y-6">
          {#if componentsLoaded}
            {#if activeTab === 'controls'}
              <DroneControl />
              <Recording />
            {:else if activeTab === 'modes'}
              <FreeFly />
            {:else if activeTab === 'tricks'}
              <Tricks />
            {:else if activeTab === 'models'}
              <FaceRecognition />
            {:else if activeTab === 'settings'}
              <Settings />
            {/if}
          {:else}
            <div class="theme-surface rounded-lg p-6">
              <SkeletonLoader lines={5} height="md" />
            </div>
          {/if}
        </div>
        
        <!-- Right Panel - Additional Controls -->
        <div class="space-y-6">
          {#if componentsLoaded}
            {#if activeTab === 'controls'}
              <Recording />
            {/if}
          {:else}
            <div class="theme-surface rounded-lg p-6">
              <SkeletonLoader lines={3} height="sm" />
            </div>
          {/if}
        </div>
      </div>
    </div>

    <!-- Video Feed at Bottom Middle -->
    <div class="border-t theme-border py-4" style="background-color: var(--color-surface)">
      <div class="container px-6">
        <div class="max-w-4xl mx-auto">
          {#if componentsLoaded}
            <VideoFeed />
          {:else}
            <div class="aspect-video theme-surface rounded-lg flex items-center justify-center">
              <SkeletonLoader lines={1} height="lg" />
            </div>
          {/if}
        </div>
      </div>
    </div>
  </main>
{:else}
  <!-- Fallback while loading -->
  <div class="min-h-screen flex items-center justify-center" style="background-color: var(--color-background)">
    <div class="text-center space-y-4">
      <h1 class="text-3xl font-bold theme-text">Loading Tello Drone Control...</h1>
      <SkeletonLoader lines={3} height="sm" />
    </div>
  </div>
{/if}

<style>
  :global(body) {
    margin: 0;
    padding: 0;
  }
</style>
