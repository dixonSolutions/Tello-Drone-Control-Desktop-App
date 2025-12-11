<script lang="ts">
  import type { HTMLLabelAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  type $$Props = HTMLLabelAttributes & {
    class?: string;
  };

  let className: string = '';
  export { className as class };
</script>

<label
  class={cn(
    'text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70',
    className
  )}
  {...$$restProps}
>
  <slot />
</label>

