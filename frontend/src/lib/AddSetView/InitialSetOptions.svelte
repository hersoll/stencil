<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import DifficultySelector from './DifficultySelector.svelte';
  import { set_states } from '$src/globalStates.svelte';

  const MIN_PROBLEMS = 1;
  const MAX_PROBLEMS = 250;

  function handleBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      set_states.pending_set.n = Math.max(
        MIN_PROBLEMS,
        Math.min(MAX_PROBLEMS, value)
      );
    } else {
      set_states.pending_set.n = MIN_PROBLEMS;
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
      bind:value={set_states.pending_set.n}
      min="1"
      max="250"
      onblur={handleBlur}
    />
  </div>
  <div class="difficulty-container">
    <label for="difficulty-row">{i18n.t('difficulty')}:</label>
    <div class="difficulty-row">
      {i18n.t('from')}
      <DifficultySelector set={set_states.pending_set} type="starting" />
      {i18n.t('to')}
      <DifficultySelector set={set_states.pending_set} type="ending" />
    </div>
  </div>
</div>

<style>
  .options {
    display: flex;
    gap: 3rem;
    align-items: center;
  }
  label {
    font-size: 1.2rem;
    font-weight: 600;
  }

  .n-container,
  .difficulty-container {
    display: flex;
    align-items: center;
    gap: 1rem;
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
