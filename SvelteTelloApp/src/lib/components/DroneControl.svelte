<script lang="ts">
  import { droneStore, connectionStore, batteryWarning } from '$lib/stores/drone';
  import Card from './ui/card/Card.svelte';
  import CardHeader from './ui/card/CardHeader.svelte';
  import CardTitle from './ui/card/CardTitle.svelte';
  import CardContent from './ui/card/CardContent.svelte';
  import Button from './ui/button/Button.svelte';
  import { 
    Play, 
    Square, 
    AlertTriangle,
    ArrowUp, 
    ArrowDown, 
    ArrowLeft, 
    ArrowRight,
    RotateCcw,
    RotateCw,
    MoveUp,
    MoveDown
  } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { toast } from 'svelte-sonner';
  
  let connecting = false;
  let flying = false;
  let speed = 50;
  
  $: flying = $droneStore.flying;
  $: speed = $droneStore.speed;
  
  async function connect() {
    console.log('[DroneControl] Initiating connection...');
    console.log('[DroneControl] Timestamp:', new Date().toISOString());
    connecting = true;
    connectionStore.setStatus('connecting', 'Connecting...');
    
    try {
      console.log('[DroneControl] Invoking connect_drone command...');
      const result: any = await invoke('connect_drone');
      console.log('[DroneControl] Connect result:', result);
      
      if (result.success) {
        droneStore.setConnected(true);
        connectionStore.setStatus('connected', 'Connected to drone');
        toast.success('Connected to drone');
        console.log('[DroneControl] ✅ Successfully connected to drone');
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      console.error('[DroneControl] ❌ Connection failed:', error);
      connectionStore.setStatus('error', String(error));
      toast.error('Connection failed: ' + error);
    } finally {
      connecting = false;
      console.log('[DroneControl] Connection attempt finished');
    }
  }
  
  async function disconnect() {
    console.log('[DroneControl] Initiating disconnect...');
    try {
      console.log('[DroneControl] Invoking disconnect_drone command...');
      await invoke('disconnect_drone');
      droneStore.reset();
      connectionStore.setStatus('disconnected', 'Disconnected');
      toast.info('Disconnected from drone');
      console.log('[DroneControl] ✅ Successfully disconnected');
    } catch (error) {
      console.error('[DroneControl] ❌ Disconnect failed:', error);
      toast.error('Disconnect failed: ' + error);
    }
  }
  
  async function takeoff() {
    console.log('[DroneControl] Takeoff requested');
    if ($batteryWarning) {
      console.warn('[DroneControl] ⚠️ Battery too low for takeoff');
      toast.error('Battery too low for takeoff');
      return;
    }
    
    try {
      console.log('[DroneControl] Invoking takeoff command...');
      const result: any = await invoke('takeoff');
      console.log('[DroneControl] Takeoff result:', result);
      
      if (result.success) {
        droneStore.setFlying(true);
        toast.success('Taking off...');
        console.log('[DroneControl] ✅ Drone taking off');
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      console.error('[DroneControl] ❌ Takeoff failed:', error);
      toast.error('Takeoff failed: ' + error);
    }
  }
  
  async function land() {
    console.log('[DroneControl] Land requested');
    try {
      console.log('[DroneControl] Invoking land command...');
      const result: any = await invoke('land');
      console.log('[DroneControl] Land result:', result);
      
      if (result.success) {
        droneStore.setFlying(false);
        toast.success('Landing...');
        console.log('[DroneControl] ✅ Drone landing');
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      console.error('[DroneControl] ❌ Land failed:', error);
      toast.error('Land failed: ' + error);
    }
  }
  
  async function emergency() {
    console.log('[DroneControl] 🚨 EMERGENCY STOP ACTIVATED');
    try {
      await invoke('emergency');
      droneStore.setFlying(false);
      toast.error('EMERGENCY STOP');
      console.log('[DroneControl] Emergency stop executed');
    } catch (error) {
      console.error('[DroneControl] Emergency stop failed:', error);
    }
  }
  
  async function sendRC(x: number, y: number, z: number, yaw: number, duration: number = 500) {
    console.log('[DroneControl] RC control:', { x, y, z, yaw, duration });
    try {
      await invoke('send_rc_control', {
        leftRight: x,
        forwardBack: y,
        upDown: z,
        yaw: yaw
      });
      
      // Hold for duration
      await new Promise(resolve => setTimeout(resolve, duration));
      
      // Stop movement
      await invoke('send_rc_control', {
        leftRight: 0,
        forwardBack: 0,
        upDown: 0,
        yaw: 0
      });
      console.log('[DroneControl] RC control completed');
    } catch (error) {
      console.error('[DroneControl] RC control failed:', error);
    }
  }
  
  // Movement functions - x (left/right), y (forward/back), z (up/down), yaw (rotation)
  function nudgeForward() { sendRC(0, speed, 0, 0); }
  function nudgeBack() { sendRC(0, -speed, 0, 0); }
  function nudgeLeft() { sendRC(-speed, 0, 0, 0); }
  function nudgeRight() { sendRC(speed, 0, 0, 0); }
  function nudgeUp() { sendRC(0, 0, speed, 0); }
  function nudgeDown() { sendRC(0, 0, -speed, 0); }
  function rotateLeft() { sendRC(0, 0, 0, -speed); }
  function rotateRight() { sendRC(0, 0, 0, speed); }
  function stopMovement() { sendRC(0, 0, 0, 0, 50); }
</script>

<Card>
  <CardHeader>
    <CardTitle>Drone Control</CardTitle>
  </CardHeader>
  <CardContent>
    <div class="space-y-6">
      <!-- Connection Controls -->
      <div class="flex items-center justify-between">
        <div class="text-sm theme-text-muted">
          Status: 
          <span class:text-success={$droneStore.connected} class:text-error={!$droneStore.connected}>
            {$droneStore.connected ? 'Connected' : 'Disconnected'}
          </span>
        </div>
        {#if $droneStore.connected}
          <Button on:click={disconnect} variant="destructive" size="sm">
            <Square class="mr-2 h-4 w-4" />
            Disconnect
          </Button>
        {:else}
          <Button on:click={connect} size="sm" disabled={connecting}>
            <Play class="mr-2 h-4 w-4" />
            {connecting ? 'Connecting...' : 'Connect'}
          </Button>
        {/if}
      </div>

      {#if $batteryWarning}
        <div class="flex items-center gap-2 p-3 rounded-md bg-warning text-white">
          <AlertTriangle class="h-5 w-5" />
          <span class="text-sm font-medium">Low Battery Warning!</span>
        </div>
      {/if}

      {#if $droneStore.connected}
        <!-- Flight Controls -->
        <div class="space-y-4">
          <div class="flex items-center justify-center gap-2">
            <Button 
              on:click={takeoff} 
              variant="default"
              disabled={flying}
              class="theme-primary"
            >
              Takeoff
            </Button>
            <Button 
              on:click={land} 
              variant="secondary"
              disabled={!flying}
            >
              Land
            </Button>
            <Button 
              on:click={emergency} 
              variant="destructive"
              size="sm"
            >
              <AlertTriangle class="mr-2 h-4 w-4" />
              EMERGENCY
            </Button>
          </div>

          <!-- Movement Controls -->
          <div class="grid grid-cols-3 gap-2">
            <!-- Forward -->
            <div class="col-start-2">
              <Button 
                on:click={nudgeForward} 
                variant="outline" 
                class="w-full"
                disabled={!flying}
              >
                <ArrowUp class="h-5 w-5" />
              </Button>
            </div>
            
            <!-- Left, Stop, Right -->
            <div class="col-start-1 row-start-2">
              <Button 
                on:click={nudgeLeft} 
                variant="outline" 
                class="w-full"
                disabled={!flying}
              >
                <ArrowLeft class="h-5 w-5" />
              </Button>
            </div>
            <div class="col-start-2 row-start-2">
              <Button 
                on:click={stopMovement} 
                variant="outline" 
                class="w-full"
                disabled={!flying}
              >
                ■
              </Button>
            </div>
            <div class="col-start-3 row-start-2">
              <Button 
                on:click={nudgeRight} 
                variant="outline" 
                class="w-full"
                disabled={!flying}
              >
                <ArrowRight class="h-5 w-5" />
              </Button>
            </div>
            
            <!-- Back -->
            <div class="col-start-2 row-start-3">
              <Button 
                on:click={nudgeBack} 
                variant="outline" 
                class="w-full"
                disabled={!flying}
              >
                <ArrowDown class="h-5 w-5" />
              </Button>
            </div>
          </div>

          <!-- Altitude & Rotation Controls -->
          <div class="grid grid-cols-2 gap-4">
            <!-- Altitude -->
            <div class="space-y-2">
              <p class="text-sm font-medium theme-text-muted text-center">Altitude</p>
              <div class="flex flex-col gap-2">
                <Button 
                  on:click={nudgeUp} 
                  variant="outline"
                  disabled={!flying}
                  class="w-full"
                >
                  <MoveUp class="mr-2 h-4 w-4" />
                  Up
                </Button>
                <Button 
                  on:click={nudgeDown} 
                  variant="outline"
                  disabled={!flying}
                  class="w-full"
                >
                  <MoveDown class="mr-2 h-4 w-4" />
                  Down
                </Button>
              </div>
            </div>
            
            <!-- Rotation -->
            <div class="space-y-2">
              <p class="text-sm font-medium theme-text-muted text-center">Rotation</p>
              <div class="flex flex-col gap-2">
                <Button 
                  on:click={rotateLeft} 
                  variant="outline"
                  disabled={!flying}
                  class="w-full"
                >
                  <RotateCcw class="mr-2 h-4 w-4" />
                  Left
                </Button>
                <Button 
                  on:click={rotateRight} 
                  variant="outline"
                  disabled={!flying}
                  class="w-full"
                >
                  <RotateCw class="mr-2 h-4 w-4" />
                  Right
                </Button>
              </div>
            </div>
          </div>
        </div>
      {:else}
        <div class="text-center py-8 theme-text-muted">
          <p>Connect to drone to access controls</p>
        </div>
      {/if}
    </div>
  </CardContent>
</Card>
