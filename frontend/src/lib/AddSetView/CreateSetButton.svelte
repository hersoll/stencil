<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { setState } from '$src/globalStates.svelte';
  import { defaultSetOptions } from '$src/types';

  function submitSet() {
    setState.addedSets.push({
      id: setState.setCount,
      set: {
        problems: structuredClone($state.snapshot(setState.pendingSet)),
        options: defaultSetOptions
      }
    });
    setState.setCount += 1;
  }
</script>

<button
  class="primary create-btn"
  disabled={setState.pendingSet.topics.length == 0}
  onclick={submitSet}>{i18n.t('add_set')}</button
>

<style>
  .create-btn {
    font-size: 1.1rem;
    width: 15rem;
    padding: 0.5rem 0;
    box-shadow: var(--shadow-elevation-medium);
  }
</style>
