<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { sets, set_id } from '$src/states.svelte';

  function submitSet() {
    sets.set_states.push({
      id: set_id.count,
      set: structuredClone($state.snapshot(sets.current_set))
    });
    set_id.count += 1;
  }
</script>

<button
  class="primary create-btn"
  disabled={sets.current_set.problems.topics.length == 0}
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
