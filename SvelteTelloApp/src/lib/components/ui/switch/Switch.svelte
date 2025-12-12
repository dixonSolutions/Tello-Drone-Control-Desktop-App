<script lang="ts">
  export let checked = false;
  export let disabled = false;
  export let loading = false;
  export let onCheckedChange: ((checked: boolean) => void) | undefined = undefined;
  
  function handleClick() {
    if (disabled || loading) return;
    checked = !checked;
    if (onCheckedChange) {
      onCheckedChange(checked);
    }
  }
</script>

<button
  type="button"
  role="switch"
  aria-checked={checked}
  {disabled}
  on:click={handleClick}
  class="switch-root {checked ? 'checked' : ''} {disabled || loading ? 'disabled' : ''}"
  style="
    display: inline-flex;
    align-items: center;
    width: 44px;
    height: 24px;
    border-radius: 12px;
    padding: 0;
    border: none;
    cursor: {disabled || loading ? 'not-allowed' : 'pointer'};
    background-color: {checked ? 'var(--color-primary)' : 'var(--color-border)'};
    opacity: {disabled || loading ? '0.5' : '1'};
    transition: background-color 0.2s ease;
    position: relative;
  "
>
  <span
    class="switch-thumb {loading ? 'loading' : ''}"
    style="
      display: block;
      width: 20px;
      height: 20px;
      background-color: white;
      border-radius: 50%;
      transition: transform 0.2s ease;
      transform: translateX({checked ? '22px' : '2px'});
      box-shadow: 0 2px 4px rgba(0,0,0,0.2);
    "
  >
    {#if loading}
      <span class="loader" style="
        position: absolute;
        inset: 0;
        border: 2px solid #f3f3f3;
        border-top: 2px solid var(--color-primary);
        border-radius: 50%;
        animation: spin 1s linear infinite;
      "></span>
    {/if}
  </span>
</button>

<style>
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
</style>

