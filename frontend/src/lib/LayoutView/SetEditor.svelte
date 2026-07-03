<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import type { SetState } from '$src/types';

  let { set }: { set: SetState } = $props();

  const MIN_SPACING = 0;
  const MAX_SPACING = 300;
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
</script>

<div class="set-options" style="position-anchor: --set-{set.id}">
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
      bind:value={set.options.set_options.spacing}
      onchange={e =>
        (set.options.set_options.spacing = e.currentTarget.valueAsNumber)}
      onblur={handleSpacingBlur}
    />
  </div>
  <div class="label-div">
    <label for="heading">{i18n.t('set_option_heading')}</label>
    <input
      type="text"
      value={set.options.set_options.heading}
      id="heading"
      class="text-input"
      placeholder="Lös uppgifterna"
      onchange={e => (set.options.set_options.heading = e.currentTarget.value)}
    />
  </div>
  <div class="label-div">
    <label for="columns"> {i18n.t('set_option_columns')}</label>
    <input
      type="number"
      id="columns"
      bind:value={set.options.set_options.questionColumns}
      onchange={e =>
        (set.options.set_options.questionColumns =
          e.currentTarget.valueAsNumber)}
      onblur={handleColumnsBlur}
    />
  </div>
</div>

<style>
  .set-options {
    position: absolute;
    background-color: var(--bg-dark);
    border-radius: 0.5rem;
    padding: 0.5rem;
    /* position-area: right; */
    top: anchor(top);
    left: anchor(right);
    width: fit-content;
    margin-left: 2rem;
  }

  .spacing-input {
    width: 4rem;
  }
</style>
