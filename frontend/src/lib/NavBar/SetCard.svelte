<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { API_URL } from '$src/main';
  import { error, setState } from '$src/globalStates.svelte';
  import { fade, fly } from 'svelte/transition';
  import {
    type ProblemSetSpec,
    type TopicWithProblems,
    type View
  } from '$src/types';
  import DeleteIcon from '../SVGIcons/DeleteIcon.svelte';
  import ReorderIcon from '../SVGIcons/ReorderIcon.svelte';

  let {
    set = $bindable(),
    setID,
    setIndex,
    view = $bindable(),
    navbarOpen
  }: {
    set: ProblemSetSpec;
    setID: number;
    setIndex: number;
    view: View;
    navbarOpen: boolean;
  } = $props();

  const MAX_TOPICS_SHOWN = 3;

  let topicsWithProblems = $state<TopicWithProblems[]>([]);
  let showLoadingMessage = $state(false);
  let isDraggable = $state(false);
  let isDragging = $derived(setState.draggedSetIndex == setIndex);

  const delay = setTimeout(() => {
    if (topicsWithProblems.length == 0) showLoadingMessage = true;
  }, 600);

  async function fetchProblems() {
    const res = await fetch(`${API_URL}/${i18n.currentLanguage}/problems`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(set.problems.topics)
    });
    showLoadingMessage = false;
    clearTimeout(delay);
    if (!res.ok) {
      error.message = `Status code ${res.status} \n ${await res.text()}`;
    }
    topicsWithProblems = await res.json();
  }

  function deleteSet() {
    setState.addedSets = setState.addedSets.filter(
      addedSet => addedSet.id !== setID
    );
  }

  $effect(() => {
    if (i18n.currentLanguage) {
      fetchProblems();
    }
  });

  function moveSetToIndex(newIndex: number) {
    const newSetOrder = [...setState.addedSets];
    const [removed_set] = newSetOrder.splice(newIndex, 1);
    newSetOrder.splice(setIndex, 0, removed_set);
    setState.addedSets = newSetOrder;
  }

  function handleDragStart() {
    setState.draggedSetIndex = setIndex;
  }

  function handleDragEnd() {
    setState.draggedSetIndex = null;
    isDraggable = false;
  }
  // Move the set if another set is dragged over it
  function handleDragOver(e: DragEvent) {
    e.preventDefault();

    if (
      setState.draggedSetIndex == null ||
      setState.draggedSetIndex === setIndex
    )
      return;

    moveSetToIndex(setState.draggedSetIndex);
    setState.draggedSetIndex = setIndex;
  }
</script>

<div
  class="card {view === 'editSet' && setState.currentEditedSetID === setID
    ? 'selected'
    : ''} 
  {isDragging ? 'dragging' : ''}"
  role="listitem"
  draggable={isDraggable}
  ondragstart={handleDragStart}
  ondragend={handleDragEnd}
  ondragover={handleDragOver}
