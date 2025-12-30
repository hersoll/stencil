<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from '../ServerMessage.svelte';
  import { type Entry, type ProblemEntry, type TopicEntryRaw } from '../types';

  let {
    topics = $bindable(),
    serverMessage,
    draggedEntry,
    dropPriority = $bindable(),
    parentDraggedOver = $bindable(),
    problem = $bindable()
  }: {
    topics: TopicEntryRaw[];
    serverMessage: ServerMessage;
    draggedEntry: Entry | null;
    dropPriority: boolean;
    parentDraggedOver: boolean;
    problem: ProblemEntry;
  } = $props();
  let topicsDraggedOver = $state(false);
  let topicDragDepth = $state(0);

  function inTopics(topic: TopicEntryRaw): boolean {
    return topics.find(t => t.id == topic.id) !== undefined;
  }

  function addTopic(topic: TopicEntryRaw) {
    topics.push(topic);
  }

  function removeTopic(topic: TopicEntryRaw) {
    topics = topics.filter(t => t.id !== topic.id);
  }

  // The topic area is entered while dragging
  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    topicDragDepth++;
    dropPriority = true;
    topicsDraggedOver = true;
  }

  function handleDragLeave() {
    topicDragDepth--;
    if (topicDragDepth == 0) {
      dropPriority = false;
      topicsDraggedOver = false;
    }
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    // Only allow non-empty topics
    if (draggedEntry?.kind === 'topic' && draggedEntry?.id >= 0) {
      if (!inTopics(draggedEntry)) {
        addTopic(draggedEntry);
      }
    }
    topicDragDepth--;
    topicsDraggedOver = false;
    // Don't relinquish dropPriority here; let the parent handle it
  }

  let topicWillBeRemoved = $state(false);
  function handleTopicDrag() {
    if (parentDraggedOver) {
      topicWillBeRemoved = false;
    } else if (!parentDraggedOver) {
      topicWillBeRemoved = true;
    }
  }

  function handleTopicDragEnd(topic: TopicEntryRaw) {
    if (topicWillBeRemoved) {
      removeTopic(topic);
    }
    topicWillBeRemoved = false;
  }

  async function fetchTopics() {
    let res = await fetch(`${API_URL}/edit/problem/${problem.id}/topics`);
    if (res.ok) {
      topics = await res.json();
    } else {
      serverMessage.show(res);
    }
  }

  $effect(() => {
    if (problem) fetchTopics();
  });
</script>

<div
  role="status"
  class="topics-container"
  class:parent-dragged={parentDraggedOver}
  class:available={draggedEntry?.kind === 'topic'}
  class:droppable={draggedEntry?.kind === 'topic' && topicsDraggedOver}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
>
  <h3 class="topic-header">Topics</h3>
  {#each topics as topic, i}
    <button
      class="topic-entry no-select"
      class:parent-dragged={parentDraggedOver}
      id={topic.name}
      draggable="true"
      in:fly={{ y: 40, duration: 400, delay: 20 * i }}
      ondrag={handleTopicDrag}
      ondragend={() => handleTopicDragEnd(topic)}
    >
      {topic.desc.sv}
    </button>
  {/each}
</div>

<style>
  .topics-container {
    grid-column: 1 / 3;
    grid-row: 1/3;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    width: 100%;
    height: 20rem;
    padding: 1rem;
    overflow-y: auto;

    background-color: var(--bg-light);
    border-radius: 1rem;
    box-shadow: var(--shadow-elevation-low);

    text-align: center;

    &.parent-dragged {
      background-color: var(--bg);
    }

    &.available {
      background-color: var(--bg-light);
    }

    &.droppable {
      border: 2px solid black;
    }
  }

  .topic-header {
    margin-bottom: 1rem;
  }

  .topic-entry {
    border: 2px solid var(--bg-dark);
    box-shadow: var(--shadow-elevation-low);
    width: 100%;
    &:hover {
      border-color: var(--primary);
    }
    &:active {
      background-color: var(--bg-light);
      &:not(.parent-dragged) {
        background-color: var(--bg);
        opacity: 0.3;
      }
    }
  }
</style>
