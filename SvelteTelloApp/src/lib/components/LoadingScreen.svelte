<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { Loader2, Plane } from 'lucide-svelte';
  
  let loading = true;
  let loadingMessage = 'Initializing...';
  
  onMount(async () => {
    loadingMessage = 'Loading theme system...';
    await new Promise(r => setTimeout(r, 300));
    
    loadingMessage = 'Setting up drone interface...';
    await new Promise(r => setTimeout(r, 300));
    
    loadingMessage = 'Ready!';
    await new Promise(r => setTimeout(r, 200));
    
    loading = false;
  });
</script>

{#if loading}
  <div 
    class="fixed inset-0 flex flex-col items-center justify-center z-[100]"
    style="background-color: var(--color-background)"
    transition:fade={{ duration: 300 }}
  >
    <div class="text-center space-y-6">
      <!-- Logo/Icon -->
      <div class="relative">
        <Plane class="h-24 w-24 mx-auto animate-bounce" style="color: var(--color-primary)" />
        <div class="absolute inset-0 flex items-center justify-center">
          <Loader2 class="h-32 w-32 animate-spin opacity-30" style="color: var(--color-primary)" />
        </div>
      </div>
      
      <!-- App Name -->
      <h1 class="text-4xl font-bold" style="color: var(--color-text)">
        Tello Drone Control
      </h1>
      
      <!-- Loading Message -->
      <div class="flex items-center gap-3 justify-center">
        <Loader2 class="h-5 w-5 animate-spin" style="color: var(--color-primary)" />
        <p class="text-xl" style="color: var(--color-text-muted)">{loadingMessage}</p>
      </div>
      
      <!-- Progress dots -->
      <div class="flex gap-2 justify-center">
        <div class="h-2 w-2 rounded-full animate-pulse" style="background-color: var(--color-primary); animation-delay: 0ms"></div>
        <div class="h-2 w-2 rounded-full animate-pulse" style="background-color: var(--color-primary); animation-delay: 200ms"></div>
        <div class="h-2 w-2 rounded-full animate-pulse" style="background-color: var(--color-primary); animation-delay: 400ms"></div>
      </div>
    </div>
  </div>
{/if}

<style>
  @keyframes pulse {
    0%, 100% {
      opacity: 0.6;
    }
    50% {
      opacity: 1;
    }
  }
  
  .animate-pulse {
    animation: pulse 1.5s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
</style>

