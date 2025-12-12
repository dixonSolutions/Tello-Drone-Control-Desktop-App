<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { droneStore } from '$lib/stores/drone';
  import { galleryStore } from '$lib/stores/gallery';
  import Button from './ui/button/Button.svelte';
  import { Camera, Video, VideoOff, Loader2 } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { toast } from 'svelte-sonner';
  
  let streaming = false;
  let canvas: HTMLCanvasElement;
  let frameCount = 0;
  let unlistenVideo: UnlistenFn | null = null;
  let fps = 0;
  let lastSecond = 0;
  let ctx: CanvasRenderingContext2D | null = null;
  let startingStream = false;
  let decoder: VideoDecoder | null = null;
  
  onMount(() => {
    console.log('[VideoFeed] 📺 Initializing WebCodecs H.264 decoder...');
    
    if (canvas) {
      ctx = canvas.getContext('2d');
      canvas.width = 960;
      canvas.height = 720;
    }
    
    // Check WebCodecs support
    if (typeof VideoDecoder === 'undefined') {
      console.error('[VideoFeed] ❌ WebCodecs not supported!');
      toast.error('WebCodecs API not supported in this browser');
      return;
    }
    
    console.log('[VideoFeed] ✅ WebCodecs supported');
    
    // Initialize H.264 decoder
    try {
      decoder = new VideoDecoder({
        output: (frame) => {
          if (ctx && canvas) {
            // Draw decoded frame to canvas
            canvas.width = frame.displayWidth;
            canvas.height = frame.displayHeight;
            ctx.drawImage(frame, 0, 0);
            frame.close();
            
            frameCount++;
            const now = Math.floor(Date.now() / 1000);
            if (now !== lastSecond) {
              fps = frameCount;
              frameCount = 0;
              lastSecond = now;
            }
          }
        },
        error: (e) => {
          console.error('[VideoFeed] ❌ Decoder error:', e);
        }
      });
      
      decoder.configure({
        codec: 'avc1.42E01E', // H.264 Baseline Profile Level 3.0
        optimizeForLatency: true
      });
      
      console.log('[VideoFeed] ✅ H.264 decoder configured');
    } catch (error) {
      console.error('[VideoFeed] ❌ Failed to create decoder:', error);
      toast.error('Failed to initialize video decoder');
      return;
    }
    
    // Listen for H.264 packets
    console.log('[VideoFeed] 📡 Setting up packet listener...');
    listen('video-packet', (event) => {
      if (!streaming || !decoder) return;
      
      try {
        const base64Data = event.payload as string;
        const binaryString = atob(base64Data);
        const bytes = new Uint8Array(binaryString.length);
        for (let i = 0; i < binaryString.length; i++) {
          bytes[i] = binaryString.charCodeAt(i);
        }
        
        // Feed to decoder as an EncodedVideoChunk
        const chunk = new EncodedVideoChunk({
          type: 'key', // Tello sends keyframes
          timestamp: performance.now() * 1000,
          data: bytes
        });
        
        decoder.decode(chunk);
      } catch (error) {
        console.error('[VideoFeed] ❌ Decode error:', error);
      }
    }).then(unlisten => {
      unlistenVideo = unlisten;
      console.log('[VideoFeed] ✅ Packet listener ready');
    });
    
    // Auto-start stream when connected
    const unsubscribe = droneStore.subscribe($drone => {
      if ($drone.connected && !streaming && !startingStream) {
        setTimeout(() => startStream(), 1000);
      } else if (!$drone.connected && streaming) {
        stopStream();
      }
    });
    
    return () => {
      unsubscribe();
      if (unlistenVideo) unlistenVideo();
      if (decoder) decoder.close();
    };
  });
  
  async function startStream() {
    if (!$droneStore.connected || streaming || startingStream) return;
    
    console.log('[VideoFeed] 🎥 Starting stream...');
    startingStream = true;
    
    try {
      frameCount = 0;
      fps = 0;
      lastSecond = Math.floor(Date.now() / 1000);
      
      const result: any = await invoke('start_video_stream');
      console.log('[VideoFeed] ✅ Stream started:', result);
      
      streaming = true;
      droneStore.setVideoActive(true);
      toast.success('Video stream started');
      
    } catch (error) {
      console.error('[VideoFeed] ❌ Start failed:', error);
      toast.error('Failed to start: ' + error);
    } finally {
      startingStream = false;
    }
  }
  
  async function stopStream() {
    if (!streaming) return;
    
    try {
      await invoke('stop_video_stream');
      streaming = false;
      droneStore.setVideoActive(false);
      toast.info('Stream stopped');
    } catch (error) {
      console.error('[VideoFeed] Failed to stop:', error);
    }
  }
  
  async function takePicture() {
    if (!canvas) return;
    
    try {
      const dataUrl = canvas.toDataURL('image/png');
      const timestamp = Date.now();
      const filename = `tello_${timestamp}.png`;
      
      galleryStore.addImage({
        id: `img_${timestamp}`,
        filename,
        path: dataUrl,
        timestamp,
        size: dataUrl.length,
        thumbnail: dataUrl
      });
      
      toast.success('Picture saved');
    } catch (error) {
      toast.error('Capture failed');
    }
  }
  
  onDestroy(() => {
    streaming = false;
    if (unlistenVideo) unlistenVideo();
    if (decoder) decoder.close();
  });
