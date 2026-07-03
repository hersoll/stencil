<script lang="ts">
  import { documentOptions, setState } from '$src/globalStates.svelte';
  import type { SetState } from '$src/types';
  import SetSkeleton from './SetSkeleton.svelte';

  let { isActive }: { isActive: boolean } = $props();

  let pages = $derived.by(() => {
    let pages: SetState[][] = [[]];
    setState.addedSets.forEach(set => {
      pages[pages.length - 1].push(set);
      if (set.options.set_options.pagebreakAfter) {
        pages.push([]);
      }
    });
    // Remove dangling page if last set has pagebreak
    if (pages[pages.length - 1].length === 0) {
      pages.pop();
    }
    return pages;
  });
</script>

<div class="page-container {isActive ? 'open' : 'closed'}">
  {#each pages as page, i}
    <div class="page">
      {#if i === 0}
        <h1>First page</h1>
      {/if}
      {#each page as set}
        <SetSkeleton {set} />
      {/each}
      {#if i === pages.length - 1 && !documentOptions.pageBreakBeforeAnswers}
        <h1>Answers</h1>
      {/if}
    </div>
  {/each}
  {#if documentOptions.pageBreakBeforeAnswers}
    <div class="page">
      <h1>Answers</h1>
    </div>
  {/if}
</div>

<style>
  .page-container {
    flex: 1 1 auto;
    width: 100%;
    min-height: 0;
    min-width: 0;

    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    background: none;
    padding: 1rem;
  }

  .page {
    width: 100%;
    flex-shrink: 1;
    padding: 0.5rem;
    background-color: var(--bg-light);
    border: 1px solid var(--border);
    box-shadow: var(--shadow-elevation-low);
  }

  @container main (width < 70rem) {
    .page-container.open {
      width: 100%;
    }
    .page-container.closed {
      display: none;
    }
  }

  h1 {
    font-size: 2rem;
    text-align: center;
    margin-bottom: 2rem;
  }
</style>
