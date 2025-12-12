<script lang="ts">
  import { connectionStore } from '$lib/stores/drone';
  import { Loader2, Wifi } from 'lucide-svelte';
  
  let showConnecting = false;
  let message = '';
  
  connectionStore.subscribe(status => {
    showConnecting = status.status === 'connecting';
    message = status.message;
  });
</script>

{#if showConnecting}
  <!-- Overlay only in video section - NOT full screen -->
  <div class="connecting-overlay">
    <div class="connecting-content">
      <div class="icon-container">
        <Wifi class="wifi-icon" />
        <Loader2 class="spinner" />
      </div>
      
      <h3 class="title">Connecting to Drone</h3>
      <p class="message">{message}</p>
      
      <div class="dots">
        <div class="dot" style="animation-delay: 0ms"></div>
        <div class="dot" style="animation-delay: 200ms"></div>
        <div class="dot" style="animation-delay: 400ms"></div>
      </div>
    </div>
  </div>
{/if}

<style>
  .connecting-overlay {
    position: fixed;
    top: 50px; /* Below header */
    left: 0;
    right: 0;
    height: 55%; /* Only video section */
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: rgba(0, 0, 0, 0.92);
    z-index: 40;
    backdrop-filter: blur(8px);
    animation: fadeIn 0.2s ease;
  }
  
  .connecting-content {
    text-align: center;
    padding: 2rem;
    max-width: 400px;
  }
  
  .icon-container {
    position: relative;
    display: inline-block;
    margin-bottom: 1.5rem;
  }
  
  .wifi-icon {
    width: 64px;
    height: 64px;
    color: var(--color-primary);
  }
  
  .spinner {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 80px;
    height: 80px;
    color: var(--color-primary);
    opacity: 0.3;
    animation: spin 2s linear infinite;
  }
  
  .title {
    font-size: 1.5rem;
    font-weight: 700;
    color: white;
    margin-bottom: 0.5rem;
  }
  
  .message {
    color: #9ca3af;
    margin-bottom: 1rem;
  }
  
  .dots {
    display: flex;
    gap: 0.5rem;
    justify-content: center;
  }
  
  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: var(--color-primary);
    animation: pulse 1.5s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
  
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
  
  @keyframes spin {
    from {
      transform: translate(-50%, -50%) rotate(0deg);
    }
    to {
      transform: translate(-50%, -50%) rotate(360deg);
    }
  }
  
  @keyframes pulse {
    0%, 100% {
      opacity: 0.4;
    }
    50% {
      opacity: 1;
    }
  }
</style>
