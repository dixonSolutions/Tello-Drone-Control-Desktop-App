<script lang="ts">
  import { droneStore } from '$lib/stores/drone';
  import Card from './ui/card/Card.svelte';
  import CardHeader from './ui/card/CardHeader.svelte';
  import CardTitle from './ui/card/CardTitle.svelte';
  import CardContent from './ui/card/CardContent.svelte';
  import Switch from './ui/switch/Switch.svelte';
  import { Plane, Zap, Lock } from 'lucide-svelte';
  import { toast } from 'svelte-sonner';
  import { invoke } from '@tauri-apps/api/tauri';
  
  let freeFlyActive = false;
  let fastModeActive = false;
  let freeFlyLoading = false;
  let fastModeLoading = false;
  
  let debugInfo = {
    mode: 'scan',
    edgeDensity: 0,
    lapVar: 0,
    looming: false,
    divergence: 0,
    tofCm: null as number | null
  };
  
  async function toggleFastMode(checked: boolean) {
    if (!$droneStore.connected || !$droneStore.flying) {
      toast.error('Drone must be flying');
      fastModeActive = false;
      return;
    }
    
    fastModeLoading = true;
    try {
      if (checked) {
        await invoke('set_speed', { speed: 100 });
        droneStore.setSpeed(100);
        fastModeActive = true;
        toast.success('Fast Mode ON (100cm/s)');
      } else {
        await invoke('set_speed', { speed: 50 });
        droneStore.setSpeed(50);
        fastModeActive = false;
        toast.success('Fast Mode OFF (50cm/s)');
      }
    } catch (error) {
      console.error('Failed to toggle fast mode:', error);
      toast.error('Fast Mode toggle failed');
    } finally {
      fastModeLoading = false;
    }
  }
  
  function toggleFreeFly(checked: boolean) {
    if (!$droneStore.flying) {
      toast.error('Drone must be flying to start Free Fly');
      freeFlyActive = false;
      return;
    }
    
    freeFlyLoading = true;
    
    if (checked) {
      // TODO: Start free fly mode via Tauri backend
      setTimeout(() => {
        freeFlyActive = true;
        freeFlyLoading = false;
        toast.success('Free Fly mode activated');
      }, 500);
    } else {
      // TODO: Stop free fly mode
      setTimeout(() => {
        freeFlyActive = false;
        freeFlyLoading = false;
        toast.info('Free Fly mode deactivated');
      }, 500);
    }
  }
</script>

