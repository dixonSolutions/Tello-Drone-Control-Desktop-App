<script lang="ts">
  import { settingsStore } from '$lib/stores/settings';
  import { droneStore } from '$lib/stores/drone';
  import Card from './ui/card/Card.svelte';
  import CardHeader from './ui/card/CardHeader.svelte';
  import CardTitle from './ui/card/CardTitle.svelte';
  import CardContent from './ui/card/CardContent.svelte';
  import Label from './ui/label/Label.svelte';
  import Input from './ui/input/Input.svelte';
  import Button from './ui/button/Button.svelte';
  import { Settings as SettingsIcon, Save } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { toast } from 'svelte-sonner';
  
  let normalSpeed = $settingsStore.normalModeSpeed;
  let fastSpeed = $settingsStore.fastModeSpeed;
  let batteryWarning = $settingsStore.batteryWarningLevel;
  let autoLand = $settingsStore.autoLandOnLowBattery;
  let keyboardControls = $settingsStore.enableKeyboardControls;
  
  async function saveSettings() {
    $settingsStore.update({
      normalModeSpeed: normalSpeed,
      fastModeSpeed: fastSpeed,
      batteryWarningLevel: batteryWarning,
      autoLandOnLowBattery: autoLand,
      enableKeyboardControls: keyboardControls,
    });
    
    // Update drone speed if connected
    if ($droneStore.connected) {
      try {
        await invoke('set_speed', { speed: normalSpeed });
      } catch (error) {
        console.error('Failed to update speed:', error);
      }
    }
    
    toast.success('Settings saved');
  }
  
  function resetSettings() {
    if (confirm('Reset all settings to defaults?')) {
      $settingsStore.reset();
      normalSpeed = $settingsStore.normalModeSpeed;
      fastSpeed = $settingsStore.fastModeSpeed;
      batteryWarning = $settingsStore.batteryWarningLevel;
      autoLand = $settingsStore.autoLandOnLowBattery;
      keyboardControls = $settingsStore.enableKeyboardControls;
      toast.info('Settings reset to defaults');
    }
  }
</script>

<Card>
  <CardHeader>
    <CardTitle class="flex items-center gap-2">
      <SettingsIcon class="h-5 w-5" />
      Settings
    </CardTitle>
  </CardHeader>
  <CardContent>
    <div class="space-y-6">
      <!-- Speed Settings -->
      <div class="space-y-4">
        <h3 class="text-sm font-semibold theme-text">Flight Speed</h3>
        
        <div class="space-y-2">
          <Label for="normal-speed">Normal Speed (cm/s)</Label>
          <Input 
            id="normal-speed"
            type="number"
            bind:value={normalSpeed}
            min="10"
            max="100"
          />
          <p class="text-xs theme-text-muted">Default speed for movements</p>
        </div>
        
        <div class="space-y-2">
          <Label for="fast-speed">Fast Speed (cm/s)</Label>
          <Input 
            id="fast-speed"
            type="number"
            bind:value={fastSpeed}
            min="10"
            max="100"
          />
          <p class="text-xs theme-text-muted">Speed when in fast mode</p>
        </div>
      </div>
      
      <!-- Battery Settings -->
      <div class="space-y-4">
        <h3 class="text-sm font-semibold theme-text">Battery Management</h3>
        
        <div class="space-y-2">
          <Label for="battery-warning">Low Battery Warning (%)</Label>
          <Input 
            id="battery-warning"
            type="number"
            bind:value={batteryWarning}
            min="10"
            max="50"
          />
          <p class="text-xs theme-text-muted">Alert when battery drops below this level</p>
        </div>
        
        <div class="flex items-center gap-2">
          <input 
            type="checkbox"
            id="auto-land"
            bind:checked={autoLand}
            class="w-4 h-4"
          />
          <Label for="auto-land">Auto-land on low battery</Label>
        </div>
      </div>
      
      <!-- Control Settings -->
      <div class="space-y-4">
        <h3 class="text-sm font-semibold theme-text">Controls</h3>
        
        <div class="flex items-center gap-2">
          <input 
            type="checkbox"
            id="keyboard-controls"
            bind:checked={keyboardControls}
            class="w-4 h-4"
          />
          <Label for="keyboard-controls">Enable keyboard controls</Label>
        </div>
      </div>
      
      <!-- Save/Reset -->
      <div class="flex gap-2 pt-4 border-t theme-border">
        <Button 
          on:click={saveSettings}
          class="flex-1 theme-primary"
        >
          <Save class="mr-2 h-4 w-4" />
          Save Settings
        </Button>
        <Button 
          on:click={resetSettings}
          variant="outline"
        >
          Reset
        </Button>
      </div>
      
      <div class="p-3 bg-info bg-opacity-10 rounded-md text-xs theme-text-muted">
        <p class="font-semibold mb-1">Note:</p>
        <p>Some settings require reconnecting to the drone to take effect.</p>
      </div>
    </div>
  </CardContent>
</Card>

