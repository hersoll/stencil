<script lang="ts">
  import type { ProblemSetSpec } from '../types';
  let { set }: { set: ProblemSetSpec } = $props();

  const MIN = 1;
  const MAX = 250;

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
  id="number_picker"
  type="number"
  bind:value={set.n}
  min="1"
  max="250"
  onblur={handleBlur}
/>

<style>
</style>
