<script lang="ts">
  import type { HTMLAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  type $$Props = HTMLAttributes<HTMLDivElement> & {
    class?: string;
  };

  let className: string = '';
  export { className as class };
</script>

<div class={cn('flex flex-col space-y-1.5 p-6', className)} {...$$restProps}>
  <slot />
</div>

