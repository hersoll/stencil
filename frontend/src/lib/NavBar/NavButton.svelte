<script lang="ts">
  let { children, ...props } = $props();
</script>

<button type="button" {...props}>
  {@render children()}
</button>

<style>
  button {
    position: relative;
    flex: 0 0 auto;
    display: flex;
    gap: 0.4rem;
    padding: 0.6rem 0.5rem;
    align-items: center;
    background-color: var(--bg);
    border: none;
    border-radius: 1rem;

    &.selected,
    &:hover {
      &:enabled {
        background-color: var(--bg-dark);
      }
    }
    &.nav-closed {
      border-radius: 0.5rem;
    }

    &:disabled {
      color: gray;
    }
    &:disabled :global(p) {
      color: inherit;
    }
    &:disabled :global(img) {
      color: inherit;
    }
  }
</style>
