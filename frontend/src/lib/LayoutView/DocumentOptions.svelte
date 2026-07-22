<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { documentOptions } from '$src/globalStates.svelte';

  /* VERY ugly to do one handleBlur() for each, but it was the best way I found */
  const FONT_SIZE_MIN = 4;
  const FONT_SIZE_MAX = 40;
  function handleFontSizeBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      documentOptions.fontSize = Math.max(
        FONT_SIZE_MIN,
        Math.min(FONT_SIZE_MAX, value)
      );
    } else {
      documentOptions.fontSize = FONT_SIZE_MIN;
    }
  }

  const MARGIN_MIN = 0;
  const MARGIN_MAX = 100;
  function handleXMarginBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      documentOptions.xMargin = Math.max(
        MARGIN_MIN,
        Math.min(MARGIN_MAX, value)
      );
    } else {
      documentOptions.xMargin = MARGIN_MIN;
    }
  }
  function handleYMarginBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      documentOptions.yMargin = Math.max(
        MARGIN_MIN,
        Math.min(MARGIN_MAX, value)
      );
    } else {
      documentOptions.yMargin = MARGIN_MIN;
    }
  }

  const PAR_SPACING_MIN = 0;
  const PAR_SPACING_MAX = 200;
  function handleParSpacingBlur(
    event: Event & { currentTarget: HTMLInputElement }
  ) {
    const value = parseInt(event.currentTarget.value);
    if (!isNaN(value)) {
      documentOptions.parSpacing = Math.max(
        PAR_SPACING_MIN,
        Math.min(PAR_SPACING_MAX, value)
      );
    } else {
      documentOptions.parSpacing = null;
    }
  }

  const PREFIX_GROUP_MIN = 1;
  const PREFIX_GROUP_MAX = 10;
  function handlePrefixGroupBlur(
    e: Event & { currentTarget: HTMLInputElement }
  ) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      documentOptions.maxPrefixGroup = Math.max(
        PAR_SPACING_MIN,
        Math.min(PAR_SPACING_MAX, value)
      );
    } else {
      documentOptions.maxPrefixGroup = PREFIX_GROUP_MIN;
    }
  }
</script>

<!-- PREFIX GROUP -->
<div class="prefix-group container">
  <label for="prefix-group">{i18n.t('document_option_prefix_group')}</label>
  <input
    name="prefix-group"
    type="number"
    bind:value={documentOptions.maxPrefixGroup}
    min={PREFIX_GROUP_MIN}
    max={PREFIX_GROUP_MAX}
    onblur={handlePrefixGroupBlur}
  />
</div>

<!-- PAR SPACING -->
<div class="par-spacing container">
  <label for="par-spacing">{i18n.t('document_option_spacing')}</label>
  <input
    id="par-spacing"
    type="number"
    bind:value={documentOptions.parSpacing}
    min={PAR_SPACING_MIN}
    max={PAR_SPACING_MAX}
    onblur={handleParSpacingBlur}
  />
</div>

<!-- LANGUAGE -->
<div class="language container">
  <label for="language">{i18n.t('document_option_language')}</label>
  <select name="language" id="language" bind:value={documentOptions.lang}>
    <option value="Sv">{i18n.t('language_sv')}</option>
    <option value="En">{i18n.t('language_en')}</option>
  </select>
</div>

<!-- FONT SIZE -->
<div class="font-size container">
  <label for="font-size">{i18n.t('document_option_font_size')}</label>
  <input
    name="font-size"
    type="number"
    bind:value={documentOptions.fontSize}
    min={FONT_SIZE_MIN}
    max={FONT_SIZE_MAX}
    onblur={handleFontSizeBlur}
  />
</div>

<!-- PAPER SIZE -->
<!-- <div class="paper-size container"> -->
<!--   <label for="paper_size">{i18n.t('document_option_paper_size')}</label> -->
<!--   <select -->
<!--     name="paper_size" -->
<!--     id="paper_size" -->
<!--     bind:value={documentOptions.paperSize} -->
<!--   > -->
<!--     <option value="A4">A4</option> -->
<!--     <option value="A5">A5</option> -->
<!--   </select> -->
<!-- </div> -->

<!-- MARGINS  -->
<div class="margins container">
  <label for="x-margin">{i18n.t('document_option_margins')}</label>
  <input
    id="x-margin"
    type="number"
    bind:value={documentOptions.xMargin}
    min={MARGIN_MIN}
    max={MARGIN_MAX}
    onblur={handleXMarginBlur}
  />
  <input
    id="y-margin"
    type="number"
    bind:value={documentOptions.yMargin}
    min={MARGIN_MIN}
    max={MARGIN_MAX}
    onblur={handleYMarginBlur}
  />
</div>

<!-- COLOR  -->
<div class="color container">
  <label for="color">{i18n.t('document_option_color')}</label>
  <input
    id="color"
    type="checkbox"
    class="checkbox"
    bind:checked={documentOptions.color}
  />
</div>

<style>
  .container {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  label {
    font-size: 1.1rem;
    font-weight: 500;
  }
  input {
    font-size: 1rem;
    background-color: var(--bg-light);
    padding: 0.25rem 0.5rem;
    border: none;
    border-radius: 0.5rem;
    width: 4rem;
    box-shadow: var(--shadow-elevation-low);
  }
  select {
    font-size: 1rem;
    background-color: var(--bg-light);
    border: none;
    padding: 0.25rem;
    box-shadow: var(--shadow-elevation-low);
    border-radius: 0.5rem;
  }
  .checkbox {
    width: auto;
    box-shadow: none;
  }

  /* Mobile layout */
  @container body (width < 50rem) {
    label {
      font-weight: 400;
      font-size: 1rem;
    }
  }
</style>
