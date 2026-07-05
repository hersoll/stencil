<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import DifficultySelector from './DifficultySelector.svelte';
  import type { ProblemOptions } from '$src/types';

  let { problemOptions = $bindable() }: { problemOptions: ProblemOptions } =
    $props();

  const MIN_PROBLEMS = 1;
  const MAX_PROBLEMS = 250;

  function handleBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      problemOptions.n = Math.max(MIN_PROBLEMS, Math.min(MAX_PROBLEMS, value));
    } else {
      problemOptions.n = MIN_PROBLEMS;
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
      value={problemOptions.n}
      min="1"
      max="250"
      onblur={handleBlur}
      oninput={e => (problemOptions.n = e.currentTarget.valueAsNumber)}
    />
  </div>
  <div class="difficulty-container">
    <label for="difficulty-row">{i18n.t('difficulty')}:</label>
    <div class="difficulty-row">
      {i18n.t('from')}
      <DifficultySelector bind:problemOptions type="starting" fontSize={1} />
      {i18n.t('to')}
      <DifficultySelector bind:problemOptions type="ending" fontSize={1} />
    </div>
  </div>
</div>

<style>
  .options {
    display: flex;
    width: 100%;
    gap: 3rem;
    align-items: center;
  }
  label {
    font-size: 1.2rem;
    font-weight: 600;
    white-space: nowrap;
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
  @container main (width < 73rem) {
    .options {
      width: fit-content;
    }
  }
  /* Too wide for three items in top row */
  @container main (width < 56rem) {
    .options {
      grid-column: 1 / -1;
      justify-self: center;
    }
  }

  /* Too wide for three items in top row */
  @container main (width < 47rem) {
    .options {
      gap: 1rem;
    }
  }
</style>
