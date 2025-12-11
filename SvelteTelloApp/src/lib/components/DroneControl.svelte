<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
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
    MoveDown,
    ChevronUp,
    ChevronDown,
    ChevronLeft,
    ChevronRight
  } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { toast } from 'svelte-sonner';
  
  let connecting = false;
  let flying = false;
  let speed = 50;
  let pressedKeys = new Set<string>();
  let rcInterval: number;
  
  $: flying = $droneStore.flying;
  $: speed = $droneStore.speed;
  
  onMount(() => {
    // Keyboard controls
    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);
    
    // Start RC control loop
    startRCLoop();
    
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
      if (rcInterval) clearInterval(rcInterval);
    };
  });
  
  function handleKeyDown(e: KeyboardEvent) {
    const key = e.key.toLowerCase();
    pressedKeys.add(key);
    
    // Space bar emergency stop
    if (key === ' ') {
      e.preventDefault();
      emergency();
    }
  }
  
  function handleKeyUp(e: KeyboardEvent) {
    pressedKeys.delete(e.key.toLowerCase());
  }
  
  function startRCLoop() {
    // Send RC commands at 10Hz like Python app
    rcInterval = setInterval(() => {
      if (!flying || !$droneStore.connected) return;
      
      let lr = 0;  // left/right
      let fb = 0;  // forward/back
      let ud = 0;  // up/down
      let yaw = 0; // rotation
      
      // WASD controls
      if (pressedKeys.has('w') || pressedKeys.has('arrowup')) fb = speed;
      if (pressedKeys.has('s') || pressedKeys.has('arrowdown')) fb = -speed;
      if (pressedKeys.has('a')) lr = -speed;
      if (pressedKeys.has('d')) lr = speed;
      
      // Arrow keys for yaw (rotation)
      if (pressedKeys.has('arrowleft')) yaw = -speed;
      if (pressedKeys.has('arrowright')) yaw = speed;
      
      // Q/E for up/down
      if (pressedKeys.has('q')) ud = speed;
      if (pressedKeys.has('e')) ud = -speed;
      
      if (lr !== 0 || fb !== 0 || ud !== 0 || yaw !== 0) {
        sendRC(lr, fb, ud, yaw).catch(() => {});
      }
    }, 100);
  }
  
  async function connect() {
    console.log('[DroneControl] Initiating connection...');
    connecting = true;
    connectionStore.setStatus('connecting', 'Connecting...');
    
    try {
      const result: any = await invoke('connect_drone');
      
      if (result.success) {
        droneStore.setConnected(true);
        connectionStore.setStatus('connected', 'Connected to drone');
        toast.success('Connected to drone');
        console.log('[DroneControl] ✅ Successfully connected');
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      console.error('[DroneControl] ❌ Connection failed:', error);
      connectionStore.setStatus('error', String(error));
      toast.error('Connection failed: ' + error);
    } finally {
      connecting = false;
    }
  }
  
  async function disconnect() {
    try {
      await invoke('disconnect_drone');
      droneStore.reset();
      connectionStore.setStatus('disconnected', 'Disconnected');
      toast.info('Disconnected from drone');
    } catch (error) {
      console.error('[DroneControl] ❌ Disconnect failed:', error);
      toast.error('Disconnect failed: ' + error);
    }
  }
  
  async function takeoff() {
    if ($batteryWarning) {
      toast.error('Battery too low for takeoff');
      return;
    }
    
    try {
      const result: any = await invoke('takeoff');
      
      if (result.success) {
        droneStore.setFlying(true);
        toast.success('Taking off...');
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      console.error('[DroneControl] ❌ Takeoff failed:', error);
      toast.error('Takeoff failed: ' + error);
    }
  }
  
  async function land() {
    try {
      console.log('[DroneControl] Landing...');
      const result: any = await invoke('land');
      
      if (result.success) {
        droneStore.setFlying(false);
        toast.success('Landing...');
        console.log('[DroneControl] ✅ Landing initiated');
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      console.error('[DroneControl] ❌ Land failed:', error);
      toast.error('Land failed: ' + error);
    }
  }
  
  async function emergency() {
    console.log('[DroneControl] 🚨 EMERGENCY STOP');
    try {
      await invoke('emergency');
      droneStore.setFlying(false);
      toast.error('EMERGENCY STOP');
    } catch (error) {
      console.error('[DroneControl] Emergency failed:', error);
    }
  }
  
  async function sendRC(x: number, y: number, z: number, yaw: number) {
    try {
      await invoke('send_rc_control', {
        leftRight: x,
        forwardBack: y,
        upDown: z,
        yaw: yaw
      });
    } catch (error) {
      // Silent fail for RC commands
    }
  }
  
  async function moveWithHold(x: number, y: number, z: number, yaw: number, duration: number = 500) {
    await sendRC(x, y, z, yaw);
    await new Promise(resolve => setTimeout(resolve, duration));
    await sendRC(0, 0, 0, 0);
  }
  
  // Movement functions
  function nudgeForward() { moveWithHold(0, speed, 0, 0); }
  function nudgeBack() { moveWithHold(0, -speed, 0, 0); }
  function nudgeLeft() { moveWithHold(-speed, 0, 0, 0); }
  function nudgeRight() { moveWithHold(speed, 0, 0, 0); }
  async function nudgeUp() { 
    try {
      // Use 20cm move like Python app
      await invoke('send_command', { command: 'up 20' });
      toast.info('Moving up 20cm');
    } catch (e) {
      moveWithHold(0, 0, speed, 0);
    }
  }
  async function nudgeDown() { 
    try {
      await invoke('send_command', { command: 'down 20' });
      toast.info('Moving down 20cm');
    } catch (e) {
      moveWithHold(0, 0, -speed, 0);
    }
  }
  function rotateLeft() { moveWithHold(0, 0, 0, -speed); }
  function rotateRight() { moveWithHold(0, 0, 0, speed); }
  function stopMovement() { sendRC(0, 0, 0, 0); }
</script>

<Card>
  <CardHeader>
    <CardTitle>Drone Control</CardTitle>
  </CardHeader>
  <CardContent>
    <div class="space-y-4">
      <!-- Connection Controls -->
      <div class="flex items-center justify-between">
        <div class="text-sm" style="color: var(--color-text-muted)">
          Status: 
          <span style="color: {$droneStore.connected ? 'var(--color-success)' : 'var(--color-error)'}">
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
        <div class="flex items-center gap-2 p-2 rounded" style="background-color: var(--color-warning); color: white">
          <AlertTriangle class="h-4 w-4" />
          <span class="text-sm font-medium">Low Battery!</span>
        </div>
      {/if}

      {#if $droneStore.connected}
        <!-- Flight Controls -->
        <div class="space-y-3">
          <div class="flex items-center justify-center gap-2">
            <Button 
              on:click={takeoff} 
              variant="default"
              disabled={flying}
              size="sm"
            >
              Takeoff
            </Button>
            <Button 
              on:click={land} 
              variant="secondary"
              disabled={!flying}
              size="sm"
            >
              Land
            </Button>
            <Button 
              on:click={emergency} 
              variant="destructive"
              size="sm"
            >
              <AlertTriangle class="mr-1 h-3 w-3" />
              STOP
            </Button>
          </div>

          <!-- Movement Controls Grid -->
          <div class="grid grid-cols-3 gap-1">
            <!-- Forward -->
            <div class="col-start-2">
              <Button 
                on:click={nudgeForward} 
                variant="outline" 
                class="w-full h-10"
                disabled={!flying}
                size="sm"
              >
                <ChevronUp class="h-5 w-5" />
              </Button>
            </div>
            
            <!-- Left, Stop, Right -->
            <Button 
              on:click={nudgeLeft} 
              variant="outline" 
              class="w-full h-10"
              disabled={!flying}
              size="sm"
            >
              <ChevronLeft class="h-5 w-5" />
            </Button>
            <Button 
              on:click={stopMovement} 
              variant="outline" 
              class="w-full h-10"
              disabled={!flying}
              size="sm"
            >
              ■
            </Button>
            <Button 
              on:click={nudgeRight} 
              variant="outline" 
              class="w-full h-10"
              disabled={!flying}
              size="sm"
            >
              <ChevronRight class="h-5 w-5" />
            </Button>
            
            <!-- Back -->
            <div class="col-start-2">
              <Button 
                on:click={nudgeBack} 
                variant="outline" 
                class="w-full h-10"
                disabled={!flying}
                size="sm"
              >
                <ChevronDown class="h-5 w-5" />
              </Button>
            </div>
          </div>

          <!-- Altitude & Rotation -->
          <div class="grid grid-cols-2 gap-2">
            <div class="space-y-1">
              <p class="text-xs font-medium text-center" style="color: var(--color-text-muted)">Altitude</p>
              <Button 
                on:click={nudgeUp} 
                variant="outline"
                disabled={!flying}
                class="w-full"
                size="sm"
              >
                <MoveUp class="mr-1 h-3 w-3" />
                Up 20cm
              </Button>
              <Button 
                on:click={nudgeDown} 
                variant="outline"
                disabled={!flying}
                class="w-full"
                size="sm"
              >
                <MoveDown class="mr-1 h-3 w-3" />
                Down 20cm
              </Button>
            </div>
            
            <div class="space-y-1">
              <p class="text-xs font-medium text-center" style="color: var(--color-text-muted)">Rotation</p>
              <Button 
                on:click={rotateLeft} 
                variant="outline"
                disabled={!flying}
                class="w-full"
                size="sm"
              >
                <RotateCcw class="mr-1 h-3 w-3" />
                Left
              </Button>
              <Button 
                on:click={rotateRight} 
                variant="outline"
                disabled={!flying}
                class="w-full"
                size="sm"
              >
                <RotateCw class="mr-1 h-3 w-3" />
                Right
              </Button>
            </div>
          </div>

          <!-- Keyboard Hints -->
          <div class="text-xs p-2 rounded" style="background-color: var(--color-surface); color: var(--color-text-muted)">
            <p class="font-semibold mb-1">Keyboard Controls:</p>
            <p>W/S: Forward/Back | A/D: Left/Right</p>
            <p>Q/E: Up/Down | ←/→: Rotate | SPACE: Emergency Stop</p>
          </div>
        </div>
      {:else}
        <div class="text-center py-6" style="color: var(--color-text-muted)">
          <p>Connect to drone to access controls</p>
        </div>
      {/if}
    </div>
  </CardContent>
</Card>
