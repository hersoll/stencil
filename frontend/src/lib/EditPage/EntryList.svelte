<script lang="ts">
  import { onMount } from 'svelte';
  import {
    defaultChapterEntry,
    defaultCourseEntry,
    defaultPrefixEntry,
    defaultProblemEntry,
    defaultTopicEntry,
    type ChapterEntryRaw,
    type CourseEntryRaw,
    type Entry,
    type PrefixEntryRaw,
    type ProblemEntryRaw,
    type TopicEntryRaw
  } from './types';
  import { API_URL } from '$src/main';
  import { error } from '$src/states.svelte';
  import { fly } from 'svelte/transition';

  let {
    kind,
    search,
    handleEntryClick,
    handleEntryDrag,
    handleEntryDrop,
    onClickOutsideList
  } = $props();
  let entries = $state<Entry[]>([]);
  let listElement: HTMLDivElement;
  let defaultEntry: Entry;

  let foundEntries = $derived.by(() => {
    if (search == '') {
      return entries;
    } else {
      return entries.filter(
        entry =>
          entry.name.toLowerCase().includes(search.toLowerCase()) ||
          (entry.kind == 'problem' &&
            entry.module.toLowerCase().includes(search.toLowerCase())) ||
          (entry.kind == 'prefix' &&
            entry.translations.sv.text
              .toLowerCase()
              .includes(search.toLowerCase()))
      );
    }
  });

  async function fetchEntries() {
    const response = await fetch(`${API_URL}/edit/${kind}`);
    if (!response.ok) {
      error.message = `Status code ${response.status} \n ${await response.text()}`;
    }
    return response;
  }

  async function getProblems() {
    const response = await fetchEntries();
    const rawEntries: ProblemEntryRaw[] = await response.json();
    entries = rawEntries.map(p => ({
      ...p,
      kind: 'problem'
    }));
  }

  async function getTopics() {
    const response = await fetchEntries();
    const rawEntries: TopicEntryRaw[] = await response.json();
    entries = rawEntries.map(p => ({
      ...p,
      kind: 'topic'
    }));
  }

  async function getChapters() {
    const response = await fetchEntries();
    const rawEntries: ChapterEntryRaw[] = await response.json();
    entries = rawEntries.map(p => ({
      ...p,
      kind: 'chapter'
    }));
  }

  async function getCourses() {
    const response = await fetchEntries();
    const rawEntries: CourseEntryRaw[] = await response.json();
    entries = rawEntries.map(p => ({
      ...p,
      kind: 'course'
    }));
  }

  async function getPrefixes() {
    const response = await fetchEntries();
    const rawEntries: PrefixEntryRaw[] = await response.json();
    entries = rawEntries.map(p => ({
      ...p,
      kind: 'prefix'
    }));
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
    document.addEventListener('click', handleOutsideClick);

    return () => document.removeEventListener('click', handleOutsideClick);
  });

  async function resetList() {
    entries = [];
    switch (kind) {
      case 'problem':
        await getProblems();
        break;
      case 'topic':
        await getTopics();
        break;
      case 'chapter':
        await getChapters();
        break;
      case 'course':
        await getCourses();
        break;
      case 'prefix':
        await getPrefixes();
        break;
    }
  }

  $effect(() => {
    switch (kind) {
      case 'problem':
        defaultEntry = { ...defaultProblemEntry };
        break;
      case 'topic':
        defaultEntry = { ...defaultTopicEntry };
        break;
      case 'chapter':
        defaultEntry = { ...defaultChapterEntry };
        break;
      case 'course':
        defaultEntry = { ...defaultCourseEntry };
        break;
      case 'prefix':
        defaultEntry = { ...defaultPrefixEntry };
        break;
    }
    resetList();
  });
</script>

<div class="container">
  <div class="list-header">
    <h3 class="header-text">{kind == 'problem' ? 'Module' : 'Name'}</h3>
    <h3 class="header-text">{kind == 'problem' ? 'Name' : 'Description'}</h3>
    <button class="reset-btn" onclick={resetList}>⟳</button>
  </div>
  <div class="list-grid" bind:this={listElement}>
    <button
      class="list-entry no-select new-entry {false ? 'dragging' : ''}"
      draggable="true"
      in:fly={{ y: 40, duration: 400 }}
      ondragstart={e => {
        e.currentTarget.classList.add('dragging');
        handleEntryDrag(defaultEntry);
      }}
      ondragend={e => {
        e.currentTarget.classList.remove('dragging');
        handleEntryDrop();
      }}
    >
      <p class="list-text placeholder-text">New {kind}</p>
    </button>
    {#each foundEntries as entry, i}
      <button
        class="list-entry no-select {false ? 'dragging' : ''}"
        onclick={e => handleEntryClick(e, entry)}
        draggable="true"
        in:fly={{ y: 40, duration: 400, delay: 20 * i }}
        ondragstart={e => {
          e.currentTarget.classList.add('dragging');
          handleEntryDrag(entry);
        }}
        ondragend={e => {
          e.currentTarget.classList.remove('dragging');
          handleEntryDrop();
        }}
      >
        {#if entry.kind == 'problem'}
          <p class="list-text">{entry.module.replaceAll('_', ' ')}</p>
          <p class="list-text">{entry.name.replaceAll('_', ' ')}</p>
        {:else if entry.kind == 'prefix'}
          <p class="list-text">{entry.name.replaceAll('_', ' ')}</p>
          <p class="list-text">{entry.translations.sv.text}</p>
        {:else}
          <p class="list-text">{entry.name.replaceAll('_', ' ')}</p>
          <p class="list-text">{entry.desc.sv}</p>
        {/if}
      </button>
    {/each}
  </div>
  <div class="counter-container">
    {#if entries.length > 0}
      <p class="counter" in:fly={{ y: -10, duration: 400 }}>
        {foundEntries.length}
        {kind}{kind == 'prefix' ? 'es' : 's'} found
      </p>
    {/if}
  </div>
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
  .new-entry {
    grid-template-columns: 35.5rem;
    justify-items: center;
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

  .counter-container {
    height: 1.2rem;
  }
  .counter {
    text-align: center;
  }

  .dragging {
    opacity: 0.7;
    border: 2px solid transparent;
  }
</style>
