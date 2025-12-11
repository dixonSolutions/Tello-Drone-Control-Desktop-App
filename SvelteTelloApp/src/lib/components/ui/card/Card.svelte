<script lang="ts">
  import type { HTMLAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  type $$Props = HTMLAttributes<HTMLDivElement> & {
    class?: string;
  };

  let className: string = '';
  export { className as class };
</script>

<div
  class={cn('rounded-lg border shadow-sm', className)}
  style="background-color: var(--color-surface); border-color: var(--color-border); color: var(--color-text)"
  {...$$restProps}
>
  <slot />
</div>
