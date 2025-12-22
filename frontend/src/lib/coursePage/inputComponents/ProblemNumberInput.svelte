<script lang="ts">
  import type { ProblemSetSpec } from '../types';
  import i18n from '$src/i18n.svelte';

  let { set }: { set: ProblemSetSpec } = $props();
  const MIN = 1;
  const MAX = 250;

  function handleBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      set.n = Math.max(MIN, Math.min(MAX, value));
    } else {
      set.n = 10;
    }
  }
</script>

<div class="n-container">
  <label for="n">{i18n.t('pick_number')}</label>
  <input
    name="n"
    type="number"
    bind:value={set.n}
    min="1"
    max="250"
    onblur={handleBlur}
  />
</div>

<style>
  .n-container {
    display: grid;

    & input {
      width: 50%;
      border: none;
      border-radius: 0.5rem;
      font-size: 1rem;
      line-height: 1.5rem;
      padding-left: 0.5rem;
    }
  }
</style>
