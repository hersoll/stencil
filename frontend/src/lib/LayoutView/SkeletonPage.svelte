<script lang="ts">
  import { adjustValue } from '$src/commonFunctions.svelte';
  import { documentOptions, setState } from '$src/globalStates.svelte';
  import SetSkeleton from './SetSkeleton.svelte';
  import i18n from '$src/i18n.svelte';
  import AnswerSkeleton from './AnswerSkeleton.svelte';

  let { isActive }: { isActive: boolean } = $props();

  let pages = $derived.by(() => {
    let pages: number[][] = [[]];
    for (let i = 0; i < setState.addedSets.length; i++) {
      pages[pages.length - 1].push(i);
      if (setState.addedSets[i].options.formatting_options.pagebreakAfter) {
        pages.push([]);
      }
    }
    // Remove dangling page if last set has pagebreak
    if (pages[pages.length - 1].length === 0) {
      pages.pop();
    }
    return pages;
  });
  let xMargin = $derived(adjustValue(0, 40, documentOptions.xMargin, 0, 2, 1));
  let yMargin = $derived(adjustValue(0, 40, documentOptions.yMargin, 0, 2, 1));
</script>

<div class="page-container {isActive ? 'open' : 'closed'}">
  {#if setState.addedSets.length == 0}{:else}
    {#each pages as page, i}
      <div
        class="page"
        style="--x-margin: {xMargin}rem; --y-margin: {yMargin}rem;"
      >
        {#if i === 0}
          <div class="name-container">
            <input
              type="checkbox"
              id="name-field"
              checked={documentOptions.nameField}
              onchange={e =>
                (documentOptions.nameField = e.currentTarget.checked)}
            />
            <label
              for="name-field"
              class={documentOptions.nameField ? 'checked' : 'unchecked'}
              >{i18n.t('name_field')}: _____________</label
            >
          </div>
          <input
            class="input title"
            placeholder={i18n.t('title_placeholder')}
            type="text"
            bind:value={documentOptions.title}
          />
          <input
            class="input subtitle"
            placeholder={i18n.t('subtitle_placeholder')}
            type="text"
            bind:value={documentOptions.subtitle}
          />
        {/if}
        {#each page as index, j}
          <SetSkeleton bind:set={setState.addedSets[index]} index={j} />
        {/each}
        {#if i === pages.length - 1 && !documentOptions.pageBreakBeforeAnswers}
          <AnswerSkeleton ownPage={false} />
        {/if}
      </div>
    {/each}
    {#if documentOptions.pageBreakBeforeAnswers}
      <div
        class="page"
        style="--x-margin: {xMargin}rem; --y-margin: {yMargin}rem;"
      >
        <AnswerSkeleton ownPage={true} />
      </div>
    {/if}
  {/if}
</div>

<style>
  .page-container {
    flex: 1 1 auto;
    width: 100%;
    min-height: 0;
    height: fit-content;
    min-width: 0;
    max-width: 52rem;

    display: flex;
    flex-direction: column;
    gap: 1.5rem;

    background: none;
    padding: 1rem;
  }

  .page {
    display: flex;
    flex-direction: column;
    align-items: center;
    position: relative;
    width: 100%;
    flex-shrink: 1;
    padding: var(--y-margin) var(--x-margin);
    background-color: var(--bg-light);
    border: 1px solid var(--border);
    box-shadow: var(--shadow-elevation-low);
    transition: all 0.4s;
  }

  @container main (width < 70rem) {
    .page-container.open {
      width: 100%;
    }
    .page-container.closed {
      display: none;
    }
  }

  .input {
    background: none;
    border: none;
    text-align: center;
    field-sizing: content;
    width: fit-content;
  }

  .name-container {
    position: absolute;
    top: 0.75rem;
    right: 0.75rem;
    display: flex;
    align-items: center;
    gap: 0.4rem;

    label {
      &.checked {
        color: var(--text);
      }
      &.unchecked {
        color: var(--text-very-muted);
      }
    }
  }

  .title {
    margin-top: 1.5rem;
    margin-bottom: 0.3rem;
    font-size: 1.2rem;
  }
  .subtitle {
    font-size: 0.9rem;
    margin-bottom: 1.5rem;
  }
</style>
