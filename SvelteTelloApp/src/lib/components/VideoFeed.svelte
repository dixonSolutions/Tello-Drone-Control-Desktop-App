<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { droneStore } from '$lib/stores/drone';
  import { recognitionStore } from '$lib/stores/recognition';
  import Card from './ui/card/Card.svelte';
  import CardHeader from './ui/card/CardHeader.svelte';
  import CardTitle from './ui/card/CardTitle.svelte';
  import CardContent from './ui/card/CardContent.svelte';
  import Button from './ui/button/Button.svelte';
  import { Play, Square, Camera, VideoIcon } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { toast } from 'svelte-sonner';
  
  let streaming = false;
  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null;
  let streamInterval: number;
  
  onMount(() => {
    if (canvas) {
      ctx = canvas.getContext('2d');
    }
  });
  
  async function toggleStream() {
    if (!$droneStore.connected) {
      toast.error('Connect to drone first');
      return;
    }
    
    if (!streaming) {
      try {
        await invoke('start_video_stream');
        streaming = true;
        $droneStore.setVideoActive(true);
        toast.success('Video stream started');
        startVideoLoop();
      } catch (error) {
        toast.error('Failed to start stream: ' + error);
      }
    } else {
      try {
        await invoke('stop_video_stream');
        streaming = false;
        $droneStore.setVideoActive(false);
        toast.info('Video stream stopped');
        stopVideoLoop();
      } catch (error) {
        toast.error('Failed to stop stream: ' + error);
      }
    }
  }
  
  function startVideoLoop() {
    // TODO: Implement actual video frame retrieval from UDP socket
    // For now, this is a placeholder
    streamInterval = setInterval(() => {
      if (ctx && canvas) {
        // Draw placeholder or actual frame here
        ctx.fillStyle = '#1a1a1a';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = '#ffffff';
        ctx.font = '20px sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText('Video Stream Active', canvas.width / 2, canvas.height / 2);
        ctx.font = '14px sans-serif';
        ctx.fillText('Waiting for frames...', canvas.width / 2, canvas.height / 2 + 30);
        
        // Draw recognition bbox if active
        if ($recognitionStore.active && $recognitionStore.bbox) {
          const bbox = $recognitionStore.bbox;
          ctx.strokeStyle = '#22c55e';
          ctx.lineWidth = 3;
          ctx.strokeRect(bbox.x, bbox.y, bbox.width, bbox.height);
          
          if ($recognitionStore.recognizedName) {
            ctx.fillStyle = '#22c55e';
            ctx.fillRect(bbox.x, bbox.y - 30, 200, 30);
            ctx.fillStyle = '#ffffff';
            ctx.font = '16px sans-serif';
            ctx.textAlign = 'left';
            ctx.fillText(
              `${$recognitionStore.recognizedName} (${Math.round($recognitionStore.confidence * 100)}%)`,
              bbox.x + 5,
              bbox.y - 10
            );
          }
        }
      }
    }, 33); // ~30 FPS
  }
  
  function stopVideoLoop() {
    if (streamInterval) {
      clearInterval(streamInterval);
    }
  }
  
  async function takePicture() {
    if (!streaming) {
      toast.error('Start video stream first');
      return;
    }
    
    // TODO: Implement picture capture
    toast.success('Picture captured');
  }
  
  onDestroy(() => {
    stopVideoLoop();
  });
</script>

<Card>
  <CardHeader>
    <CardTitle>Live Video Feed</CardTitle>
  </CardHeader>
  <CardContent>
    <div class="space-y-4">
      <div class="aspect-video bg-black rounded-lg overflow-hidden relative">
        <canvas 
          bind:this={canvas}
          width="960"
          height="720"
          class="w-full h-full"
        />
        
        {#if !streaming}
          <div class="absolute inset-0 flex flex-col items-center justify-center text-white bg-black bg-opacity-50">
            <VideoIcon class="h-12 w-12 mb-4 opacity-50" />
            <p class="text-lg">Video stream inactive</p>
            <p class="text-sm opacity-70">Connect and start stream to view</p>
          </div>
        {/if}
      </div>
      
      <div class="flex gap-2">
        <Button 
          on:click={toggleStream} 
          class="flex-1"
          disabled={!$droneStore.connected}
        >
          {#if streaming}
            <Square class="mr-2 h-4 w-4" />
            Stop Stream
          {:else}
            <Play class="mr-2 h-4 w-4" />
            Start Stream
          {/if}
        </Button>
        
        <Button 
          on:click={takePicture}
          variant="outline"
          disabled={!streaming}
        >
          <Camera class="h-4 w-4" />
        </Button>
      </div>
    </div>
  </CardContent>
</Card>
