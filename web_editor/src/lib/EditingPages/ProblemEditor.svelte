<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from '../ServerMessage.svelte';
  import type { Entry, PrefixEntryRaw, ProblemEntry } from '../types';
  import PrefixField from '../EditingComponents/PrefixField.svelte';
  import DescriptionField from '../EditingComponents/DescriptionField.svelte';
  import ProblemTranslationsField from '../EditingComponents/ProblemTranslationsField.svelte';
  import LanguageHeader from '../EditingComponents/LanguageHeader.svelte';
  import NewOrEditingLabel from '../EditingComponents/NewOrEditingLabel.svelte';
  import SubmitButton from '../EditingComponents/SubmitButton.svelte';
  import TopicsFieldForProblems from '../EditingComponents/TopicsFieldForProblems.svelte';

  let {
    problem = $bindable(),
    draggedEntry,
    originalEntry = $bindable(),
    activeEntry = $bindable(),
    entryIsCopy,
    draggedOver,
    dropPriority = $bindable(),
    resetList
  }: {
    problem: ProblemEntry;
    draggedOver: boolean;
    originalEntry: string;
    activeEntry: Entry | null;
    entryIsCopy: boolean;
    draggedEntry: Entry | null;
    dropPriority: boolean;
    resetList: Function;
  } = $props();

  let serverMessage: ServerMessage;
  let currentPrefix: PrefixEntryRaw | null = $state(null);

  async function handleSubmit() {
    const method = entryIsCopy
      ? // New problem
        'POST'
      : // Existing problem
        'PATCH';
    const response = await fetch(`${API_URL}/edit/problem`, {
      method,
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(problem)
    });

    serverMessage.show(response);
    originalEntry = JSON.stringify(activeEntry);
    resetList();
  }

  async function loadPrefixData() {
    const res = await fetch(`${API_URL}/edit/prefix/id/${problem.prefix_id}`);
    if (res.ok) {
      currentPrefix = await res.json();
    } else {
      await serverMessage.show(res);
    }
  }

  $effect(() => {
    if (problem.prefix_id) {
      loadPrefixData();
    } else {
      currentPrefix = null;
    }
  });
</script>

<ServerMessage bind:this={serverMessage} />

<div
  class="editing-area-container"
  class:dragged-over={draggedOver}
  in:fly={{ y: -15, duration: 600 }}
>
  <NewOrEditingLabel entry={problem} {entryIsCopy} />

  <!-- TRANSLATIONS -->
  <div class="translation-grid">
    <div></div>
    <label for="module">Module</label>
    <label for="name">Name</label>

    <div style="display: flex; align-items: center;">
      <label for="public" style="margin-right: 0.4rem;">Public?</label>
      <input name="public" type="checkbox" bind:checked={problem.public} />
    </div>
    <input
      name="module"
      type="text"
      class="editing-text-input"
      bind:value={problem.module}
    />
    <input
      name="name"
      type="text"
      class="editing-text-input"
      bind:value={problem.name}
    />

    <LanguageHeader />
    <DescriptionField bind:entry={problem} />
    <ProblemTranslationsField bind:problem />
  </div>
  <div class="attachments-grid">
    <TopicsFieldForProblems
      --height="16rem"
      bind:topic_data={problem.topic_data}
      {serverMessage}
      bind:entry={problem}
      {draggedEntry}
      bind:dropPriority
      parentDraggedOver={draggedOver}
    />
    <PrefixField
      {currentPrefix}
      bind:dropPriority
      bind:problem
      {draggedEntry}
      bind:parentDraggedOver={draggedOver}
    />
    <SubmitButton {handleSubmit} />
  </div>
</div>

<style>
  @import '../editingArea.css';

  .attachments-grid {
    margin-top: 2rem;
    display: grid;
    grid-template-columns: 6rem 1fr 1fr;
    gap: 1rem;
  }
</style>
