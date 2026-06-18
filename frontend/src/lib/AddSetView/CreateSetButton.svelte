<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { set_states, set_id } from '$src/globalStates.svelte';
  import { defaultSetOptions } from '$src/types';

  function submitSet() {
    set_states.added_sets.push({
      id: set_id.count,
      set: {
        problems: structuredClone($state.snapshot(set_states.pending_set)),
        options: defaultSetOptions
      }
    });
    set_id.count += 1;
  }
</script>

<button
  class="primary create-btn"
  disabled={set_states.pending_set.topics.length == 0}
  onclick={submitSet}>{i18n.t('add_set')}</button
>

<style>
  .create-btn {
    font-size: 1.2rem;
    width: 20rem;
    height: 3rem;
    box-shadow: var(--shadow-elevation-medium);
  }
</style>
