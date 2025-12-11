<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils.js';

  type $$Props = HTMLInputAttributes;

  let className: $$Props['class'] = undefined;
  export let value: $$Props['value'] = undefined;
  export { className as class };
</script>

<input
  class={cn(
    'flex h-10 w-full rounded-md border px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:opacity-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50',
    className
  )}
  style="background-color: var(--color-surface); border-color: var(--color-border); color: var(--color-text)"
  bind:value
  {...$$restProps}
  on:blur
  on:change
  on:click
  on:focus
  on:focusin
  on:focusout
  on:keydown
  on:keypress
  on:keyup
  on:mouseover
  on:mouseenter
  on:mouseleave
  on:paste
  on:input
/>
