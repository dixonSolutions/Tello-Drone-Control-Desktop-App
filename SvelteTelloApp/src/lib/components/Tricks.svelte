<script lang="ts">
  import { droneStore } from '$lib/stores/drone';
  import Card from './ui/card/Card.svelte';
  import CardHeader from './ui/card/CardHeader.svelte';
  import CardTitle from './ui/card/CardTitle.svelte';
  import CardContent from './ui/card/CardContent.svelte';
  import Button from './ui/button/Button.svelte';
  import { 
    Sparkles, 
    ArrowUp, 
    ArrowDown, 
    ArrowLeft, 
    ArrowRight,
    Lock
  } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { toast } from 'svelte-sonner';
  
  async function performFlip(direction: 'f' | 'b' | 'l' | 'r') {
    if (!$droneStore.flying) {
      toast.error('Drone must be flying to perform tricks');
      return;
    }
    
    if ($droneStore.battery < 30) {
      toast.error('Battery too low for tricks');
      return;
    }
    
    try {
      const result: any = await invoke('flip', { direction });
      if (result.success) {
        toast.success(`Flip executed!`);
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      toast.error('Flip failed: ' + error);
    }
  }
</script>

<Card>
  <CardHeader>
    <div class="flex items-center justify-between">
      <CardTitle class="flex items-center gap-2">
        <Sparkles class="h-5 w-5" />
        Tricks & Flips
      </CardTitle>
      {#if !$droneStore.flying}
        <Lock class="h-4 w-4" style="color: var(--color-text-muted)" />
      {/if}
    </div>
  </CardHeader>
  <CardContent>
    <div class="space-y-4">
      <!-- Always show controls, just disable them -->
      <div class="grid grid-cols-3 gap-2">
        <!-- Forward Flip -->
        <div class="col-start-2">
          <button
            on:click={() => performFlip('f')}
            disabled={!$droneStore.flying}
            class="flip-btn"
            style="opacity: {$droneStore.flying ? '1' : '0.3'}"
          >
            <ArrowUp class="h-5 w-5" />
          </button>
        </div>
        
        <!-- Left Flip -->
        <div class="col-start-1 row-start-2">
          <button
            on:click={() => performFlip('l')}
            disabled={!$droneStore.flying}
            class="flip-btn"
            style="opacity: {$droneStore.flying ? '1' : '0.3'}"
          >
            <ArrowLeft class="h-5 w-5" />
          </button>
        </div>
        
        <!-- Center Label -->
        <div class="col-start-2 row-start-2 flex items-center justify-center">
          <span class="text-sm font-bold" style="color: var(--color-text-muted)">FLIP</span>
        </div>
        
        <!-- Right Flip -->
        <div class="col-start-3 row-start-2">
          <button
            on:click={() => performFlip('r')}
            disabled={!$droneStore.flying}
            class="flip-btn"
            style="opacity: {$droneStore.flying ? '1' : '0.3'}"
          >
            <ArrowRight class="h-5 w-5" />
          </button>
        </div>
        
        <!-- Back Flip -->
        <div class="col-start-2 row-start-3">
          <button
            on:click={() => performFlip('b')}
            disabled={!$droneStore.flying}
            class="flip-btn"
            style="opacity: {$droneStore.flying ? '1' : '0.3'}"
          >
            <ArrowDown class="h-5 w-5" />
          </button>
        </div>
      </div>
      
      {#if !$droneStore.connected}
        <div class="text-center p-3 rounded-lg" style="background-color: var(--color-surface); border: 1px dashed var(--color-border); color: var(--color-text-muted)">
          <Lock class="h-8 w-8 mx-auto mb-2 opacity-30" />
          <p class="text-sm font-medium">Not Connected</p>
          <p class="text-xs">Connect to drone first</p>
        </div>
      {:else if !$droneStore.flying}
        <div class="text-center p-3 rounded-lg" style="background-color: var(--color-surface); border: 1px dashed var(--color-border); color: var(--color-text-muted)">
          <Sparkles class="h-8 w-8 mx-auto mb-2 opacity-30" />
          <p class="text-sm font-medium">Drone Not Flying</p>
          <p class="text-xs">Takeoff required to perform tricks</p>
        </div>
      {:else}
        <div class="p-3 rounded-lg text-xs" style="background-color: rgba(245, 158, 11, 0.1); border: 1px solid rgba(245, 158, 11, 0.3); color: var(--color-text)">
          <p class="font-semibold mb-1">⚠️ Warning:</p>
          <ul class="list-disc list-inside space-y-1" style="color: var(--color-text-muted)">
            <li>Ensure adequate space (2m+ clearance)</li>
            <li>Battery should be >30%</li>
            <li>Stable flight before attempting</li>
          </ul>
        </div>
      {/if}
    </div>
  </CardContent>
</Card>

<style>
  .flip-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    aspect-ratio: 1;
    padding: 16px;
    border-radius: 8px;
    border: 2px solid var(--color-border);
    background-color: var(--color-primary);
    color: white;
    cursor: pointer;
    transition: all 0.2s;
  }

  .flip-btn:not(:disabled):hover {
    transform: scale(1.05);
    box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  }

  .flip-btn:not(:disabled):active {
    transform: scale(0.95);
  }

  .flip-btn:disabled {
    cursor: not-allowed;
  }
</style>
