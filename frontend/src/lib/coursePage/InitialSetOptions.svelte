<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import DifficultySelector from './DifficultySelector.svelte';
  import { sets } from '$src/states.svelte';

  const MIN_PROBLEMS = 1;
  const MAX_PROBLEMS = 250;

  function handleBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      sets.current_set.problems.n = Math.max(
        MIN_PROBLEMS,
        Math.min(MAX_PROBLEMS, value)
      );
    } else {
      sets.current_set.problems.n = MIN_PROBLEMS;
    }
  }
</script>

<div class="options">
  <div class="n-container">
    <label for="n">{i18n.t('pick_number')}:</label>
    <input
      name="n"
      class="number-picker"
      type="number"
      bind:value={sets.current_set.problems.n}
      min="1"
      max="250"
      onblur={handleBlur}
    />
  </div>
  <div class="difficulty-container">
    <label for="difficulty-row">{i18n.t('difficulty')}:</label>
    <div class="difficulty-row">
      {i18n.t('from')}
      <DifficultySelector set={sets.current_set.problems} type="starting" />
      {i18n.t('to')}
      <DifficultySelector set={sets.current_set.problems} type="ending" />
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
    margin-top: 2rem;
    margin-bottom: 2rem;
  }
  label {
    font-size: 1.5rem;
    font-weight: 600;
  }

  @media (max-width: 75rem) {
    .options {
      flex-direction: column;
      gap: 1rem;
    }
  }
  .n-container,
  .difficulty-container {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .number-picker,
  .difficulty-row {
    border-radius: 0.5rem;
    border: 1px solid light-dark(var(--bg-dark), var(--bg));
    background-color: var(--bg-light);
    box-shadow: var(--shadow-elevation-low);
    padding: 0.5rem;
    font-size: 1.1rem;
  }

  .difficulty-row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .number-picker {
    width: 5rem;
    line-height: 1.5rem;
  }
</style>
