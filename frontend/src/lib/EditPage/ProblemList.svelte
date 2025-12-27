<script lang="ts">
  import { onMount } from 'svelte';
  import { defaultProblemEntry, type ProblemEntry } from './types';
  import { API_URL } from '$src/main';
  import { error } from '$src/states.svelte';
  import { fly } from 'svelte/transition';

  let { search, handleProblemClick, handleProblemDrag, onClickOutsideList } =
    $props();
  let problems = $state<ProblemEntry[]>([]);
  let listElement: HTMLDivElement;

  let foundProblems = $derived.by(() => {
    if (search == '') {
      return problems;
    } else {
      return problems.filter(
        problem =>
          problem.module.toLowerCase().includes(search.toLowerCase()) ||
          problem.name.toLowerCase().includes(search.toLowerCase())
      );
    }
  });

  async function getProblems() {
    const response = await fetch(`${API_URL}/edit/problem`);
    if (!response.ok) {
      error.message = `Status code ${response.status} \n ${await response.text()}`;
    }
    problems = await response.json();
  }

  function handleOutsideClick(e: Event) {
    let popoverElement = document.getElementById('context-menu');
    if (
      !listElement.contains(e.target as Node) &&
      !popoverElement?.contains(e.target as Node)
    ) {
      onClickOutsideList();
    }
  }

  onMount(() => {
    getProblems();
    document.addEventListener('click', handleOutsideClick);

    return () => document.removeEventListener('click', handleOutsideClick);
  });
</script>

<div class="container">
  <div class="list-header">
    <h3 class="header-text">Module</h3>
    <h3 class="header-text">Name</h3>
    <button class="reset-btn" onclick={getProblems}>⟳</button>
  </div>
  <div class="list-grid" bind:this={listElement}>
    <button
      class="list-entry no-select {false ? 'dragging' : ''}"
      draggable="true"
      in:fly={{ y: 40, duration: 400 }}
      ondragstart={e => {
        e.currentTarget.classList.add('dragging');
        handleProblemDrag(defaultProblemEntry);
      }}
      ondragend={e => e.currentTarget.classList.remove('dragging')}
    >
      <p class="list-text placeholder-text">New problem</p>
    </button>
    {#each foundProblems as problem, i}
      <button
        class="list-entry no-select {false ? 'dragging' : ''}"
        onclick={e => handleProblemClick(e, problem)}
        draggable="true"
        in:fly={{ y: 40, duration: 400, delay: 20 * i }}
        ondragstart={e => {
          e.currentTarget.classList.add('dragging');
          handleProblemDrag(problem);
        }}
        ondragend={e => e.currentTarget.classList.remove('dragging')}
      >
        <p class="list-text">{problem.module.replaceAll('_', ' ')}</p>
        <p class="list-text">{problem.name.replaceAll('_', ' ')}</p>
      </button>
    {/each}
  </div>
  <p class="counter">{foundProblems.length} problems found</p>
</div>

<style>
  .container {
    position: relative;
    width: fit-content;
    padding: 1rem;
    border-radius: 1rem;
    box-shadow: 6px 4px 20px oklch(from var(--bg) calc(l - 0.1) c h) inset;
  }

  .list-entry,
  .list-header {
    display: grid;
    column-gap: 0.5rem;
    grid-template-columns: 15rem 20rem;
    padding-left: 0.5rem;
  }
  .list-header {
    margin-bottom: 0.5rem;
  }

  .reset-btn {
    position: absolute;
    top: 0.75rem;
    right: 1rem;
    font-size: 1.5rem;
    padding: 0;
    margin: 0;
    background: none;
    border: none;
    &:hover {
      color: var(--text-muted);
    }
  }

  .list-grid {
    background-color: var(--bg);
    display: flex;
    flex-direction: column;
    row-gap: 0.25rem;
    height: calc(100vh - 30rem);
    overflow-y: auto;
    width: fit-content;
    padding-right: 1rem;
    margin-bottom: 0.5rem;

    p {
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }
  }

  .list-entry {
    align-self: self-start;
    text-align: left;
    font-weight: 400;
    background-color: var(--bg-light);
    box-shadow: var(--shadow-elevation-low);
    padding: 0.5rem;
    border-radius: 0.5rem;
    cursor: pointer;
  }

  .list-text {
    font-size: 0.9rem;
  }

  .placeholder-text {
    color: var(--text-muted);
  }

  .counter {
    text-align: center;
  }

  .dragging {
    opacity: 0.7;
    border: 2px solid transparent;
  }
</style>
