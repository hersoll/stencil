<script lang="ts">
  import { stripKind, type Entry, type Kind } from './types';
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
  let entryIsNew = $state(false);

  let privateEntryCount = $state(0);
  // Will contain the reset function from the EntryList, so it can be called inside this component
  let resetList = $state(() => {});

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
      if (clickedEntry.id == -1) {
        entryIsNew = true;
      } else {
        entryIsNew = false;
      }
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
      activeEntry = { ...clickedEntry };
      originalEntry = JSON.stringify(activeEntry);
      entryIsNew = true;
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
      resetList();
    }
  }

  async function publishEntries() {
    const response = await fetch(`${API_URL}/edit/publish/${kind}`);
    serverMessage.show(response);
    resetList();
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
      bind:resetList
      getPrivateCount={(count: number) => (privateEntryCount = count)}
    />
    <EditingArea
      {clickedEntry}
      bind:activeEntry
      bind:originalEntry
      {entryHasBeenEdited}
      {entryIsNew}
      {editDialog}
      {editEntry}
      {resetList}
    />
  </div>

  {#if privateEntryCount > 0}
    <button class="publish-all-btn" onclick={publishEntries}>
      Publish every {kind} ({privateEntryCount})
    </button>
  {/if}
  <button
    class="clear-btn"
    onclick={() => {
      if (entryHasBeenEdited) clearDialog.show();
      else {
        activeEntry = null;
      }
    }}
  >
    Clear
  </button>
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
    max-width: 100rem;
    margin: 0 auto;
    position: relative;
    padding: 2rem;
    background: none;
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

  .publish-all-btn {
    position: absolute;
    top: 2rem;
    right: 8rem;
    background-color: lavender;
    box-shadow: var(--shadow-elevation-low);
  }
</style>
