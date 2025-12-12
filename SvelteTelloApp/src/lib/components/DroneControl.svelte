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
    MoveUp,
    MoveDown,
    ChevronUp,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    RotateCcw,
    RotateCw,
    Loader2
  } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { toast } from 'svelte-sonner';
  
  let connecting = false;
  let flying = false;
  let speed = 50;
  let pressedKeys = new Set<string>();
  let rcInterval: number;
  let takingOff = false;
  let landing = false;
  
  $: flying = $droneStore.flying;
  $: speed = $droneStore.speed;
  
  onMount(() => {
    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);
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
    if (key === ' ') {
      e.preventDefault();
      emergency();
    }
  }
  
  function handleKeyUp(e: KeyboardEvent) {
    pressedKeys.delete(e.key.toLowerCase());
  }
  
  function startRCLoop() {
    rcInterval = setInterval(() => {
      if (!flying || !$droneStore.connected) return;
      
      let lr = 0, fb = 0, ud = 0, yaw = 0;
      
      if (pressedKeys.has('w') || pressedKeys.has('arrowup')) fb = speed;
      if (pressedKeys.has('s') || pressedKeys.has('arrowdown')) fb = -speed;
      if (pressedKeys.has('a')) lr = -speed;
      if (pressedKeys.has('d')) lr = speed;
      if (pressedKeys.has('arrowleft')) yaw = -speed;
      if (pressedKeys.has('arrowright')) yaw = speed;
      if (pressedKeys.has('q')) ud = speed;
      if (pressedKeys.has('e')) ud = -speed;
      
      if (lr !== 0 || fb !== 0 || ud !== 0 || yaw !== 0) {
        sendRC(lr, fb, ud, yaw).catch(() => {});
      }
    }, 100);
  }
  
  async function connect() {
    connecting = true;
    connectionStore.setStatus('connecting', 'Connecting...');
    
    try {
      const result: any = await invoke('connect_drone');
      
      if (result.success) {
        droneStore.setConnected(true);
        connectionStore.setStatus('connected', 'Connected to drone');
        toast.success('Connected to drone');
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      console.error('[DroneControl] Connection failed:', error);
      // Connection failed - ensure disconnected state
      droneStore.setConnected(false);
      droneStore.setFlying(false);
      droneStore.setVideoActive(false);
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
      console.error('[DroneControl] Disconnect failed:', error);
      toast.error('Disconnect failed: ' + error);
    }
  }
  
  async function takeoff() {
    if ($batteryWarning) {
      toast.error('Battery too low for takeoff');
      return;
    }
    
    takingOff = true;
    try {
      const result: any = await invoke('takeoff');
      
      if (result.success) {
        droneStore.setFlying(true);
        toast.success('Taking off...');
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      console.error('[DroneControl] Takeoff failed:', error);
      toast.error('Takeoff failed: ' + error);
    } finally {
      setTimeout(() => takingOff = false, 2000);
    }
  }
  
  async function land() {
    landing = true;
    try {
      const result: any = await invoke('land');
      
      if (result.success) {
        droneStore.setFlying(false);
        toast.success('Landing...');
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      console.error('[DroneControl] Land failed:', error);
      toast.error('Land failed: ' + error);
    } finally {
      setTimeout(() => landing = false, 2000);
    }
  }
  
  async function emergency() {
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
    } catch (error) {}
  }
  
  async function moveWithHold(x: number, y: number, z: number, yaw: number, duration: number = 500) {
    await sendRC(x, y, z, yaw);
    await new Promise(resolve => setTimeout(resolve, duration));
    await sendRC(0, 0, 0, 0);
  }
  
  function nudgeForward() { moveWithHold(0, speed, 0, 0); }
  function nudgeBack() { moveWithHold(0, -speed, 0, 0); }
  function nudgeLeft() { moveWithHold(-speed, 0, 0, 0); }
  function nudgeRight() { moveWithHold(speed, 0, 0, 0); }
  async function nudgeUp() { 
    try {
      await invoke('send_command', { command: 'up 20' });
      toast.info('Up 20cm');
    } catch (e) {
      moveWithHold(0, 0, speed, 0);
    }
  }
  async function nudgeDown() { 
    try {
      await invoke('send_command', { command: 'down 20' });
      toast.info('Down 20cm');
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
    <CardTitle>Flight Control</CardTitle>
  </CardHeader>
  <CardContent>
    <div class="space-y-4">
      {#if $batteryWarning}
        <div class="flex items-center gap-2 p-3 rounded-lg" style="background-color: rgba(239, 68, 68, 0.1); border: 1px solid rgba(239, 68, 68, 0.3)">
          <AlertTriangle class="h-4 w-4" style="color: #ef4444" />
          <span class="text-sm font-medium" style="color: #ef4444">Low Battery - Land Soon!</span>
        </div>
      {/if}

      {#if $droneStore.connected}
        <!-- Main Flight Controls -->
        <div class="flex items-center justify-center gap-2">
          <Button 
            on:click={takeoff} 
            variant="default"
            disabled={flying || takingOff}
            size="sm"
            style="min-width: 100px"
          >
            {#if takingOff}
              <Loader2 class="mr-2 h-4 w-4 animate-spin" />
              Taking Off...
            {:else}
              <Play class="mr-2 h-4 w-4" />
              Takeoff
            {/if}
          </Button>
          <Button 
            on:click={land} 
            variant="secondary"
            disabled={!flying || landing}
            size="sm"
            style="min-width: 100px"
          >
            {#if landing}
              <Loader2 class="mr-2 h-4 w-4 animate-spin" />
              Landing...
            {:else}
              <Square class="mr-2 h-4 w-4" />
              Land
            {/if}
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

        <!-- Control Pads -->
        <div class="grid grid-cols-3 gap-4 p-4 rounded-lg" style="background-color: var(--color-surface)">
          <!-- Movement Pad -->
          <div class="flex flex-col gap-1">
            <p class="text-xs font-semibold mb-1 text-center" style="color: var(--color-text-muted)">MOVE</p>
            <div class="grid grid-cols-3 gap-1">
              <div></div>
              <button 
                on:click={nudgeForward} 
                disabled={!flying}
                class="control-btn"
              >
                <ChevronUp class="h-5 w-5" />
              </button>
              <div></div>
              
              <button 
                on:click={nudgeLeft} 
                disabled={!flying}
                class="control-btn"
              >
                <ChevronLeft class="h-5 w-5" />
              </button>
              <button 
                on:click={stopMovement} 
                disabled={!flying}
                class="control-btn stop"
              >
                ■
              </button>
              <button 
                on:click={nudgeRight} 
                disabled={!flying}
                class="control-btn"
              >
                <ChevronRight class="h-5 w-5" />
              </button>
              
              <div></div>
              <button 
                on:click={nudgeBack} 
                disabled={!flying}
                class="control-btn"
              >
                <ChevronDown class="h-5 w-5" />
              </button>
              <div></div>
            </div>
          </div>

          <!-- Altitude Pad -->
          <div class="flex flex-col items-center gap-1">
            <p class="text-xs font-semibold mb-1" style="color: var(--color-text-muted)">ALTITUDE</p>
            <button 
              on:click={nudgeUp} 
              disabled={!flying}
              class="control-btn vertical"
            >
              <MoveUp class="h-5 w-5" />
            </button>
            <div class="text-xs font-mono" style="color: var(--color-text-muted); padding: 8px 0">20cm</div>
            <button 
              on:click={nudgeDown} 
              disabled={!flying}
              class="control-btn vertical"
            >
              <MoveDown class="h-5 w-5" />
            </button>
          </div>

          <!-- Rotation Pad -->
          <div class="flex flex-col items-center gap-1">
            <p class="text-xs font-semibold mb-1" style="color: var(--color-text-muted)">ROTATE</p>
            <button 
              on:click={rotateLeft} 
              disabled={!flying}
              class="control-btn vertical"
            >
              <RotateCcw class="h-5 w-5" />
            </button>
            <div class="text-xs font-mono" style="color: var(--color-text-muted); padding: 8px 0">YAW</div>
            <button 
              on:click={rotateRight} 
              disabled={!flying}
              class="control-btn vertical"
            >
              <RotateCw class="h-5 w-5" />
            </button>
          </div>
        </div>

        <!-- Keyboard Hint -->
        <div class="text-xs p-2 rounded text-center" style="background-color: var(--color-surface); color: var(--color-text-muted); border: 1px solid var(--color-border)">
          <p class="font-semibold mb-1">⌨️ Keyboard: WASD/Arrows • Q/E: Alt • Space: Emergency</p>
        </div>
      {:else}
        <div class="text-center py-8">
          <p class="text-sm mb-4" style="color: var(--color-text-muted)">Connect to drone to access controls</p>
          <Button on:click={connect} size="lg" disabled={connecting}>
            {#if connecting}
              <Loader2 class="mr-2 h-5 w-5 animate-spin" />
              Connecting...
            {:else}
              <Play class="mr-2 h-5 w-5" />
              Connect
            {/if}
          </Button>
        </div>
      {/if}
    </div>
  </CardContent>
</Card>

<style>
  .control-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 12px;
    border-radius: 8px;
    border: 2px solid var(--color-border);
    background: transparent;
    color: var(--color-text);
    cursor: pointer;
    transition: all 0.2s;
    aspect-ratio: 1;
  }

  .control-btn:not(:disabled):hover {
    background-color: var(--color-primary);
    border-color: var(--color-primary);
    color: white;
    transform: scale(1.05);
  }

  .control-btn:not(:disabled):active {
    transform: scale(0.95);
  }

  .control-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .control-btn.stop {
    font-size: 24px;
    font-weight: bold;
  }

  .control-btn.vertical {
    width: 80px;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>
