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
  import { error } from '$src/lib/error.svelte';
  import { fly } from 'svelte/transition';

  let {
    kind,
    handleEntryClick,
    handleEntryDrag,
    handleEntryDrop,
    onClickOutsideList
  } = $props();
  let search = $state('');
  let entries = $state<Entry[]>([]);
  let listElement: HTMLDivElement;
  let defaultEntry: Entry;

  const MINIMUM_SEARCH_CHARS = 3;
  function matchesSearch(entry: Entry, search: string): boolean {
    // Only start searching after minimum
    if (search.length < MINIMUM_SEARCH_CHARS) {
      return true;
    }

    search = search.toLowerCase();
    if (search.startsWith('id: ')) {
      return entry.id.toString() == search.slice(4);
    }
    if (search.startsWith('name: ')) {
      return entry.name.includes(search.slice(6));
    }
    if ('public'.startsWith(search)) {
      return entry.public;
    }
    if ('private'.startsWith(search)) {
      return !entry.public;
    }

    return (
      entry.name.includes(search) ||
      ((entry.kind == 'problem' ||
        entry.kind == 'topic' ||
        entry.kind == 'chapter' ||
        entry.kind == 'course') &&
        (entry.desc.sv.toLowerCase().includes(search) ||
          entry.desc.en.toLowerCase().includes(search))) ||
      (entry.kind == 'problem' &&
        (entry.module.includes(search) ||
          entry.translations.sv.question.toLowerCase().includes(search) ||
          entry.translations.sv.answer.toLowerCase().includes(search) ||
          entry.translations.sv.solution.toLowerCase().includes(search) ||
          entry.translations.en.question.toLowerCase().includes(search) ||
          entry.translations.en.answer.toLowerCase().includes(search) ||
          entry.translations.en.solution.toLowerCase().includes(search))) ||
      (entry.kind == 'prefix' &&
        (entry.translations.sv.text.toLowerCase().includes(search) ||
          entry.translations.en.text.toLowerCase().includes(search)))
    );
  }

  let foundEntries = $derived.by(() => {
    if (search == '') {
      return entries;
    } else {
      return entries.filter(entry => matchesSearch(entry, search));
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
  <div class="utility-grid">
    <input
      class="search-bar"
      type="search"
      placeholder="Search"
      autocorrect="off"
      bind:value={search}
      onkeydown={e =>
        e.key === 'Enter' && (e.preventDefault(), e.currentTarget?.blur())}
    />
    <button class="reset-btn" onclick={resetList}>⟳</button>
  </div>
  <div class="list-header">
    <h3 class="header-text">{kind == 'problem' ? 'Module' : 'Name'}</h3>
    <h3 class="header-text">{kind == 'problem' ? 'Name' : 'Description'}</h3>
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
        class="list-entry no-select {false ? 'dragging' : ''} {entry.public
          ? ''
          : 'private'}"
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
    display: flex;
    flex-direction: column;
    height: 54.75rem;
    position: relative;
    width: fit-content;
    padding: 1rem;
    border-radius: 1rem;
    box-shadow: 6px 4px 20px oklch(from var(--bg) calc(l - 0.1) c h) inset;
  }

  .utility-grid {
    display: flex;
    justify-content: space-between;
  }
  .search-bar {
    width: 19rem;
    background-color: var(--bg-light);
    padding: 0.5rem;
    font-size: 1rem;
    border: none;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    margin-top: 0.5rem;
    box-shadow: var(--shadow-elevation-low);
  }

  .list-entry,
  .list-header {
    display: grid;
    column-gap: 0.5rem;
    grid-template-columns:
      clamp(10rem, calc(45vw - 30.45rem), 15rem)
      clamp(14rem, calc(60vw - 40rem), 20rem);
    padding-left: 0.5rem;
  }
  .list-header {
    margin-bottom: 0.5rem;
  }
  .new-entry {
    justify-items: center;
    .list-text {
      grid-column: 1 / 3;
    }
  }

  .reset-btn {
    font-size: 2rem;
    padding: 0;
    margin: 0;
    padding-right: 0.5rem;
    padding-bottom: 1rem;
    background: none;
    border: none;
    &:hover {
      color: var(--text-muted);
    }
  }

  .list-grid {
    flex: 1;
    min-height: 0;
    background: none;
    display: flex;
    flex-direction: column;
    row-gap: 0.25rem;
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

    &.private {
      background-color: lavender;
      .list-text {
        color: var(--primary-text);
      }
    }
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
