<script lang="ts">
  import { stripKind, type Entry } from './types';
  import { API_URL } from '$src/main';
  import ContextMenu from './ContextMenu.svelte';
  import ServerMessage from './ServerMessage.svelte';
  import EntryList from './EntryList.svelte';
  import EditingArea from './EditingArea.svelte';
  import { fly } from 'svelte/transition';
  import ConfirmDialog from './ConfirmDialog.svelte';

  let clickedEntry = $state<Entry | null>(null);
  let activeEntry = $state<Entry | null>(null);
  let originalEntry = $state('');
  let entryHasBeenEdited = $state(false);

  type Kind = 'problem' | 'topic' | 'chapter' | 'course' | 'prefix';
  let kind = $state<Kind>('problem');
  let kinds = [
    { name: 'course', desc: 'Courses' },
    { name: 'chapter', desc: 'Chapters' },
    { name: 'topic', desc: 'Topics' },
    { name: 'problem', desc: 'Problems' },
    { name: 'prefix', desc: 'Prefixes' }
  ];

  function isKind(s: string): s is Kind {
    return ['problem', 'topic', 'chapter', 'course', 'prefix'].includes(s);
  }

  let contextMenu: ContextMenu;
  let serverMessage: ServerMessage;
  let copyDialog: ConfirmDialog;
  let editDialog: ConfirmDialog;
  let clearDialog: ConfirmDialog;

  function onClickOutsideList() {
    clickedEntry = null;
  }

  function editEntry() {
    if (entryHasBeenEdited) {
      editDialog.show();
    } else {
      commitEdit();
    }
  }

  function commitEdit() {
    if (clickedEntry) {
      activeEntry = { ...clickedEntry };
      originalEntry = JSON.stringify(activeEntry);
    }
    clickedEntry = null;
  }

  function copyEntry() {
    if (entryHasBeenEdited) {
      copyDialog.show();
    } else {
      commitCopy();
    }
  }

  function commitCopy() {
    if (clickedEntry) {
      activeEntry = { ...clickedEntry, id: -1 };
      originalEntry = JSON.stringify(activeEntry);
    }
    clickedEntry = null;
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
    if (!entryHasBeenEdited) {
      clickedEntry = null;
    }
  }

  function handleEntryClick(e: MouseEvent, entry: Entry) {
    clickedEntry = entry;
    contextMenu.show({ x: e.x, y: e.y });
  }

  $effect(() => {
    entryHasBeenEdited =
      activeEntry !== null && JSON.stringify(activeEntry) !== originalEntry;
  });
</script>

<main in:fly={{ y: 60, duration: 600 }}>
  <div class="btn-container">
    {#each kinds as k}
      <button
        class="kind-switcher"
        value={k.name}
        class:current-kind={kind === k.name}
        disabled={kind === k.name}
        onclick={() => {
          if (isKind(k.name)) kind = k.name;
        }}
      >
        {k.desc}
      </button>
    {/each}
  </div>

  <div class="major-grid">
    <EntryList
      {kind}
      {handleEntryClick}
      {handleEntryDrag}
      {handleEntryDrop}
      {onClickOutsideList}
    />
    <EditingArea
      {clickedEntry}
      bind:activeEntry
      bind:originalEntry
      {entryHasBeenEdited}
      {editDialog}
    />
  </div>
  <button
    class="clear-btn"
    onclick={() => {
      if (entryHasBeenEdited) clearDialog.show();
      else {
        activeEntry = null;
      }
    }}>Clear</button
  >
</main>

<ContextMenu
  bind:this={contextMenu}
  editFunc={editEntry}
  copyFunc={copyEntry}
  deleteFunc={deleteEntry}
  {clickedEntry}
/>

<ServerMessage bind:this={serverMessage} />

<ConfirmDialog
  bind:this={copyDialog}
  onConfirm={() => {
    commitCopy();
  }}
  message={`Are you sure you want to overwrite your changes?`}
/>

<ConfirmDialog
  bind:this={editDialog}
  onConfirm={() => {
    commitEdit();
  }}
  message={`Are you sure you want to overwrite your changes?`}
/>

<ConfirmDialog
  bind:this={clearDialog}
  onConfirm={() => {
    activeEntry = null;
  }}
  message={`Are you sure you want to clear your changes?`}
/>

<style>
  main {
    position: relative;
    padding: 2rem;
    border-radius: 2rem;
    background-color: var(--bg);
    box-shadow: var(--shadow-elevation-low);
  }

  .btn-container {
    display: flex;
    width: 40rem;
    justify-content: space-around;
    margin-bottom: 1rem;
  }

  .kind-switcher {
    box-shadow: var(--shadow-elevation-low);

    &:active {
      box-shadow: none;
    }
    &:disabled {
      box-shadow: none;
      background-color: var(--primary);
      color: var(--text);
      cursor: default;
    }
  }

  .major-grid {
    display: grid;
    justify-content: start;
    grid-template-columns: auto auto;
    gap: 2rem;
  }

  .clear-btn {
    position: absolute;
    top: 2rem;
    right: 2rem;
    box-shadow: var(--shadow-elevation-low);
  }
</style>
