<script lang="ts">
  import type { ProblemSetSpec } from '../types';
  let { set }: { set: ProblemSetSpec } = $props();

  const MIN = 0;
  const MAX = 10;
  function handleBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      set.n = Math.max(MIN, Math.min(MAX, value));
    } else {
      set.n = MIN;
    }
  }
</script>

<input
  name="n"
  type="number"
  bind:value={set.n}
  min="1"
  max="250"
  onblur={handleBlur}
/>

<style>
  input {
    border: none;
    border-radius: 0.5rem;
    font-size: 1.2rem;
    line-height: 1.5rem;
    padding-left: 0.5rem;
  }
</style>
