<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { droneStore } from '$lib/stores/drone';
  import { galleryStore } from '$lib/stores/gallery';
  import Button from './ui/button/Button.svelte';
  import { Camera, Video, VideoOff } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { toast } from 'svelte-sonner';
  
  let streaming = false;
  let canvas: HTMLCanvasElement;
  let frameCount = 0;
  let unlistenVideo: UnlistenFn | null = null;
  let packetsPerSecond = 0;
  let lastSecond = 0;
  let decoder: any = null;
  
  onMount(async () => {
    console.log('[VideoFeed] Component mounted');
    
    if (canvas) {
      // Initialize Broadway H.264 decoder
      try {
        const { default: Player } = await import('broadway-player');
        
        decoder = new Player({
          useWorker: true,
          webgl: 'auto',
          size: {
            width: 960,
            height: 720
          },
          canvas: canvas
        });
        
        console.log('[VideoFeed] ✅ Broadway H.264 decoder initialized');
      } catch (error) {
        console.error('[VideoFeed] Failed to initialize decoder:', error);
        toast.error('Video decoder initialization failed');
      }
    }
    
    // Listen for video packets from Rust backend
    unlistenVideo = await listen('video-packet', (event) => {
      if (!streaming || !decoder) return;
      
      try {
        // Decode base64
        const base64Data = event.payload as string;
        const binaryString = atob(base64Data);
        const bytes = new Uint8Array(binaryString.length);
        for (let i = 0; i < binaryString.length; i++) {
          bytes[i] = binaryString.charCodeAt(i);
        }
        
        // Feed to Broadway decoder - it will automatically render to canvas
        decoder.decode(bytes);
        
        frameCount++;
        
        // Calculate packets per second
        const now = Math.floor(Date.now() / 1000);
        if (now !== lastSecond) {
          packetsPerSecond = frameCount;
          frameCount = 0;
          lastSecond = now;
          
          if (packetsPerSecond > 0) {
            console.log('[VideoFeed] Decoding at', packetsPerSecond, 'fps');
          }
        }
      } catch (error) {
        console.error('[VideoFeed] Error decoding packet:', error);
      }
    });
    
    // Auto-start stream when connected
    const unsubscribe = droneStore.subscribe($drone => {
      if ($drone.connected && !streaming) {
        setTimeout(() => {
          console.log('[VideoFeed] Auto-starting stream...');
          startStream();
        }, 1500);
      } else if (!$drone.connected && streaming) {
        stopStream();
      }
    });
    
    return () => unsubscribe();
  });
  
  async function startStream() {
    if (!$droneStore.connected || streaming) return;
    
    if (!decoder) {
      toast.error('Video decoder not ready');
      return;
    }
    
    try {
      console.log('[VideoFeed] Starting video stream...');
      frameCount = 0;
      packetsPerSecond = 0;
      lastSecond = Math.floor(Date.now() / 1000);
      
      await invoke('start_video_stream');
      streaming = true;
      droneStore.setVideoActive(true);
      toast.success('Video stream started - decoding H.264');
      console.log('[VideoFeed] ✅ Video stream started, decoder ready');
      
    } catch (error) {
      console.error('[VideoFeed] ❌ Failed to start stream:', error);
      toast.error('Failed to start stream: ' + error);
    }
  }
  
  async function stopStream() {
    if (!streaming) return;
    
    try {
      console.log('[VideoFeed] Stopping video stream...');
      await invoke('stop_video_stream');
      streaming = false;
      droneStore.setVideoActive(false);
      toast.info('Video stream stopped');
      console.log('[VideoFeed] ✅ Video stream stopped');
    } catch (error) {
      console.error('[VideoFeed] ❌ Failed to stop stream:', error);
    }
  }
  
  async function takePicture() {
    if (!canvas) return;
    
    try {
      console.log('[VideoFeed] Capturing picture...');
      
      const dataUrl = canvas.toDataURL('image/png');
      const timestamp = Date.now();
      const filename = `tello_capture_${timestamp}.png`;
      
      galleryStore.addImage({
        id: `img_${timestamp}`,
        filename,
        path: dataUrl,
        timestamp,
        size: dataUrl.length,
        thumbnail: dataUrl
      });
      
      console.log('[VideoFeed] ✅ Picture captured:', filename);
      toast.success('Picture saved to gallery');
    } catch (error) {
      console.error('[VideoFeed] Capture failed:', error);
      toast.error('Failed to capture picture');
    }
  }
  
  onDestroy(() => {
    console.log('[VideoFeed] Component destroyed');
    streaming = false;
    if (unlistenVideo) {
      unlistenVideo();
    }
    if (decoder) {
      decoder = null;
    }
  });
</script>

<!-- Video feed with H.264 decoding -->
<div class="h-full w-full flex flex-col p-4" style="background-color: #000000">
  <!-- Video controls overlay -->
  <div class="flex items-center justify-between mb-2 px-2">
    <div class="flex items-center gap-2">
      <div 
        class="w-2 h-2 rounded-full" 
        style="background-color: {streaming && packetsPerSecond > 0 ? '#ef4444' : '#6b7280'}; {streaming && packetsPerSecond > 0 ? 'animation: pulse 2s infinite' : ''}"
      ></div>
      <span class="text-sm font-medium" style="color: white">
        {#if streaming && packetsPerSecond > 0}
          LIVE - {packetsPerSecond} FPS
        {:else if streaming}
          LIVE - Waiting for frames...
        {:else if $droneStore.connected}
          Ready
        {:else}
          Disconnected
        {/if}
      </span>
    </div>
    
    <div class="flex gap-2">
      {#if streaming}
        <Button on:click={stopStream} variant="outline" size="sm">
          <VideoOff class="h-3 w-3 mr-1" />
          Stop
        </Button>
      {:else}
        <Button on:click={startStream} size="sm" disabled={!$droneStore.connected}>
          <Video class="h-3 w-3 mr-1" />
          Start
        </Button>
      {/if}
      
      <Button on:click={takePicture} variant="outline" size="sm" disabled={!streaming || packetsPerSecond === 0}>
        <Camera class="h-3 w-3" />
      </Button>
    </div>
  </div>
  
  <!-- Canvas - Broadway will render decoded video here -->
  <div class="flex-1 relative flex items-center justify-center">
    <canvas 
      bind:this={canvas}
      class="max-w-full max-h-full"
      style="display: block; object-fit: contain; background-color: #000000"
    />
    
    {#if !$droneStore.connected}
      <div class="absolute inset-0 flex flex-col items-center justify-center" style="background-color: rgba(0,0,0,0.95)">
        <p class="text-xl font-medium mb-2" style="color: white">Not Connected</p>
        <p class="text-sm" style="color: #9ca3af">Connect to drone to view live video</p>
      </div>
    {:else if streaming && packetsPerSecond === 0}
      <div class="absolute inset-0 flex flex-col items-center justify-center" style="background-color: rgba(0,0,0,0.8)">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-t-transparent mb-4" style="border-color: #3b82f6; border-top-color: transparent"></div>
        <p class="text-lg font-medium" style="color: white">Waiting for video stream...</p>
        <p class="text-sm" style="color: #9ca3af">H.264 decoder ready</p>
      </div>
    {/if}
  </div>
</div>

<style>
  @keyframes pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }
</style>
