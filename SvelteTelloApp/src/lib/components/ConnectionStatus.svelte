<script lang="ts">
  import { droneStore, connectionStore } from '$lib/stores/drone';
  import { Signal, SignalHigh, SignalLow, SignalZero } from 'lucide-svelte';
  
  let statusColor = 'text-muted-foreground';
  
  $: {
    if ($connectionStore.status === 'connected') {
      statusColor = 'text-success';
    } else if ($connectionStore.status === 'connecting') {
      statusColor = 'text-info';
    } else if ($connectionStore.status === 'error') {
      statusColor = 'text-error';
    } else {
      statusColor = 'theme-text-muted';
    }
  }
</script>

<div class="flex items-center gap-2">
  {#if $connectionStore.status === 'connected'}
    <Signal class="h-4 w-4 {statusColor}" />
  {:else if $connectionStore.status === 'connecting'}
    <Signal class="h-4 w-4 {statusColor} animate-pulse" />
  {:else}
    <SignalZero class="h-4 w-4 {statusColor}" />
  {/if}
  <span class="text-sm font-medium {statusColor}">
    {$connectionStore.message}
  </span>
</div>

