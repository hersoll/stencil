<script lang="ts">
  import ProblemEditor from './ProblemEditor.svelte';
  import type { Entry } from './types';

  let {
    activeEntry = $bindable(),
    clickedEntry
  }: {
    activeEntry: Entry | null;
    clickedEntry: Entry | null;
  } = $props();

  let draggedOver = $state(false);

  /// used to store the "real" value while drag preview is showing
  let temp_storage: Entry | null;
  let dragDepth = $state(0);

  //Required for handling child components
  function handleDragOver(e: DragEvent) {
    e.preventDefault();
  }

  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    dragDepth++;
    if (dragDepth === 1) {
      if (activeEntry) {
        temp_storage = { ...activeEntry };
      }
      if (
        clickedEntry &&
        (activeEntry?.kind == clickedEntry.kind || !activeEntry)
      ) {
        activeEntry = { ...clickedEntry };
      }
      draggedOver = true;
    }
  }

  function handleDragLeave() {
    dragDepth--;
    if (dragDepth == 0) {
      draggedOver = false;
      if (temp_storage) {
        activeEntry = { ...temp_storage };
      } else {
        activeEntry = null;
      }
      temp_storage = null;
    }
  }

  function handleDrop(e: DragEvent) {
    dragDepth--;
    e.preventDefault();
    if (clickedEntry) {
      activeEntry = { ...clickedEntry };
    }
    draggedOver = false;
    temp_storage = null;
  }
</script>

<div
  role="region"
  class="container"
  class:drag-over={draggedOver}
  ondragover={handleDragOver}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
>
  {#if activeEntry?.kind == 'problem'}
    <ProblemEditor bind:problem={activeEntry} {draggedOver} />
  {/if}
</div>

<style>
  .container {
    width: 53.5rem;
    padding: 1rem;
    border-radius: 1rem;
    box-shadow: 6px 4px 20px oklch(from var(--bg) calc(l - 0.1) c h) inset;
  }

  .drag-over {
    background-color: var(--bg-dark);
    box-shadow: 6px 4px 20px oklch(from var(--bg-dark) calc(l - 0.2) c h) inset;
  }
</style>
