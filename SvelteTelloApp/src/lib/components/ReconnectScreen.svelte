<script lang="ts">
  import { droneStore, connectionStore } from '$lib/stores/drone';
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import Button from './ui/button/Button.svelte';
  import { RefreshCw, WifiOff, Wifi } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  
  let showPopup = false;
  let connecting = false;
  
  onMount(() => {
    // Show popup whenever not connected
    droneStore.subscribe($drone => {
      showPopup = !$drone.connected;
    });
  });
  
  async function attemptConnect() {
    connecting = true;
    connectionStore.setStatus('connecting', 'Connecting...');
    
    try {
      const result: any = await invoke('connect_drone');
      
      if (result.success) {
        droneStore.setConnected(true);
        connectionStore.setStatus('connected', 'Connected successfully');
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      console.error('[ReconnectScreen] Connect failed:', error);
      connectionStore.setStatus('error', String(error));
    } finally {
      connecting = false;
    }
  }
</script>

{#if showPopup}
  <!-- Non-closeable overlay -->
  <div 
    class="fixed inset-0 flex items-center justify-center z-50 backdrop-blur-md"
    style="background-color: rgba(0, 0, 0, 0.85)"
    transition:fade={{ duration: 200 }}
  >
    <div class="rounded-lg shadow-2xl p-10 max-w-lg w-full mx-4" style="background-color: var(--color-surface)">
      <div class="text-center space-y-6">
        <div class="relative inline-block">
          <WifiOff class="h-32 w-32 mx-auto" style="color: var(--color-error)" />
        </div>
        
        <div>
          <h2 class="text-4xl font-bold mb-3" style="color: var(--color-text)">
            Not Connected
          </h2>
          <p class="text-lg" style="color: var(--color-text-muted)">
            Connect to your Tello drone to get started
          </p>
        </div>
        
        <div class="text-sm text-left p-4 rounded-lg space-y-2" style="background-color: var(--color-background)">
          <p class="font-semibold mb-2" style="color: var(--color-text)">Before connecting:</p>
          <ul class="list-disc list-inside space-y-1" style="color: var(--color-text-muted)">
            <li>Turn on your Tello drone</li>
            <li>Connect computer to TELLO-XXXXXX WiFi network</li>
            <li>Wait for WiFi connection to establish</li>
          </ul>
        </div>
        
        <Button 
          on:click={attemptConnect}
          disabled={connecting}
          class="w-full"
          size="lg"
          style="background-color: var(--color-primary); color: var(--color-text); font-size: 1.125rem; padding: 1.5rem"
        >
          {#if connecting}
            <RefreshCw class="mr-2 h-6 w-6 animate-spin" />
            Connecting...
          {:else}
            <Wifi class="mr-2 h-6 w-6" />
            Connect to Drone
          {/if}
        </Button>
      </div>
    </div>
  </div>
{/if}

<style>
  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
  
  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>


