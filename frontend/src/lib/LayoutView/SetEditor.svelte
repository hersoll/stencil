<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { setIDsWithHeader } from './layoutStates.svelte';
  import { type SetState, type View } from '$src/types';
  import DifficultySelector from '../AddSetView/DifficultySelector.svelte';
  import { setState } from '$src/globalStates.svelte';
  import EditIcon from '../SVGIcons/EditIcon.svelte';

  let { set = $bindable(), view = $bindable() }: { set: SetState; view: View } =
    $props();

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
      set.options.formatting_options.spacing = Math.max(
        MIN_SPACING,
        Math.min(MAX_SPACING, value)
      );
    } else {
      set.options.formatting_options.spacing = null;
    }
  }

  const MIN_COLUMNS = 1;
  const MAX_COLUMNS = 5;
  function handleColumnsBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      set.options.formatting_options.questionColumns = Math.max(
        MIN_COLUMNS,
        Math.min(MAX_COLUMNS, value)
      );
    } else {
      set.options.formatting_options.questionColumns = MIN_COLUMNS;
    }
  }

  function checkHeaderBox() {
    if (!setIDsWithHeader.includes(set.id)) setIDsWithHeader.push(set.id);
  }
  function uncheckHeaderBox() {
    setIDsWithHeader.splice(setIDsWithHeader.indexOf(set.id), 1);
    set.options.formatting_options.heading = null;
  }
</script>

<div
  class="card"
  id="set-editor-card-{set.id}"
  style="position-anchor: --set-{set.id}"
>
  <button
    class="mobile edit-set-btn"
    onclick={() => {
      setState.currentEditedSetID = set.id;
      view = 'editSet';
    }}
  >
    <EditIcon />
  </button>
  <div class="card-header">
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
        <label for="n-{set.id}">Uppgifter:</label>
        <input
          id="n-{set.id}"
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
        id="pagebreak-after"
        type="checkbox"
        checked={set.options.formatting_options.pagebreakAfter}
        onchange={e =>
          (set.options.formatting_options.pagebreakAfter =
            e.currentTarget.checked)}
      />
    </div>
    <div class="label-div">
      <label for="spacing">{i18n.t('set_option_spacing')}</label>
      <input
        type="number"
        class="spacing-input"
        id="spacing"
        value={set.options.formatting_options.spacing}
        min="0"
        max="120"
        oninput={e =>
          (set.options.formatting_options.spacing =
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
      <label for="columns-{set.id}"> {i18n.t('set_option_columns')}</label>
      <input
        type="number"
        id="columns-{set.id}"
        min="1"
        max="5"
        value={set.options.formatting_options.questionColumns}
        onchange={e =>
          (set.options.formatting_options.questionColumns =
            e.currentTarget.valueAsNumber)}
        onblur={handleColumnsBlur}
      />
    </div>
  </div>
</div>

<style>
  .card {
    position: absolute;
    top: anchor(top);
    left: anchor(right);
    width: 31rem;
    margin-left: 2rem;
  }
  .card-header {
    display: grid;
    grid-template-columns: 18rem 12rem;
    gap: 1rem;
  }
  .topic-titles {
    display: flex;
    flex-direction: column;
  }

  .options-grid {
    display: grid;
    grid-template-columns: auto auto;
    column-gap: 0.5rem;
    row-gap: 0.25rem;
    padding-top: 0.5rem;
    font-size: 0.9rem;
  }
  .problem-options {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    justify-content: start;
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

  .edit-set-btn {
    display: none;
  }

  @container main (width < 70rem) {
    .card {
      position: relative;
      margin-left: 0;
    }
  }
  @container body (width < 50rem) {
    .card {
      position: relative;
      width: 100%;
      max-width: 30rem;

      /* Increasing the font size to prevent iOS zoom */
      label {
        font-size: 1rem;
      }
      input {
        font-size: 1rem;
      }
    }
    .card-header {
      display: grid;
      grid-template-columns: auto;
      gap: 1rem;
      margin-bottom: 0;

      .problem-options {
        align-items: start;
        row-gap: 0.4rem;
        input {
          font-size: 1rem;
        }
      }
    }
    .options-grid {
      padding-top: 0.4rem;
      grid-template-columns: auto;
    }

    .edit-set-btn {
      display: block;
      position: absolute;
      background: none;
      width: 1.5rem;
      height: 1.5rem;
      top: 0.5rem;
      right: 0rem;
      padding: 0;
      border: none;
    }
  }
</style>
