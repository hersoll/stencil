<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { document_options } from '$src/states.svelte';

  /* VERY ugly to do one handleBlur() for each, but it was the best way I found */
  const FONT_SIZE_MIN = 4;
  const FONT_SIZE_MAX = 40;
  function handleFontSizeBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      document_options.font_size = Math.max(
        FONT_SIZE_MIN,
        Math.min(FONT_SIZE_MAX, value)
      );
    } else {
      document_options.font_size = FONT_SIZE_MIN;
    }
  }
  const ANSWER_COLUMNS_MIN = 1;
  const ANSWER_COLUMNS_MAX = 3;
  function handleAnswerColumnsBlur(
    e: Event & { currentTarget: HTMLInputElement }
  ) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      document_options.answer_columns = Math.max(
        ANSWER_COLUMNS_MIN,
        Math.min(ANSWER_COLUMNS_MAX, value)
      );
    } else {
      document_options.answer_columns = ANSWER_COLUMNS_MIN;
    }
  }
  const MARGIN_MIN = 0;
  const MARGIN_MAX = 100;
  function handleXMarginBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      document_options.x_margin = Math.max(
        MARGIN_MIN,
        Math.min(MARGIN_MAX, value)
      );
    } else {
      document_options.x_margin = MARGIN_MIN;
    }
  }
  function handleYMarginBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      document_options.y_margin = Math.max(
        MARGIN_MIN,
        Math.min(MARGIN_MAX, value)
      );
    } else {
      document_options.y_margin = MARGIN_MIN;
    }
  }

  const PAR_SPACING_MIN = 0;
  const PAR_SPACING_MAX = 200;
  let parSpacingEnabled = $state(false);
  let parSpacingChoice = $state(PAR_SPACING_MIN);
  function handleParSpacingCheck(event: Event) {
    const checkbox = event.target as HTMLInputElement;
    if (checkbox.checked) {
      document_options.par_spacing = parSpacingChoice;
    } else {
      document_options.par_spacing = null;
    }
  }
  function handleParSpacingBlur(
    e: Event & { currentTarget: HTMLInputElement }
  ) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      document_options.par_spacing = Math.max(
        PAR_SPACING_MIN,
        Math.min(PAR_SPACING_MAX, value)
      );
      parSpacingChoice = document_options.par_spacing;
    } else {
      document_options.par_spacing = null;
    }
  }

  const PREFIX_GROUP_MIN = 0;
  const PREFIX_GROUP_MAX = 10;
  function handlePrefixGroupBlur(
    e: Event & { currentTarget: HTMLInputElement }
  ) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      document_options.max_prefix_group = Math.max(
        PAR_SPACING_MIN,
        Math.min(PAR_SPACING_MAX, value)
      );
    } else {
      document_options.max_prefix_group = PREFIX_GROUP_MIN;
    }
  }
</script>

<h2>{i18n.t('options')}</h2>

<div class="options">
  <!-- TITLE  -->
  <div class="title-container">
    <label for="title">{i18n.t('document_option_title')}</label>
    <input name="title" type="text" bind:value={document_options.title} />
  </div>

  <!-- LANGUAGE -->
  <div class="language-container">
    <label for="language">{i18n.t('document_option_language')}</label>
    <select name="language" id="language" bind:value={document_options.lang}>
      <option value="Sv">{i18n.t('language_sv')}</option>
      <option value="En">{i18n.t('language_en')}</option>
    </select>
  </div>

  <!-- WRITE SOLUTIONS  -->
  <div class="solutions-container">
    <label for="write_solutions">{i18n.t('document_option_solutions')}</label>
    <select
      name="write_solutions"
      id="write_solutions"
      bind:value={document_options.write_solutions}
    >
      <option value="First">{i18n.t('document_option_solutions_first')}</option>
      <option value="All">{i18n.t('document_option_solutions_all')}</option>
      <option value="None">{i18n.t('document_option_solutions_none')}</option>
    </select>
  </div>

  <!-- PAPER SIZE -->
  <div class="paper-size-container">
    <label for="paper_size">{i18n.t('document_option_paper_size')}</label>
    <select
      name="paper_size"
      id="paper_size"
      bind:value={document_options.paper_size}
    >
      <option value="A4">A4</option>
      <option value="A5">A5</option>
    </select>
  </div>

  <!-- FONT SIZE -->
  <div class="font-size-container">
    <label for="font-size">{i18n.t('document_option_font_size')}</label>
    <input
      name="font-size"
      type="number"
      bind:value={document_options.font_size}
      min={FONT_SIZE_MIN}
      max={FONT_SIZE_MAX}
      onblur={handleFontSizeBlur}
    />
  </div>

  <!-- ANSWER COLUMNS -->
  <div class="answer-columns-container">
    <label for="answer-columns"
      >{i18n.t('document_option_answer_columns')}</label
    >
    <input
      name="answer-columns"
      type="number"
      bind:value={document_options.answer_columns}
      min={ANSWER_COLUMNS_MIN}
      max={ANSWER_COLUMNS_MAX}
      onblur={handleAnswerColumnsBlur}
    />
  </div>

  <!-- MARGINS  -->
  <div class="margins-container">
    <p>Margins (FIX)</p>
    <label for="x-margin">x</label>
    <input
      name="x-margin"
      type="number"
      bind:value={document_options.x_margin}
      min={MARGIN_MIN}
      max={MARGIN_MAX}
      onblur={handleXMarginBlur}
    />
    <label for="y-margin">y</label>
    <input
      name="y-margin"
      type="number"
      bind:value={document_options.y_margin}
      min={MARGIN_MIN}
      max={MARGIN_MAX}
      onblur={handleYMarginBlur}
    />
  </div>

  <!-- PAR SPACING -->
  <div class="par-spacing-container">
    <label for="par-spacing">{i18n.t('document_option_spacing')}</label>
    <input
      type="checkbox"
      name="par-spacing"
      bind:checked={parSpacingEnabled}
      onchange={handleParSpacingCheck}
    />
    {#if parSpacingEnabled}
      <input
        name="par-spacing-amount"
        type="number"
        bind:value={parSpacingChoice}
        min={PAR_SPACING_MIN}
        max={PAR_SPACING_MAX}
        onblur={handleParSpacingBlur}
      />
    {/if}
  </div>

  <!-- PREFIX GROUP -->
  <div class="prefix-group-container">
    <label for="prefix-group">{i18n.t('document_option_prefix_group')}</label>
    <input
      name="prefix-group"
      type="number"
      bind:value={document_options.max_prefix_group}
      min={PREFIX_GROUP_MIN}
      max={PREFIX_GROUP_MAX}
      onblur={handlePrefixGroupBlur}
    />
  </div>

  <!-- COLOR  -->
  <div class="color-container">
    <label for="color">{i18n.t('document_option_color')}</label>
    <input name="color" type="checkbox" bind:checked={document_options.color} />
  </div>
</div>

<style>
  .options {
    margin-top: 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  h2 {
    margin: 0;
    font-size: 2rem;
  }
</style>
