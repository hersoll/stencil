<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { setIDsWithHeader } from './layoutStates.svelte';
  import { type SetState } from '$src/types';
  import DifficultySelector from '../AddSetView/DifficultySelector.svelte';

  let { set = $bindable() }: { set: SetState } = $props();

  function handleNBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      set.options.problem_options.n = Math.max(1, Math.min(250, value));
    } else {
      set.options.problem_options.n = 1;
    }
  }

  const MIN_SPACING = 0;
  const MAX_SPACING = 120;
  function handleSpacingBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      set.options.set_options.spacing = Math.max(
        MIN_SPACING,
        Math.min(MAX_SPACING, value)
      );
    } else {
      set.options.set_options.spacing = null;
    }
  }

  const MIN_COLUMNS = 1;
  const MAX_COLUMNS = 5;
  function handleColumnsBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      set.options.set_options.questionColumns = Math.max(
        MIN_COLUMNS,
        Math.min(MAX_COLUMNS, value)
      );
    } else {
      set.options.set_options.questionColumns = MIN_COLUMNS;
    }
  }

  function checkHeaderBox() {
    if (!setIDsWithHeader.includes(set.id)) setIDsWithHeader.push(set.id);
  }
  function uncheckHeaderBox() {
    setIDsWithHeader.splice(setIDsWithHeader.indexOf(set.id), 1);
    set.options.set_options.heading = '';
  }
</script>

<div class="set-options" style="position-anchor: --set-{set.id}">
  <div class="heading-grid">
    <div class="topic-titles">
      {#if set.topics.length <= 3}
        {#each set.topics as topic}
          <h2 class={set.topics.length == 1 ? 'big-text' : ''}>{topic.desc}</h2>
        {/each}
      {:else}
        <h2>{set.topics[0].desc} + {set.topics.length - 1}</h2>
      {/if}
    </div>
    <div class="problem-options">
      <div class="label-div">
        <label for="n">Uppgifter:</label>
        <input
          id="n"
          class="number-picker"
          type="number"
          value={set.options.problem_options.n}
          min="1"
          max="250"
          onblur={handleNBlur}
          oninput={e =>
            (set.options.problem_options.n = e.currentTarget.valueAsNumber)}
        />
      </div>
      <div class="difficulty-container">
        <DifficultySelector
          type="starting"
          bind:problemOptions={set.options.problem_options}
          fontSize={0.9}
        />
        <p style="display: inline;">-</p>
        <DifficultySelector
          type="ending"
          bind:problemOptions={set.options.problem_options}
          fontSize={0.9}
        />
      </div>
    </div>
  </div>
  <div class="options-grid">
    <div class="label-div">
      <label for="pagebreak-after">{i18n.t('set_option_pagebreak')}</label>
      <input
        name="pagebreak-after"
        type="checkbox"
        checked={set.options.set_options.pagebreakAfter}
        onchange={e =>
          (set.options.set_options.pagebreakAfter = e.currentTarget.checked)}
      />
    </div>
    <div class="label-div">
      <label for="spacing">{i18n.t('set_option_spacing')}</label>
      <input
        type="number"
        class="spacing-input"
        id="spacing"
        value={set.options.set_options.spacing}
        min="0"
        max="120"
        oninput={e =>
          (set.options.set_options.spacing =
            e.currentTarget.value == '' ? null : e.currentTarget.valueAsNumber)}
        onblur={handleSpacingBlur}
      />
    </div>
    <div class="label-div">
      <label for="heading">{i18n.t('set_option_heading')}</label>
      <input
        type="checkbox"
        id="heading"
        checked={setIDsWithHeader.includes(set.id)}
        onchange={e =>
          e.currentTarget.checked ? checkHeaderBox() : uncheckHeaderBox()}
      />
    </div>
    <div class="label-div">
      <label for="columns"> {i18n.t('set_option_columns')}</label>
      <input
        type="number"
        id="columns"
        min="1"
        max="5"
        value={set.options.set_options.questionColumns}
        onchange={e =>
          (set.options.set_options.questionColumns =
            e.currentTarget.valueAsNumber)}
        onblur={handleColumnsBlur}
      />
    </div>
  </div>
</div>
<div class="connector" style="position-anchor: --set-{set.id}"></div>

<style>
  .set-options {
    position: absolute;
    background-color: var(--highlight);
    border-radius: 0.5rem;
    border: 1px solid var(--strong-border);
    padding: 0.5rem;
    top: anchor(top);
    left: anchor(right);
    width: 32rem;
    margin-left: 2rem;
  }
  .heading-grid {
    display: grid;
    grid-template-columns: 18rem 12rem;
    gap: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--text-muted);
  }
  .options-grid {
    display: grid;
    grid-template-columns: auto auto;
    column-gap: 0.5rem;
    row-gap: 0.25rem;
    padding-top: 0.5rem;
    font-size: 0.9rem;
  }
  .topic-titles {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: start;
  }
  .problem-options {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    justify-content: center;
    align-items: end;
    font-size: 0.9rem;
    .number-picker {
      font-size: 0.8rem;
    }
  }

  .label-div {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    color: var(--text-muted);
  }

  h2 {
    font-size: 1rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    &.big-text {
      font-size: 1.2rem;
    }
  }

  .spacing-input {
    width: 4rem;
  }

  .connector {
    position: absolute;
    border-top: 1px solid var(--strong-border);
    top: anchor(top);
    left: anchor(right);
    width: 3rem;
  }
</style>
