<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import DifficultySelector from './inputComponents/DifficultySelector.svelte';
  import { sets, set_id } from '$src/states.svelte';
  import ProblemNumberInput from './inputComponents/ProblemNumberInput.svelte';

  function submitSet() {
    sets.set_states.push({
      id: set_id.count,
      set: structuredClone($state.snapshot(sets.current_set))
    });
    set_id.count += 1;
  }
</script>

<div class="options">
  <h2>{i18n.t('options')}</h2>
  <ProblemNumberInput set={sets.current_set} />
  <div class="difficulty-container">
    <label for="">{i18n.t('difficulty')}:</label>
    <br />
    <div class="difficulty-row">
      <DifficultySelector set={sets.current_set} type="starting" />
      {i18n.t('to')}
      <DifficultySelector set={sets.current_set} type="ending" />
    </div>
  </div>
  <button
    class="primary create-btn"
    disabled={sets.current_set.topics.length == 0}
    onclick={submitSet}>{i18n.t('create_set')}</button
  >
</div>

<style>
  .options {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  h2 {
    margin: 0;
  }

  .difficulty-row {
    margin-top: 0.2rem;
    display: flex;
    gap: 0.35rem;
    align-items: center;
  }

  .create-btn {
    box-shadow: var(--shadow-elevation-medium);
  }
</style>
