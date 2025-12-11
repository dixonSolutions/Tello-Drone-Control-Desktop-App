<script lang="ts">
  import { connectionStore } from '$lib/stores/drone';
  import { fade } from 'svelte/transition';
  import { Loader2 } from 'lucide-svelte';
  
  let showConnecting = false;
  
  connectionStore.subscribe(status => {
    showConnecting = status.status === 'connecting';
    if (status.status === 'connecting') {
      console.log('[ConnectingOverlay] Showing overlay:', status.message);
    }
  });
</script>

{#if showConnecting}
  <div 
    class="fixed inset-0 flex items-center justify-center z-50 backdrop-blur-sm"
    style="background-color: rgba(0, 0, 0, 0.5)"
    transition:fade={{ duration: 200 }}
  >
    <div class="rounded-lg shadow-2xl p-8 max-w-md w-full mx-4 animate-slide-up" style="background-color: var(--color-surface)">
      <div class="text-center space-y-4">
        <div class="relative inline-block">
          <Loader2 class="h-16 w-16 animate-spin" style="color: var(--color-primary)" />
        </div>
        
        <h3 class="text-2xl font-bold" style="color: var(--color-text)">
          Connecting to Drone
        </h3>
        
        <p style="color: var(--color-text-muted)">
          Establishing connection to Tello drone...
        </p>
        
        <div class="flex gap-2 justify-center pt-2">
          <div class="h-2 w-2 rounded-full animate-pulse" style="background-color: var(--color-primary); animation-delay: 0ms"></div>
          <div class="h-2 w-2 rounded-full animate-pulse" style="background-color: var(--color-primary); animation-delay: 200ms"></div>
          <div class="h-2 w-2 rounded-full animate-pulse" style="background-color: var(--color-primary); animation-delay: 400ms"></div>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  @keyframes pulse {
    0%, 100% {
      opacity: 0.4;
    }
    50% {
      opacity: 1;
    }
  }
  
  .animate-pulse {
    animation: pulse 1.5s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
</style>

