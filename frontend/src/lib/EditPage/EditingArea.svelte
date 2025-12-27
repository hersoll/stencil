<script lang="ts">
  import ProblemEditor from './ProblemEditor.svelte';
  import type { ProblemEntry } from './types';

  let {
    activeProblem = $bindable(),
    clickedProblem
  }: {
    activeProblem: ProblemEntry | null;
    clickedProblem: ProblemEntry;
  } = $props();

  let draggedOver = $state(false);

  /// used to store the "real" value while drag preview is showing
  let temp_storage: ProblemEntry;
  let dragDepth = $state(0);

  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    dragDepth++;
    if (dragDepth === 1) {
      if (activeProblem) {
        temp_storage = { ...activeProblem };
      }
      activeProblem = { ...clickedProblem };
      draggedOver = true;
    }
  }

  //Required for handling child components
  function handleDragOver(e: DragEvent) {
    e.preventDefault();
  }

  function handleDragLeave() {
    dragDepth--;
    if (dragDepth == 0) {
      draggedOver = false;
      if (temp_storage) {
        activeProblem = { ...temp_storage };
      } else {
        activeProblem = null;
      }
    }
  }

  function handleDrop(e: DragEvent) {
    dragDepth--;
    e.preventDefault();
    activeProblem = { ...clickedProblem };
    draggedOver = false;
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
  {#if activeProblem}
    <ProblemEditor bind:problem={activeProblem} {draggedOver} />
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
