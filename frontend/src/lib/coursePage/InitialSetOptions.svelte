<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import DifficultySelector from './inputComponents/DifficultySelector.svelte';
  import { sets } from '$src/states.svelte';
  import ProblemNumberInput from './inputComponents/ProblemNumberInput.svelte';
</script>

<div class="options">
  <div class="n-container">
    <label for="n">{i18n.t('pick_number')}:</label>
    <ProblemNumberInput set={sets.current_set} />
  </div>
  <div class="difficulty-container">
    <label for="difficulty-row">{i18n.t('difficulty')}:</label>
    <div class="difficulty-row">
      <DifficultySelector set={sets.current_set} type="starting" />
      {i18n.t('to')}
      <DifficultySelector set={sets.current_set} type="ending" />
    </div>
  </div>
</div>

<style>
  .options {
    background-color: var(--bg-light);
    border-radius: 1rem;
    box-shadow: var(--shadow-elevation-medium);
    padding: 1rem;
    display: flex;
    gap: 4rem;
    justify-content: center;
    margin-top: 4rem;
    margin-bottom: 2rem;
  }
  label {
    font-size: 1.5rem;
    font-weight: 600;
  }
  .n-container,
  .difficulty-container {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .number_picker {
    border: none;
    border-radius: 0.5rem;
    font-size: 1.2rem;
    line-height: 1.5rem;
    padding-left: 0.5rem;
  }


  .difficulty-row {
    margin-top: 0.2rem;
    display: flex;
    gap: 0.35rem;
    align-items: center;
  }
</style>