>
  <button
    class="set-btn
  {navbarOpen ? 'nav-open' : 'nav-closed'}"
    in:fly={{ y: 40, duration: 400 }}
    disabled={topicsWithProblems.length == 0}
    onclick={() => {
      setState.currentEditedSetID = setID;
      setState.currentEditedSetContents = topicsWithProblems;
      view = 'editSet';
    }}
  >
    {#if showLoadingMessage}
      <h3 in:fade={{ duration: 500 }}>{i18n.t('loading')}...</h3>
    {:else if topicsWithProblems.length <= MAX_TOPICS_SHOWN}
      {#each topicsWithProblems as topic}
        <h3 in:fade={{ duration: 200 }}>{topic.desc}</h3>
      {/each}
    {:else}
      <h3 in:fade={{ duration: 200 }}>
        {topicsWithProblems[0].desc}
      </h3>
      <h3 in:fade={{ duration: 200 }}>
        + {topicsWithProblems.length - 1}
        {i18n.t('topics').toLowerCase()}
      </h3>
    {/if}
    <div class="set-description">
      <p in:fade={{ duration: 300 }}>
        {set.problems.n}
        {i18n.t('problems').toLowerCase()}
      </p>
      <p in:fade={{ duration: 300 }}>
        {set.problems.startingDifficulty == set.problems.endingDifficulty
          ? i18n.t(set.problems.startingDifficulty)
          : i18n.t(set.problems.startingDifficulty) +
            ' ' +
            i18n.t('to') +
            ' ' +
            i18n.t(set.problems.endingDifficulty)}
      </p>
    </div>
  </button>

  <div class="icon-container">
    <button
      class="reorder-btn"
      onmouseover={() => (isDraggable = true)}
      onfocus={() => (isDraggable = true)}
      popovertarget="reordering-popover-{setIndex}"
    >
      <ReorderIcon />
    </button>

    <button class="delete-btn" onclick={deleteSet}>
      <DeleteIcon />
    </button>
    <div popover id="reordering-popover-{setIndex}" class="reordering-popover">
      <button
        class="move-up-btn"
        disabled={setIndex === 0}
        onclick={() => moveSetToIndex(setIndex - 1)}
        popovertarget="reordering-popover-{setIndex}"
        popovertargetaction="hide"
      >
        {i18n.t('move_up')}
      </button>
      <button
        class="move-down-btn"
        disabled={setIndex === setState.addedSets.length - 1}
        onclick={() => moveSetToIndex(setIndex + 1)}
        popovertarget="reordering-popover-{setIndex}"
        popovertargetaction="hide"
      >
        {i18n.t('move_down')}
      </button>
    </div>
  </div>
</div>

<style>
  .card {
    position: relative;
    background-color: var(--bg);
    border-radius: 0.5rem;
    &.selected {
      background-color: var(--highlight);
      .icon-container {
        .reorder-btn {
          &:hover {
            background-color: var(--bg);
          }
        }
      }
    }
    &.dragging {
      background-color: var(--highlight);
    }
  }
  .set-btn {
    text-align: left;
    background: none;
    width: 100%;
    padding: 0.5rem;
    border: none;
    display: flex;
    flex-direction: column;

    &.nav-closed {
      display: none;
    }

    &:hover {
      background-color: var(--highlight);
      border: none;
      cursor: pointer;
      &:disabled {
        border: none;
        cursor: default;
      }
    }

    h3 {
      font-size: 0.9rem;
      color: var(--text);

      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    p {
      font-size: 0.8rem;
      margin-top: 0.15rem;
      color: var(--text-muted);
    }

    .set-description {
      margin-top: 0.2rem;
      display: flex;
      flex-direction: column;
    }
  }
  .icon-container {
    position: absolute;
    right: 0.2rem;
    bottom: 0.2rem;
    display: flex;
    gap: 0.5rem;
    align-items: center;
    button {
      background: none;
      border: none;
      width: 1.5rem;
      height: 1.5rem;
      padding: 0;
      transition: background-color 0.2s;
    }

    button > :global(svg) {
      opacity: 0;
      transition: opacity 0.3s;
    }

    .reorder-btn {
      &:hover {
        cursor: grab;
        background-color: var(--highlight);
      }
    }

    .delete-btn {
      &:hover {
        background-color: var(--danger);
      }
    }
  }

  .card:hover {
    .icon-container :global(svg) {
      opacity: 1;
    }
  }

  .reordering-popover {
    inset: auto;
    margin: 0;
    position-area: top;
    opacity: 0;

    padding: 0;
    border: 2px solid var(--bg-light);
    border-radius: 0.5rem;
    background: none;
    margin-bottom: 0.25rem;

    transition: all 0.25s allow-discrete;

    &:popover-open {
      opacity: 1;
      display: flex;
      flex-direction: column;
    }
    @starting-style {
      &:popover-open {
        opacity: 0;
      }
    }

    button {
      padding: 0.4rem;
      margin: 0;
      width: auto;
      height: auto;
      font-size: 0.8rem;
      background-color: var(--bg-light);
      border: none;
      border-radius: 0;
      box-shadow: none;
      &:hover:enabled {
        background-color: var(--bg-dark);
      }
      &:disabled {
        color: lightgray;
      }
    }
  }
</style>
