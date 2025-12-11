<script lang="ts">
  import { droneStore, connectionStore } from '$lib/stores/drone';
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api/tauri';
  import Button from './ui/button/Button.svelte';
  import { RefreshCw, Wifi, AlertCircle } from 'lucide-svelte';
  
  let showReconnect = false;
  let reconnecting = false;
  let lastErrorMessage = '';
  
  onMount(() => {
    console.log('[ReconnectScreen] Component mounted');
    
    connectionStore.subscribe(status => {
      console.log('[ReconnectScreen] Connection status:', status);
      showReconnect = status.status === 'error';
      if (status.status === 'error') {
        lastErrorMessage = status.message;
      }
    });
  });
  
  async function attemptReconnect() {
    console.log('[ReconnectScreen] Attempting to reconnect...');
    reconnecting = true;
    connectionStore.setStatus('connecting', 'Reconnecting...');
    
    try {
      const result: any = await invoke('connect_drone');
      console.log('[ReconnectScreen] Reconnect result:', result);
      
      if (result.success) {
        droneStore.setConnected(true);
        connectionStore.setStatus('connected', 'Reconnected successfully');
        showReconnect = false;
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      console.error('[ReconnectScreen] Reconnect failed:', error);
      connectionStore.setStatus('error', String(error));
    } finally {
      reconnecting = false;
    }
  }
</script>

{#if showReconnect}
  <div 
    class="fixed inset-0 flex items-center justify-center z-50 bg-black/70 backdrop-blur-sm"
    transition:fade={{ duration: 200 }}
  >
    <div class="theme-surface rounded-lg shadow-2xl p-8 max-w-md w-full mx-4 animate-slide-up">
      <div class="text-center space-y-6">
        <div class="relative inline-block">
          <AlertCircle class="h-20 w-20 text-error mx-auto" />
        </div>
        
        <div>
          <h3 class="text-2xl font-bold theme-text mb-2">
            Connection Lost
          </h3>
          <p class="theme-text-muted text-sm">
            {lastErrorMessage || 'Unable to communicate with the drone'}
          </p>
        </div>
        
        <div class="space-y-3 text-sm theme-text-muted text-left p-4 rounded-md" style="background-color: var(--color-surface)">
          <p class="font-semibold theme-text">Troubleshooting:</p>
          <ul class="list-disc list-inside space-y-1">
            <li>Check WiFi connection to TELLO-XXXXXX</li>
            <li>Ensure drone is powered on</li>
            <li>Move closer to the drone</li>
            <li>Restart the drone if needed</li>
          </ul>
        </div>
        
        <Button 
          on:click={attemptReconnect}
          disabled={reconnecting}
          class="w-full theme-primary"
          size="lg"
        >
          {#if reconnecting}
            <RefreshCw class="mr-2 h-5 w-5 animate-spin" />
            Reconnecting...
          {:else}
            <Wifi class="mr-2 h-5 w-5" />
            Reconnect
          {/if}
        </Button>
      </div>
    </div>
  </div>
{/if}

