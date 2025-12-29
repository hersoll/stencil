<script lang="ts">
  import { stripKind, type Entry } from './types';
  import { API_URL } from '$src/main';
  import ContextMenu from './ContextMenu.svelte';
  import ServerMessage from './ServerMessage.svelte';
  import EntryList from './EntryList.svelte';
  import EditingArea from './EditingArea.svelte';
  import { fly } from 'svelte/transition';

  let search = $state('');

  let clickedEntry = $state<Entry | null>(null);
  let activeEntry = $state<Entry | null>(null);

  let kind = $state<'problem' | 'topic' | 'chapter' | 'course' | 'prefix'>(
    'problem'
  );

  let contextMenu: ContextMenu;
  let serverMessage: ServerMessage;

  function onClickOutsideList() {
    clickedEntry = null;
  }

  function editEntry() {
    if (clickedEntry) {
      activeEntry = { ...clickedEntry };
    }
  }

  function copyEntry() {
    if (clickedEntry) {
      activeEntry = { ...clickedEntry, id: -1 };
    }
  }

  async function deleteEntry() {
    if (clickedEntry) {
      const response = await fetch(`${API_URL}/edit/${clickedEntry.kind}`, {
        method: 'DELETE',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(stripKind(clickedEntry))
      });

      serverMessage.show(response);
    }
  }

  function handleEntryDrag(entry: Entry) {
    clickedEntry = entry;
  }

  function handleEntryDrop() {
    clickedEntry = null;
  }

  function handleEntryClick(e: MouseEvent, entry: Entry) {
    clickedEntry = entry;
    contextMenu.show({ x: e.x, y: e.y });
  }
</script>

<main in:fly={{ y: 60, duration: 600 }}>
  <select
    name="select-kind"
    id="select-kind"
    class="select-kind"
    bind:value={kind}
  >
    <option value="problem">Problems</option>
    <option value="topic">Topics</option>
    <option value="chapter">Chapters</option>
    <option value="course">Courses</option>
    <option value="prefix">Prefixes</option>
  </select>
  <input
    class="search-bar"
    type="search"
    placeholder="Search"
    autocorrect="off"
    bind:value={search}
    onkeydown={e =>
      e.key === 'Enter' && (e.preventDefault(), e.currentTarget?.blur())}
  />

  <div class="major-grid">
    <EntryList
      {kind}
      {handleEntryClick}
      {handleEntryDrag}
      {handleEntryDrop}
      {onClickOutsideList}
      {search}
    />
    <EditingArea {clickedEntry} bind:activeEntry />
  </div>
  <button class="clear-btn" onclick={(activeEntry = null)}>Clear</button>
</main>

<ContextMenu
  bind:this={contextMenu}
  editFunc={editEntry}
  copyFunc={copyEntry}
  deleteFunc={deleteEntry}
  {clickedEntry}
/>

<ServerMessage bind:this={serverMessage} />

<style>
  main {
    position: relative;
    margin-top: 2rem;
    padding: 2rem;
    border-radius: 2rem;
    background-color: var(--bg);
    box-shadow: var(--shadow-elevation-low);
  }

  .major-grid {
    display: grid;
    justify-content: start;
    grid-template-columns: auto auto;
    gap: 2rem;
  }

  .search-bar {
    width: 19rem;
    background-color: var(--bg-light);
    padding: 0.5rem;
    font-size: 1rem;
    border: none;
    border-radius: 0.5rem;
    margin-bottom: 2rem;
    box-shadow: var(--shadow-elevation-low);
  }

  select {
    font-size: 1.1rem;
    background-color: var(--bg-light);
    border: none;
    border-radius: 0.5rem;
    margin-right: 2rem;
  }

  .clear-btn {
    position: absolute;
    top: 2rem;
    right: 2rem;
    box-shadow: var(--shadow-elevation-low);
  }
</style>
