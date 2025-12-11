<script lang="ts">
  export let lines: number = 3;
  export let height: 'sm' | 'md' | 'lg' = 'md';
  
  const heights = {
    sm: 'h-4',
    md: 'h-8',
    lg: 'h-12'
  };
</script>

<div class="space-y-3 animate-pulse">
  {#each Array(lines) as _, i}
    <div class="rounded {heights[height]} theme-surface opacity-50"></div>
  {/each}
</div>

<style>
  @keyframes pulse {
    0%, 100% {
      opacity: 0.3;
    }
    50% {
      opacity: 0.5;
    }
  }
  
  .animate-pulse {
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
</style>

