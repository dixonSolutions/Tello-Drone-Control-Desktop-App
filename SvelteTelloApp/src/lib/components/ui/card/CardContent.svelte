<script lang="ts">
  import type { HTMLAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  type $$Props = HTMLAttributes<HTMLDivElement> & {
    class?: string;
  };

  let className: string = '';
  export { className as class };
</script>

<div class={cn('p-6 pt-0', className)} {...$$restProps}>
  <slot />
</div>