</script>

<div class="h-full w-full flex flex-col p-4" style="background-color: #000000">
  <!-- Controls -->
  <div class="flex items-center justify-between mb-2 px-2">
    <div class="flex items-center gap-2">
      <div 
        class="w-2 h-2 rounded-full" 
        style="background-color: {streaming && fps > 0 ? '#ef4444' : '#6b7280'}; {streaming && fps > 0 ? 'animation: pulse 2s infinite' : ''}"
      ></div>
      <span class="text-sm font-medium" style="color: white">
        {#if streaming && fps > 0}
          LIVE - {fps} FPS
        {:else if streaming || startingStream}
          LIVE - Waiting...
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
        <Button on:click={startStream} size="sm" disabled={!$droneStore.connected || startingStream}>
          {#if startingStream}
            <Loader2 class="h-3 w-3 mr-1 animate-spin" />
          {:else}
            <Video class="h-3 w-3 mr-1" />
          {/if}
          {startingStream ? 'Starting...' : 'Start'}
        </Button>
      {/if}
      
      <Button on:click={takePicture} variant="outline" size="sm" disabled={!streaming || fps === 0}>
        <Camera class="h-3 w-3" />
      </Button>
    </div>
  </div>
  
  <!-- Video Canvas -->
  <div class="flex-1 relative flex items-center justify-center">
    <canvas 
      bind:this={canvas}
      class="max-w-full max-h-full"
      style="display: block; object-fit: contain; background-color: #000000"
    />
    
    {#if !$droneStore.connected}
      <div class="absolute inset-0 flex flex-col items-center justify-center" style="background-color: rgba(0,0,0,0.95)">
        <p class="text-xl font-medium mb-2" style="color: white">Not Connected</p>
        <p class="text-sm" style="color: #9ca3af">Connect to view live video</p>
      </div>
    {:else if startingStream || (streaming && fps === 0)}
      <div class="absolute inset-0 flex flex-col items-center justify-center" style="background-color: rgba(0,0,0,0.85); pointer-events: none;">
        <Loader2 class="h-16 w-16 mb-4 animate-spin" style="color: #3b82f6" />
        <p class="text-lg font-medium" style="color: white">
          {startingStream ? 'Starting...' : 'Waiting for video...'}
        </p>
      </div>
    {/if}
  </div>
</div>

<style>
  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }
  .animate-spin { animation: spin 1s linear infinite; }
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
</style>
