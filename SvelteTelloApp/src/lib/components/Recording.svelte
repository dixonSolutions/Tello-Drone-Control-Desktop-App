<script lang="ts">
  import { droneStore } from '$lib/stores/drone';
  import Card from './ui/card/Card.svelte';
  import CardHeader from './ui/card/CardHeader.svelte';
  import CardTitle from './ui/card/CardTitle.svelte';
  import CardContent from './ui/card/CardContent.svelte';
  import Button from './ui/button/Button.svelte';
  import { Circle, Square } from 'lucide-svelte';
  import { toast } from 'svelte-sonner';
  
  let recording = false;
  let recordingStartTime = 0;
  let elapsedTime = '00:00';
  let recordingInterval: number;
  
  function startRecording() {
    if (!$droneStore.videoActive) {
      toast.error('Start video stream first');
      return;
    }
    
    recording = true;
    recordingStartTime = Date.now();
    
    recordingInterval = setInterval(() => {
      const elapsed = Math.floor((Date.now() - recordingStartTime) / 1000);
      const minutes = Math.floor(elapsed / 60);
      const seconds = elapsed % 60;
      elapsedTime = `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
    }, 1000);
    
    // TODO: Implement actual recording
    toast.success('Recording started');
  }
  
  function stopRecording() {
    recording = false;
    if (recordingInterval) {
      clearInterval(recordingInterval);
    }
    elapsedTime = '00:00';
    
    // TODO: Implement actual recording stop and save
    toast.success('Recording saved');
  }
</script>

<Card>
  <CardHeader>
    <CardTitle>Recording</CardTitle>
  </CardHeader>
  <CardContent>
    <div class="space-y-4">
      {#if recording}
        <div class="flex items-center justify-center gap-3 p-4 bg-error text-white rounded-lg">
          <Circle class="h-4 w-4 fill-current animate-pulse" />
          <span class="font-mono font-bold text-lg">{elapsedTime}</span>
        </div>
      {/if}
      
      <Button 
        on:click={recording ? stopRecording : startRecording}
        variant={recording ? 'destructive' : 'default'}
        class="w-full"
        disabled={!$droneStore.videoActive && !recording}
      >
        {#if recording}
          <Square class="mr-2 h-4 w-4" />
          Stop Recording
        {:else}
          <Circle class="mr-2 h-4 w-4" />
          Start Recording
        {/if}
      </Button>
      
      {#if !$droneStore.videoActive && !recording}
        <p class="text-sm text-center theme-text-muted">
          Start video stream to record
        </p>
      {/if}
    </div>
  </CardContent>
</Card>

