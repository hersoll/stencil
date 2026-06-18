<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import DifficultySelector from '../../AddSetView/DifficultySelector.svelte';
  import type { ProblemSetSpec } from '$src/types.ts';
  let { set = $bindable() }: { set: ProblemSetSpec } = $props();

  const MIN_PROBLEMS = 1;
  const MAX_PROBLEMS = 250;

  function handleNumberBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      set.problems.n = Math.max(MIN_PROBLEMS, Math.min(MAX_PROBLEMS, value));
    } else {
      set.problems.n = MIN_PROBLEMS;
    }
  }

  const MIN_COLUMNS = 1;
  const MAX_COLUMNS = 8;
  function handleColumnsBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      set.options.question_columns = Math.max(
        MIN_COLUMNS,
        Math.min(MAX_COLUMNS, value)
      );
    } else {
      set.options.question_columns = MIN_COLUMNS;
    }
  }

  const MIN_SPACING = 0;
  const MAX_SPACING = 300;
  function handleSpacingBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      set.options.spacing = Math.max(MIN_SPACING, Math.min(MAX_SPACING, value));
    } else {
      set.options.spacing = null;
    }
  }
</script>

<div class="options-container">
  <h2>{i18n.t('options')}</h2>
  <div class="flex">
    <div class="col">
      <div class="label-div">
        <label for="n">{i18n.t('pick_number')}</label>
        <input
          name="n"
          id="number_picker"
          type="number"
          bind:value={set.problems.n}
          min="1"
          max="250"
          onblur={handleNumberBlur}
        />
      </div>
      <div class="label-div">
        <label for="difficulty">{i18n.t('difficulty')}</label>
        <div class="difficulty-options">
          <p>{i18n.t('from')}</p>
          <DifficultySelector set={set.problems} type="starting" />
          <p>{i18n.t('to')}</p>
          <DifficultySelector set={set.problems} type="ending" />
        </div>
      </div>
    </div>
    <div class="col">
      <div class="label-div">
        <label for="heading">{i18n.t('set_option_heading')}</label>
        <input
          type="text"
          value={set.options.heading}
          id="heading"
          class="text-input"
          placeholder="Lös uppgifterna"
          onchange={e => (set.options.heading = e.currentTarget.value)}
        />
      </div>
      <div class="label-div">
        <label for="columns"> {i18n.t('set_option_columns')}</label>
        <input
          type="number"
          id="columns"
          bind:value={set.options.question_columns}
          onchange={e =>
            (set.options.question_columns = e.currentTarget.valueAsNumber)}
          onblur={handleColumnsBlur}
        />
      </div>
      <div class="label-div">
        <label for="spacing">{i18n.t('set_option_spacing')}</label>
        <input
          type="number"
          id="spacing"
          bind:value={set.options.spacing}
          onchange={e => (set.options.spacing = e.currentTarget.valueAsNumber)}
          onblur={handleSpacingBlur}
        />
      </div>
      <div class="label-div pagebreak">
        <label for="pagebreak">{i18n.t('set_option_pagebreak')}</label>
        <input
          type="checkbox"
          id="pagebreak"
          checked={set.options.pagebreak_after}
          onchange={e =>
            (set.options.pagebreak_after = e.currentTarget.checked)}
        />
      </div>
    </div>
  </div>
</div>

<style>
  .options-container {
    background-color: var(--bg-light);
    border-radius: 1rem;
    box-shadow: var(--shadow-elevation-medium);
    height: min-content;
    padding: 1rem;
  }
  .flex {
    display: flex;
    flex-direction: column;
  }

  .label-div {
    font-size: 1.1rem;
    font-weight: 500;
    display: flex;
    flex-direction: column;
    margin-bottom: 0.5rem;

    input {
      width: 4.5rem;
      font-size: 1rem;
      border-radius: 0.5rem;
      border: none;
      border: 1px solid var(--bg-dark);
      box-shadow: var(--shadow-elevation-low);
      padding: 0.5rem;
    }

    .text-input {
      width: 15rem;
    }

    &.pagebreak {
      flex-direction: row;
      align-items: center;
      gap: 0.5rem;
    }
  }

  .difficulty-options {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
    padding: 0.5rem;
    width: min-content;
    border-radius: 0.5rem;
    border: none;
    border: 1px solid var(--bg-dark);
    box-shadow: var(--shadow-elevation-low);

    p {
      font-weight: 400;
      font-size: 1rem;
    }
  }

  h2 {
    margin-top: -0.25rem;
    margin-bottom: 0.5rem;
  }

  @media (max-width: 75rem) and (min-width: 50rem) {
    .flex {
      flex-direction: row;
      gap: 5rem;
    }
  }
</style>
