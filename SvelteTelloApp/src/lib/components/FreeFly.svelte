<script lang="ts">
  import { droneStore } from '$lib/stores/drone';
  import Card from './ui/card/Card.svelte';
  import CardHeader from './ui/card/CardHeader.svelte';
  import CardTitle from './ui/card/CardTitle.svelte';
  import CardContent from './ui/card/CardContent.svelte';
  import Button from './ui/button/Button.svelte';
  import { Plane, Play, Square } from 'lucide-svelte';
  import { toast } from 'svelte-sonner';
  
  let freeFlyActive = false;
  let debugInfo = {
    mode: 'scan',
    edgeDensity: 0,
    lapVar: 0,
    looming: false,
    divergence: 0,
    tofCm: null as number | null
  };
  
  function toggleFreeFly() {
    if (!$droneStore.flying) {
      toast.error('Drone must be flying to start Free Fly');
      return;
    }
    
    if (!freeFlyActive) {
      // TODO: Start free fly mode via Tauri backend
      freeFlyActive = true;
      toast.success('Free Fly mode activated');
    } else {
      // TODO: Stop free fly mode
      freeFlyActive = false;
      toast.info('Free Fly mode deactivated');
    }
  }
</script>

<Card>
  <CardHeader>
    <CardTitle class="flex items-center gap-2">
      <Plane class="h-5 w-5" />
      Free Fly Mode
    </CardTitle>
  </CardHeader>
  <CardContent>
    <div class="space-y-4">
      <p class="text-sm theme-text-muted">
        Autonomous navigation using edge detection and optical flow. The drone will maintain altitude and avoid obstacles.
      </p>
      
      {#if $droneStore.connected && $droneStore.flying}
        <Button 
          on:click={toggleFreeFly}
          variant={freeFlyActive ? 'destructive' : 'default'}
          class="w-full"
        >
          {#if freeFlyActive}
            <Square class="mr-2 h-4 w-4" />
            Stop Free Fly
          {:else}
            <Play class="mr-2 h-4 w-4" />
            Start Free Fly
          {/if}
        </Button>
        
        {#if freeFlyActive}
          <div class="space-y-2 p-3 theme-surface rounded-lg border theme-border">
            <p class="text-sm font-semibold theme-text">Debug Info:</p>
            <div class="grid grid-cols-2 gap-2 text-xs theme-text-muted font-mono">
              <div>Mode: <span class="text-info">{debugInfo.mode}</span></div>
              <div>Edge: {debugInfo.edgeDensity.toFixed(1)}</div>
              <div>LapVar: {debugInfo.lapVar.toFixed(1)}</div>
              <div>Looming: {debugInfo.looming ? 'YES' : 'NO'}</div>
              <div>Divergence: {debugInfo.divergence.toFixed(2)}</div>
              <div>ToF: {debugInfo.tofCm ?? 'N/A'}cm</div>
            </div>
          </div>
        {/if}
      {:else if $droneStore.connected}
        <div class="text-center py-8 theme-text-muted">
          <Plane class="h-12 w-12 mx-auto mb-3 opacity-50" />
          <p>Drone must be flying to start Free Fly mode</p>
        </div>
      {:else}
        <div class="text-center py-8 theme-text-muted">
          <p>Connect to drone and takeoff first</p>
        </div>
      {/if}
      
      <div class="p-3 bg-info bg-opacity-10 rounded-md text-sm theme-text-muted">
        <p class="font-semibold mb-1">How it works:</p>
        <ul class="list-disc list-inside space-y-1">
          <li>Scans environment for clear paths</li>
          <li>Moves forward when safe</li>
          <li>Evades obstacles automatically</li>
          <li>Maintains safe altitude</li>
        </ul>
      </div>
    </div>
  </CardContent>
</Card>

