<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from './ServerMessage.svelte';
  import type {
    ChapterEntryRaw,
    Entry,
    ProblemEntryRaw,
    TopicEntry
  } from './types';
  import DescriptionField from './EditingComponents/DescriptionField.svelte';
  import LanguageHeader from './EditingComponents/LanguageHeader.svelte';
  import NewOrEditingLabel from './EditingComponents/NewOrEditingLabel.svelte';
  import SubmitButton from './EditingComponents/SubmitButton.svelte';
  import ProblemsField from './EditingComponents/ProblemsField.svelte';
  import ChaptersField from './EditingComponents/ChaptersField.svelte';

  let {
    topic = $bindable(),
    draggedEntry,
    draggedOver,
    dropPriority = $bindable()
  }: {
    topic: TopicEntry;
    draggedOver: boolean;
    draggedEntry: Entry | null;
    dropPriority: boolean;
  } = $props();

  let serverMessage: ServerMessage;
  let topic_problems: ProblemEntryRaw[] = $state([]);
  let topic_chapters: ChapterEntryRaw[] = $state([]);

  async function handleSubmit() {
    const method =
      topic.id < 0
        ? // New problem
          'POST'
        : // Existing problem
          'PATCH';
    const problem_ids = topic_problems.map(p => p.id);
    const chapter_ids = topic_chapters.map(c => c.id);
    const response = await fetch(`${API_URL}/edit/topic`, {
      method,
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        topic,
        problems: problem_ids,
        chapters: chapter_ids
      })
    });

    serverMessage.show(response);
  }
</script>

<ServerMessage bind:this={serverMessage} />

<div
  class="editing-area-container"
  class:dragged-over={draggedOver}
  in:fly={{ y: -15, duration: 600 }}
>
  <NewOrEditingLabel entry={topic} />

  <!-- TRANSLATIONS -->
  <div class="translation-grid">
    <span></span>
    <label for="name" class="name-label">Name</label>

    <span></span>
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
      {topic}
      bind:problems={topic_problems}
      bind:dropPriority
      {serverMessage}
      parentDraggedOver={draggedOver}
      {draggedEntry}
    />
    <ChaptersField
      bind:chapters={topic_chapters}
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
  @import './editingArea.css';

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
