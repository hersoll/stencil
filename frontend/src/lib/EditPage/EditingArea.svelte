<script lang="ts">
  import './editingArea.css';
  import PrefixEditor from './PrefixEditor.svelte';
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
  /// Keep track of drags over children to know when there is an
  /// actual enter/exit
  let dragDepth = $state(0);
  /// We might want to drop something into a child component.
  /// This prevents the parent (this) from overriding those areas
  let childHasDropPriority = $state(false);

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
      // Are we actually dragging an entry?
      if (clickedEntry) {
        if (temp_storage) {
          activeEntry = { ...temp_storage };
        } else {
          activeEntry = null;
        }
      }
      temp_storage = null;
    }
  }

  function handleDrop(e: DragEvent) {
    dragDepth--;
    e.preventDefault();
    // Drop is on parent area and not child area
    if (!childHasDropPriority && clickedEntry) {
      activeEntry = { ...clickedEntry };
    }
    draggedOver = false;
    temp_storage = null;
    childHasDropPriority = false;
  }
</script>

<div
  role="region"
  id="editing-area"
  class="editing-area"
  class:drag-over={draggedOver}
  ondragover={handleDragOver}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
>
  {#if activeEntry?.kind == 'problem'}
    <ProblemEditor
      bind:problem={activeEntry}
      {draggedOver}
      draggedEntry={clickedEntry}
      bind:dropPriority={childHasDropPriority}
    />
  {:else if activeEntry?.kind == 'prefix'}
    <PrefixEditor bind:prefix={activeEntry} {draggedOver} />
  {/if}
</div>

<style>
  .editing-area {
    min-height: 20rem;
    width: 53.5rem;
    padding: 1rem;
    border-radius: 1rem;
    box-shadow: 6px 4px 20px oklch(from var(--bg) calc(l - 0.1) c h) inset;
    height: min-content;
  }

  .drag-over {
    background-color: var(--bg-dark);
    box-shadow: 6px 4px 20px oklch(from var(--bg-dark) calc(l - 0.2) c h) inset;
  }
</style>
