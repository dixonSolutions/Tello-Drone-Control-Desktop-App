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
    ArrowRight 
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
    <CardTitle class="flex items-center gap-2">
      <Sparkles class="h-5 w-5" />
      Tricks & Flips
    </CardTitle>
  </CardHeader>
  <CardContent>
    <div class="space-y-4">
      {#if $droneStore.connected && $droneStore.flying}
        <div class="grid grid-cols-3 gap-2">
          <!-- Forward Flip -->
          <div class="col-start-2">
            <Button 
              on:click={() => performFlip('f')}
              variant="outline"
              class="w-full theme-primary"
              style="background-color: var(--color-primary); color: var(--color-text)"
            >
              <ArrowUp class="h-5 w-5" />
            </Button>
          </div>
          
          <!-- Left Flip -->
          <div class="col-start-1 row-start-2">
            <Button 
              on:click={() => performFlip('l')}
              variant="outline"
              class="w-full theme-primary"
              style="background-color: var(--color-primary); color: var(--color-text)"
            >
              <ArrowLeft class="h-5 w-5" />
            </Button>
          </div>
          
          <!-- Center Label -->
          <div class="col-start-2 row-start-2 flex items-center justify-center">
            <span class="text-sm font-semibold theme-text-muted">FLIP</span>
          </div>
          
          <!-- Right Flip -->
          <div class="col-start-3 row-start-2">
            <Button 
              on:click={() => performFlip('r')}
              variant="outline"
              class="w-full theme-primary"
              style="background-color: var(--color-primary); color: var(--color-text)"
            >
              <ArrowRight class="h-5 w-5" />
            </Button>
          </div>
          
          <!-- Back Flip -->
          <div class="col-start-2 row-start-3">
            <Button 
              on:click={() => performFlip('b')}
              variant="outline"
              class="w-full theme-primary"
              style="background-color: var(--color-primary); color: var(--color-text)"
            >
              <ArrowDown class="h-5 w-5" />
            </Button>
          </div>
        </div>
        
        <div class="p-3 rounded-md text-sm" style="background-color: var(--color-warning); color: var(--color-text)">
          <p class="font-semibold mb-1">Warning:</p>
          <p>Ensure adequate space and battery before performing tricks</p>
        </div>
      {:else if $droneStore.connected}
        <div class="text-center py-8 theme-text-muted">
          <Sparkles class="h-12 w-12 mx-auto mb-3 opacity-50" />
          <p>Drone must be flying to perform tricks</p>
        </div>
      {:else}
        <div class="text-center py-8 theme-text-muted">
          <p>Connect to drone and takeoff to perform tricks</p>
        </div>
      {/if}
    </div>
  </CardContent>
</Card>

