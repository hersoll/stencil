<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from '../ServerMessage.svelte';
  import { type Entry, type TopicEntryRaw } from '../types';

  let {
    topic_ids = $bindable(),
    serverMessage,
    draggedEntry,
    dropPriority = $bindable(),
    parentDraggedOver = $bindable(),
    entry = $bindable()
  }: {
    topic_ids: number[];
    serverMessage: ServerMessage;
    draggedEntry: Entry | null;
    dropPriority: boolean;
    parentDraggedOver: boolean;
    entry: Entry;
  } = $props();
  let topicsDraggedOver = $state(false);
  let topicDragDepth = $state(0);
  let draggedTopic = $state<TopicEntryRaw | null>(null);
  let draggedIndex = $state(-1);
  let topics = $state<TopicEntryRaw[]>([]);

  function inTopics(topic: TopicEntryRaw): boolean {
    return topics.find(t => t.id == topic.id) !== undefined;
  }

  function addTopic(topic: TopicEntryRaw) {
    topics.push(topic);
    topic_ids.push(topic.id);
  }

  function removeTopic(topic: TopicEntryRaw) {
    topics = topics.filter(t => t.id !== topic.id);
    topic_ids = topic_ids.filter(id => id !== topic.id);
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
    } else {
      dropPriority = false;
    }
    topicDragDepth--;
    topicsDraggedOver = false;
  }

  function handleTopicDragStart(topic: TopicEntryRaw, index: number) {
    draggedTopic = topic;
    draggedIndex = index;
  }

  function handleTopicDragOver(e: DragEvent, targetIndex: number) {
    e.preventDefault();

    if (draggedIndex === -1 || draggedIndex === targetIndex) return;

    const newOrder_topics = [...topics];
    const [removed_entry] = newOrder_topics.splice(draggedIndex, 1);
    newOrder_topics.splice(targetIndex, 0, removed_entry);
    topics = newOrder_topics;
    console.log($state.snapshot(topics[0]));
    topic_ids = topics.map(t => t.id);

    draggedIndex = targetIndex;
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
    draggedTopic = null;
    draggedIndex = -1;
  }

  async function fetchTopics() {
    let res = await fetch(
      `${API_URL}/edit/topic/from_${entry.kind}/${entry.id}`
    );
    if (res.ok) {
      topics = await res.json();
    } else {
      serverMessage.show(res);
    }
  }

  $effect(() => {
    if (entry.id >= 0) fetchTopics();
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
  {#each topics as topic, i (topic.id)}
    <button
      class="topic-entry no-select"
      class:parent-dragged={parentDraggedOver}
      class:available={draggedEntry?.kind === 'topic'}
      id={topic.name}
      draggable="true"
      in:fly={{ y: 40, duration: 400, delay: 20 * i }}
      ondrag={handleTopicDrag}
      ondragend={() => handleTopicDragEnd(topic)}
      ondragstart={() => handleTopicDragStart(topic, i)}
      ondragover={e => handleTopicDragOver(e, i)}
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
    height: var(--height);
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
      box-shadow: var(--shadow-elevation-high);
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
    &.parent-dragged {
      background-color: var(--bg);
    }
    &.available {
      background-color: var(--bg-light);
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
