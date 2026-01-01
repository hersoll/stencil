<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from './ServerMessage.svelte';
  import type { Entry, ChapterEntryRaw, CourseEntry } from './types';
  import DescriptionField from './EditingComponents/DescriptionField.svelte';
  import LanguageHeader from './EditingComponents/LanguageHeader.svelte';
  import NewOrEditingLabel from './EditingComponents/NewOrEditingLabel.svelte';
  import SubmitButton from './EditingComponents/SubmitButton.svelte';
  import ChaptersField from './EditingComponents/ChaptersField.svelte';

  let {
    course = $bindable(),
    draggedEntry,
    draggedOver,
    dropPriority = $bindable()
  }: {
    course: CourseEntry;
    draggedOver: boolean;
    draggedEntry: Entry | null;
    dropPriority: boolean;
  } = $props();

  let serverMessage: ServerMessage;
  let course_chapters: ChapterEntryRaw[] = $state([]);

  async function handleSubmit() {
    const method =
      course.id < 0
        ? // New chapter
          'POST'
        : // Existing chapter
          'PATCH';
    const chapter_ids = course_chapters.map(p => p.id);
    const response = await fetch(`${API_URL}/edit/course`, {
      method,
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify([course, chapter_ids])
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
  <NewOrEditingLabel entry={course} />

  <!-- TRANSLATIONS -->
  <div class="translation-grid">
    <span></span>
    <label for="name" class="name-label">Name</label>

    <span></span>
    <input
      name="name"
      type="text"
      class="editing-text-input name-input"
      bind:value={course.name}
    />

    <LanguageHeader />
    <DescriptionField bind:entry={course} />
  </div>
  <div class="chapter-container">
    <ChaptersField
      --height="28rem"
      entry={course}
      bind:chapters={course_chapters}
      bind:dropPriority
      {serverMessage}
      parentDraggedOver={draggedOver}
      {draggedEntry}
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

  .chapter-container {
    margin-top: 2rem;
    margin-bottom: 1rem;
  }
</style>
