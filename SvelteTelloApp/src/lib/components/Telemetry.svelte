<script lang="ts">
  import { droneStore, batteryWarning } from '$lib/stores/drone';
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { Battery, BatteryWarning, Thermometer, Ruler, Compass } from 'lucide-svelte';
  
  let interval: number;
  
  onMount(() => {
    // Update telemetry every 5 seconds - also serves as connection health check
    interval = setInterval(async () => {
      if ($droneStore.connected) {
        try {
          const telemetry: any = await invoke('get_telemetry');
          droneStore.setBattery(telemetry.battery);
          droneStore.setTemperature(telemetry.temperature);
          droneStore.setHeight(telemetry.height);
          droneStore.setAttitude(telemetry.pitch, telemetry.roll, telemetry.yaw);
        } catch (error) {
          console.error('Telemetry failed - drone disconnected:', error);
          // Failed to retrieve battery/telemetry = drone is not connected
          droneStore.setConnected(false);
          droneStore.setFlying(false);
          droneStore.setVideoActive(false);
        }
      }
    }, 5000); // 5 seconds like Python app
  });
  
  onDestroy(() => {
    if (interval) clearInterval(interval);
  });
</script>

{#if $droneStore.connected}
  <div class="flex items-center gap-4 text-sm">
    <!-- Battery -->
    <div class="flex items-center gap-1.5" class:text-error={$batteryWarning} class:text-warning={$droneStore.battery < 30 && $droneStore.battery >= 15}>
      {#if $batteryWarning}
        <BatteryWarning class="h-4 w-4" />
      {:else}
        <Battery class="h-4 w-4" />
      {/if}
      <span class="font-medium">{$droneStore.battery}%</span>
    </div>
    
    <!-- Height -->
    <div class="flex items-center gap-1.5 theme-text-muted">
      <Ruler class="h-4 w-4" />
      <span>{$droneStore.height}cm</span>
    </div>
    
    <!-- Temperature -->
    <div class="flex items-center gap-1.5 theme-text-muted">
      <Thermometer class="h-4 w-4" />
      <span>{$droneStore.temperature}°C</span>
    </div>
    
    <!-- Attitude -->
    <div class="flex items-center gap-1.5 theme-text-muted">
      <Compass class="h-4 w-4" />
      <span class="text-xs">
        P:{$droneStore.pitch}° R:{$droneStore.roll}° Y:{$droneStore.yaw}°
      </span>
    </div>
  </div>
{/if}
