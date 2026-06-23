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
    padding: 0.5rem;
    &:enabled {
      box-shadow: var(--shadow-elevation-medium);
      color: var(--text-in-primary);
    }
    &:active {
      box-shadow: none;
    }
  }

  /* Too wide for all buttons in one row */
  @container main (width < 73rem) {
    .create-btn {
      grid-column: 1 / -1;
      justify-self: center;
    }
  }
  /* Too wide for three items in top row */
  @container main (width < 56rem) {
    .create-btn {
      grid-column: 2 / -1;
      justify-self: start;
    }
  }
</style>
