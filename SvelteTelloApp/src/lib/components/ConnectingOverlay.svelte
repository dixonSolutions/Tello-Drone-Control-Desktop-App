<script lang="ts">
  import { connectionStore } from '$lib/stores/drone';
  import { fade } from 'svelte/transition';
  import { Loader2 } from 'lucide-svelte';
  
  let showConnecting = false;
  
  connectionStore.subscribe(status => {
    showConnecting = status.status === 'connecting';
  });
</script>

{#if showConnecting}
  <div 
    class="fixed inset-0 flex items-center justify-center z-50 bg-black/50 backdrop-blur-sm"
    transition:fade={{ duration: 200 }}
  >
    <div class="theme-surface rounded-lg shadow-2xl p-8 max-w-md w-full mx-4 animate-slide-up">
      <div class="text-center space-y-4">
        <div class="relative inline-block">
          <Loader2 class="h-16 w-16 theme-text animate-spin" />
        </div>
        
        <h3 class="text-2xl font-bold theme-text">
          Connecting to Drone
        </h3>
        
        <p class="theme-text-muted">
          Establishing connection to Tello drone...
        </p>
        
        <div class="flex gap-2 justify-center pt-2">
          <div class="h-2 w-2 rounded-full bg-primary animate-pulse" style="animation-delay: 0ms"></div>
          <div class="h-2 w-2 rounded-full bg-primary animate-pulse" style="animation-delay: 200ms"></div>
          <div class="h-2 w-2 rounded-full bg-primary animate-pulse" style="animation-delay: 400ms"></div>
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

