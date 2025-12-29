<script lang="ts">
  let {
    currentPrefix,
    draggedEntry,
    dropPriority = $bindable(),
    parentDraggedOver = $bindable(),
    problem = $bindable()
  } = $props();

  let prefixDragDepth = $state(0);
  let prefixDraggedOver = $state(false);
  let prefixAreaIsDragged = $state(false);
  let showDeletion = $derived(!parentDraggedOver && prefixAreaIsDragged);
  let temp_storage = $state(null);

  // The prefix area is entered while dragging
  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    prefixDragDepth++;
    dropPriority = true;
    prefixDraggedOver = true;
  }

  function handleDragLeave() {
    prefixDragDepth--;
    if (prefixDragDepth == 0) {
      dropPriority = false;
      prefixDraggedOver = false;
    }
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    if (draggedEntry?.kind === 'prefix') {
      problem.prefix_id = draggedEntry.id;
    }
    prefixDragDepth--;
    prefixDraggedOver = false;
    // Don't relinquish dropPriority here; let the parent handle it
  }

  // The prefix area itself is dragged
  function handleDragStart() {
    prefixAreaIsDragged = true;
    temp_storage = problem.prefix_id;
  }

  function handleDrag() {
    if (parentDraggedOver && problem.prefix_id === null) {
      problem.prefix_id = temp_storage;
    } else if (!parentDraggedOver && problem.prefix_id !== null) {
      problem.prefix_id = null;
    }
  }

  function handleDragEnd() {
    prefixAreaIsDragged = false;
  }
</script>

<div
  role="status"
  class="prefix-container"
  draggable={problem.prefix_id === null ? 'false' : 'true'}
  class:parent-dragged={parentDraggedOver && !prefixAreaIsDragged}
  class:available={draggedEntry?.kind === 'prefix'}
  class:no-prefix={problem.prefix_id === null || showDeletion}
  class:droppable={draggedEntry?.kind === 'prefix' && prefixDraggedOver}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  ondragstart={handleDragStart}
  ondrag={handleDrag}
  ondragend={handleDragEnd}
>
  {#if problem.prefix_id && !showDeletion}
    <h4 class="prefix-heading no-select">Prefix</h4>
    <h4 class="no-select">Single</h4>
    <p class="no-select">{currentPrefix?.translations.sv.text}</p>
    <h4 class="no-select">Multiple</h4>
    <p class="no-select">{currentPrefix?.translations.sv.group_text}</p>
  {:else}
    <p class="no-select">No prefix</p>
  {/if}
</div>

<style>
  .prefix-container {
    cursor: grab;
    display: grid;
    padding: 0.5rem;
    width: 100%;
    grid-template-columns: 4rem 1fr;
    column-gap: 1rem;
    align-items: center;
    margin-bottom: 4rem;
    height: 8rem;
    background-color: var(--bg-light);
    border-radius: 1rem;
    box-shadow: var(--shadow-elevation-low);
    border: 2px dashed transparent;
    transition:
      border-color 0.2s ease,
      background-color 0.4s;

    &.parent-dragged {
      background-color: var(--bg);
    }

    &.no-prefix {
      cursor: default;
      grid-template-columns: 1fr;
      grid-template-rows: 1fr;
      justify-items: center;
      background-color: var(--bg);
      border-color: var(--bg-dark);
      box-shadow: none;
    }

    &.available {
      background-color: var(--bg-light);
      border-color: blueviolet;
      box-shadow: var(--shadow-elevation-medium);
    }

    &.droppable {
      cursor: copy;
      border-color: lightgreen;
    }
  }

  .prefix-heading {
    font-size: 1.2rem;
    grid-column: 1 / 3;
    justify-self: center;
  }
</style>
