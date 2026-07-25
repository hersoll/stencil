<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from '../ServerMessage.svelte';
  import type { Entry, ChapterEntry } from '../types';
  import DescriptionField from '../EditingComponents/DescriptionField.svelte';
  import LanguageHeader from '../EditingComponents/LanguageHeader.svelte';
  import NewOrEditingLabel from '../EditingComponents/NewOrEditingLabel.svelte';
  import SubmitButton from '../EditingComponents/SubmitButton.svelte';
  import TopicsField from '../EditingComponents/TopicsField.svelte';
  import CoursesField from '../EditingComponents/CoursesField.svelte';

  let {
    chapter = $bindable(),
    draggedEntry,
    draggedOver,
    originalEntry = $bindable(),
    activeEntry = $bindable(),
    dropPriority = $bindable()
  }: {
    chapter: ChapterEntry;
    draggedOver: boolean;
    draggedEntry: Entry | null;
    originalEntry: string;
    activeEntry: Entry | null;
    dropPriority: boolean;
  } = $props();

  let serverMessage: ServerMessage;

  async function handleSubmit() {
    const method =
      chapter.id < 0
        ? // New topic
          'POST'
        : // Existing topic
          'PATCH';
    const response = await fetch(`${API_URL}/edit/chapter`, {
      method,
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(chapter)
    });

    originalEntry = JSON.stringify(activeEntry);
    serverMessage.show(response);
  }
</script>

<ServerMessage bind:this={serverMessage} />

<div
  class="editing-area-container"
  class:dragged-over={draggedOver}
  in:fly={{ y: -15, duration: 600 }}
>
  <NewOrEditingLabel entry={chapter} />

  <!-- TRANSLATIONS -->
  <div class="translation-grid">
    <span></span>
    <label for="name" class="name-label">Name</label>

    <div style="display: flex; align-items: center;">
      <label for="public" style="margin-right: 0.4rem;">Public?</label>
      <input name="public" type="checkbox" bind:checked={chapter.public} />
    </div>
    <input
      name="name"
      type="text"
      class="editing-text-input name-input"
      bind:value={chapter.name}
    />

    <LanguageHeader />
    <DescriptionField bind:entry={chapter} />
  </div>
  <div class="attachments-grid">
    <TopicsField
      --height="26rem"
      entry={chapter}
      bind:topic_ids={chapter.topic_ids}
      bind:dropPriority
      {serverMessage}
      parentDraggedOver={draggedOver}
      {draggedEntry}
    />
    <CoursesField
      --height="26rem"
      bind:course_ids={chapter.course_ids}
      {serverMessage}
      {draggedEntry}
      parentDraggedOver={draggedOver}
      bind:dropPriority
      bind:entry={chapter}
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
    grid-template-columns: 1fr 1fr 1fr;
    column-gap: 1rem;
  }
</style>
