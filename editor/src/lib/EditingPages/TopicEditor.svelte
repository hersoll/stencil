<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from '../ServerMessage.svelte';
  import type { Entry, TopicEntry } from '../types';
  import DescriptionField from '../EditingComponents/DescriptionField.svelte';
  import LanguageHeader from '../EditingComponents/LanguageHeader.svelte';
  import NewOrEditingLabel from '../EditingComponents/NewOrEditingLabel.svelte';
  import SubmitButton from '../EditingComponents/SubmitButton.svelte';
  import ProblemsField from '../EditingComponents/ProblemsField.svelte';
  import ChaptersField from '../EditingComponents/ChaptersField.svelte';

  let {
    topic = $bindable(),
    draggedEntry,
    draggedOver,
    originalEntry = $bindable(),
    activeEntry = $bindable(),
    entryIsCopy,
    dropPriority = $bindable(),
    resetList
  }: {
    topic: TopicEntry;
    draggedOver: boolean;
    draggedEntry: Entry | null;
    originalEntry: string;
    activeEntry: Entry | null;
    entryIsCopy: boolean;
    dropPriority: boolean;
    resetList: Function;
  } = $props();

  let serverMessage: ServerMessage;

  async function handleSubmit() {
    const method = entryIsCopy
      ? // New problem
        'POST'
      : // Existing problem
        'PATCH';
    const response = await fetch(`${API_URL}/edit/topic`, {
      method,
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(topic)
    });

    originalEntry = JSON.stringify(activeEntry);
    serverMessage.show(response);
    resetList();
  }
</script>

<ServerMessage bind:this={serverMessage} />

<div
  class="editing-area-container"
  class:dragged-over={draggedOver}
  in:fly={{ y: -15, duration: 600 }}
>
  <NewOrEditingLabel entry={topic} {entryIsCopy} />

  <!-- TRANSLATIONS -->
  <div class="translation-grid">
    <span></span>
    <label for="name" class="name-label">Name</label>

    <div style="display: flex; align-items: center;">
      <label for="public" style="margin-right: 0.4rem;">Public?</label>
      <input name="public" type="checkbox" bind:checked={topic.public} />
    </div>

    <input
      name="name"
      type="text"
      class="editing-text-input name-input"
      bind:value={topic.name}
    />

    <LanguageHeader />
    <DescriptionField bind:entry={topic} />
  </div>
  <div class="attachments-grid">
    <ProblemsField
      --height="26rem"
      {topic}
      bind:dropPriority
      {serverMessage}
      parentDraggedOver={draggedOver}
      {draggedEntry}
    />
    <ChaptersField
      --height="26rem"
      bind:chapter_ids={topic.chapter_ids}
      {serverMessage}
      {draggedEntry}
      parentDraggedOver={draggedOver}
      bind:dropPriority
      bind:entry={topic}
    />
  </div>
  <SubmitButton {handleSubmit} />
</div>

<style>
  @import '../editingArea.css';

  .name-input {
    width: 35rem;
  }

  .name-label,
  .name-input {
    grid-column: 2/4;
  }

  .attachments-grid {
    margin-top: 2rem;
    margin-bottom: 2rem;
    display: grid;
    grid-template-columns: 36rem 1fr;
    grid-template-rows: 8rem auto;
    column-gap: 1rem;
  }
</style>
