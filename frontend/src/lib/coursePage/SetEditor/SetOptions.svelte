<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import DifficultySelector from '../inputComponents/DifficultySelector.svelte';
  import ProblemNumberInput from '../inputComponents/ProblemNumberInput.svelte';
  import type { ProblemSetSpec } from '../types';

  let { set }: { set: ProblemSetSpec } = $props();
</script>

<div class="options-container">
  <h2>{i18n.t('options')}</h2>
  <div class="label-div">
    <label for="n">{i18n.t("pick_number")}</label>
    <ProblemNumberInput {set} />
  </div>
  <div class="label-div">
    <label for="difficulty">{i18n.t("difficulty")}</label>
    <DifficultySelector {set} type="starting" />
    <DifficultySelector {set} type="ending" />
  </div>
  <div class="label-div">
    <label for="heading">{i18n.t('set_option_heading')}</label>
    <input
      type="text"
      value={set.options.heading}
      id="heading"
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
    />
  </div>
  <div class="label-div">
    <label for="spacing">{i18n.t('set_option_spacing')}</label>
    <input
      type="number"
      id="spacing"
      bind:value={set.options.spacing}
      onchange={e => (set.options.spacing = e.currentTarget.valueAsNumber)}
    />
  </div>
</div>

<style>
  .options-container {
    background-color: var(--bg-light);
    border-radius: 1rem;
    box-shadow: var(--shadow-elevation-medium);
    height: min-content;
    width: 20rem;
    padding: 1rem;
  }

  .label-div {
    display: flex;
    flex-direction: column;
  }

  h2 {
    margin-top: -0.25rem;
    margin-bottom: 0.5rem;
  }
</style>