<!-- Fast Mode Card -->
<Card>
  <CardHeader>
    <div class="flex items-center justify-between">
      <CardTitle class="flex items-center gap-2">
        <Zap class="h-5 w-5" style="color: {fastModeActive ? 'var(--color-primary)' : 'var(--color-text-muted)'}" />
        Fast Mode
      </CardTitle>
      {#if !$droneStore.connected}
        <Lock class="h-4 w-4" style="color: var(--color-text-muted)" />
      {/if}
    </div>
  </CardHeader>
  <CardContent>
    <div class="space-y-3">
      <div class="flex items-center justify-between p-4 rounded-lg" style="background-color: var(--color-surface); border: 2px solid {fastModeActive ? 'var(--color-primary)' : 'var(--color-border)'}">
        <div class="flex-1">
          <p class="text-sm font-semibold mb-1" style="color: var(--color-text)">
            {fastModeActive ? 'Fast Speed Active' : 'Normal Speed'}
          </p>
          <p class="text-xs" style="color: var(--color-text-muted)">
            {fastModeActive ? '100cm/s' : '50cm/s'}
          </p>
        </div>
        
        <Switch 
          bind:checked={fastModeActive}
          disabled={!$droneStore.connected || !$droneStore.flying}
          loading={fastModeLoading}
          onCheckedChange={toggleFastMode}
        />
      </div>
      
      {#if !$droneStore.connected}
        <div class="text-xs text-center p-2 rounded" style="background-color: var(--color-surface); color: var(--color-text-muted); border: 1px dashed var(--color-border)">
          🔒 Connect to drone first
        </div>
      {:else if !$droneStore.flying}
        <div class="text-xs text-center p-2 rounded" style="background-color: var(--color-surface); color: var(--color-text-muted); border: 1px dashed var(--color-border)">
          🔒 Drone must be flying to enable fast mode
        </div>
      {:else}
        <p class="text-xs text-center" style="color: var(--color-text-muted)">
          Toggle between normal (50cm/s) and fast (100cm/s) flight speed
        </p>
      {/if}
    </div>
  </CardContent>
</Card>

<!-- Free Fly Card -->
<Card>
  <CardHeader>
    <div class="flex items-center justify-between">
      <CardTitle class="flex items-center gap-2">
        <Plane class="h-5 w-5" style="color: {freeFlyActive ? 'var(--color-primary)' : 'var(--color-text-muted)'}" />
        Free Fly Mode
      </CardTitle>
      {#if !$droneStore.flying}
        <Lock class="h-4 w-4" style="color: var(--color-text-muted)" />
      {/if}
    </div>
  </CardHeader>
  <CardContent>
    <div class="space-y-4">
      <div class="flex items-center justify-between p-4 rounded-lg" style="background-color: var(--color-surface); border: 2px solid {freeFlyActive ? 'var(--color-primary)' : 'var(--color-border)'}">
        <div class="flex-1">
          <p class="text-sm font-semibold mb-1" style="color: var(--color-text)">
            {freeFlyActive ? 'Autonomous Flight Active' : 'Manual Control'}
          </p>
          <p class="text-xs" style="color: var(--color-text-muted)">
            {freeFlyActive ? 'AI navigation enabled' : 'Pilot control'}
          </p>
        </div>
        
        <Switch 
          bind:checked={freeFlyActive}
          disabled={!$droneStore.flying || !$droneStore.connected}
          loading={freeFlyLoading}
          onCheckedChange={toggleFreeFly}
        />
      </div>
      
      {#if freeFlyActive}
        <div class="space-y-2 p-3 rounded-lg border" style="background-color: var(--color-surface); border-color: var(--color-border)">
          <p class="text-xs font-semibold" style="color: var(--color-text)">📊 Telemetry:</p>
          <div class="grid grid-cols-2 gap-2 text-xs font-mono" style="color: var(--color-text-muted)">
            <div>Mode: <span style="color: var(--color-primary)">{debugInfo.mode}</span></div>
            <div>Edge: {debugInfo.edgeDensity.toFixed(1)}</div>
            <div>LapVar: {debugInfo.lapVar.toFixed(1)}</div>
            <div>Looming: {debugInfo.looming ? '⚠️ YES' : '✓ NO'}</div>
            <div>Divergence: {debugInfo.divergence.toFixed(2)}</div>
            <div>ToF: {debugInfo.tofCm ?? 'N/A'}cm</div>
          </div>
        </div>
      {/if}
      
      {#if !$droneStore.connected}
        <div class="text-center py-6" style="color: var(--color-text-muted)">
          <Lock class="h-10 w-10 mx-auto mb-2 opacity-30" />
          <p class="text-sm font-medium">Not Connected</p>
          <p class="text-xs">Connect to drone first</p>
        </div>
      {:else if !$droneStore.flying}
        <div class="text-center py-6" style="color: var(--color-text-muted)">
          <Plane class="h-10 w-10 mx-auto mb-2 opacity-30" />
          <p class="text-sm font-medium">Drone Not Flying</p>
          <p class="text-xs">Takeoff required for Free Fly mode</p>
        </div>
      {:else}
        <div class="p-3 rounded-md text-xs" style="background-color: rgba(59, 130, 246, 0.1); color: var(--color-text-muted); border: 1px solid rgba(59, 130, 246, 0.2)">
          <p class="font-semibold mb-1" style="color: var(--color-text)">ℹ️ How it works:</p>
          <ul class="list-disc list-inside space-y-1">
            <li>Scans environment for clear paths</li>
            <li>Moves forward when safe</li>
            <li>Evades obstacles automatically</li>
            <li>Maintains safe altitude</li>
          </ul>
        </div>
      {/if}
    </div>
  </CardContent>
</Card>
