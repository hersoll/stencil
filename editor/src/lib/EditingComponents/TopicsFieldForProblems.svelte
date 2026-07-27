<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from '../ServerMessage.svelte';
  import {
    type Entry,
    type TopicDifficultyData,
    type TopicEntryRaw
  } from '../types';

  let {
    topic_data = $bindable(),
    serverMessage,
    draggedEntry,
    dropPriority = $bindable(),
    parentDraggedOver = $bindable(),
    entry = $bindable()
  }: {
    topic_data: TopicDifficultyData[];
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
    topic_data.push({
      topic_id: topic.id,
      absolute_difficulty: 4,
      relative_difficulty: 4
    });
  }

  function removeTopic(topic: TopicEntryRaw) {
    topics = topics.filter(t => t.id !== topic.id);
    topic_data = topic_data.filter(
      data_topic => data_topic.topic_id !== topic.id
    );
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
    const newOrder_data = [...topic_data];
    const [removed_entry] = newOrder_topics.splice(draggedIndex, 1);
    const [removed_data] = newOrder_data.splice(draggedIndex, 1);
    newOrder_topics.splice(targetIndex, 0, removed_entry);
    newOrder_data.splice(targetIndex, 0, removed_data);
    topics = newOrder_topics;
    topic_data = newOrder_data;

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
  <div class="header-container">
    <h3>Topics</h3>
    <h4>Absolute</h4>
    <h4>Relative</h4>
  </div>
  {#each topic_data as data, i (data.topic_id)}
    {@const topic = topics.find(t => t.id === data.topic_id)}
    {#if topic}
      <button
        class="topic-entry no-select"
        class:parent-dragged={parentDraggedOver}
        class:available={draggedEntry?.kind === 'topic'}
        id={topic?.name}
        draggable="true"
        in:fly={{ y: 40, duration: 400, delay: 20 * i }}
        ondrag={handleTopicDrag}
        ondragend={() => handleTopicDragEnd(topic)}
        ondragstart={() => handleTopicDragStart(topic, i)}
        ondragover={e => handleTopicDragOver(e, i)}
      >
        {topic.desc.sv}
        <input type="number" bind:value={data.absolute_difficulty} />
        <input type="number" bind:value={data.relative_difficulty} />
      </button>
    {/if}
  {/each}
</div>

<style>
  .topics-container {
    grid-column: 1 / 4;
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

  .header-container {
    margin-bottom: 1rem;
    padding-left: 0.5rem;
    padding-right: 1rem;
    width: 100%;
    display: grid;
    grid-template-columns: 1fr 5.5rem 3.5rem;
    text-align: left;
  }

  .topic-entry {
    border: 2px solid var(--bg-dark);
    box-shadow: var(--shadow-elevation-low);
    width: 100%;
    display: grid;
    grid-template-columns: 1fr 5rem 3rem;
    text-align: left;
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

    input {
      width: 3rem;
      font-size: 1rem;
      padding: 0.1rem;
      margin-left: 0.25rem;
      border: none;
    }
  }
</style>
