<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from './ServerMessage.svelte';
  import type {
    Entry,
    PrefixEntryRaw,
    ProblemEntry,
    TopicEntryRaw
  } from './types';
  import PrefixField from './EditingComponents/PrefixField.svelte';
  import DescriptionField from './EditingComponents/DescriptionField.svelte';
  import ProblemTranslationsField from './EditingComponents/ProblemTranslationsField.svelte';
  import LanguageHeader from './EditingComponents/LanguageHeader.svelte';
  import NewOrEditingLabel from './EditingComponents/NewOrEditingLabel.svelte';
  import SubmitButton from './EditingComponents/SubmitButton.svelte';
  import TopicsField from './EditingComponents/TopicsField.svelte';

  let {
    problem = $bindable(),
    draggedEntry,
    draggedOver,
    dropPriority = $bindable()
  }: {
    problem: ProblemEntry;
    draggedOver: boolean;
    draggedEntry: Entry | null;
    dropPriority: boolean;
  } = $props();

  let serverMessage: ServerMessage;
  let currentPrefix: PrefixEntryRaw | null = $state(null);
  let problem_topics: TopicEntryRaw[] = $state([]);

  async function handleSubmit() {
    const method =
      problem.id < 0
        ? // New problem
          'POST'
        : // Existing problem
          'PATCH';
    const topic_ids = problem_topics.map(t => t.id);
    const response = await fetch(`${API_URL}/edit/problem`, {
      method,
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify([problem, topic_ids])
    });

    serverMessage.show(response);
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
  in:fly={{ y: -15, duration: 200 }}
  out:fly={{ y: 15, duration: 200 }}
>
  <NewOrEditingLabel entry={problem} />

  <!-- TRANSLATIONS -->
  <div class="translation-grid">
    <span></span>
    <label for="module">Module</label>
    <label for="name">Name</label>

    <span></span>

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
    <TopicsField
      bind:topics={problem_topics}
      {serverMessage}
      bind:problem
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
  @import './editingArea.css';

  .attachments-grid {
    margin-top: 2rem;
    display: grid;
    grid-template-columns: 6rem 1fr 1fr;
    grid-template-rows: 8rem auto;
    column-gap: 1rem;
  }
</style>
