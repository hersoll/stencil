<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from '../ServerMessage.svelte';
  import { type ChapterEntryRaw, type Entry } from '../types';

  let {
    chapter_ids = $bindable(),
    serverMessage,
    draggedEntry,
    dropPriority = $bindable(),
    parentDraggedOver,
    entry = $bindable()
  }: {
    chapter_ids: number[];
    serverMessage: ServerMessage;
    draggedEntry: Entry | null;
    dropPriority: boolean;
    parentDraggedOver: boolean;
    entry: Entry;
  } = $props();
  let draggedOver = $state(false);
  let dragDepth = $state(0);
  let draggedChapter = $state<ChapterEntryRaw | null>(null);
  let draggedIndex = $state(-1);
  let chapters = $state<ChapterEntryRaw[]>([]);

  function inChapters(chapter: ChapterEntryRaw): boolean {
    return chapters.find(c => c.id == chapter.id) !== undefined;
  }

  function addChapter(chapter: ChapterEntryRaw) {
    chapters.push(chapter);
    chapter_ids.push(chapter.id);
  }

  function removeChapter(chapter: ChapterEntryRaw) {
    chapters = chapters.filter(c => c.id !== chapter.id);
    chapter_ids = chapter_ids.filter(id => id !== chapter.id);
  }

  // The topic area is entered while dragging
  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    dragDepth++;
    dropPriority = true;
    draggedOver = true;
  }

  function handleDragLeave() {
    dragDepth--;
    if (dragDepth == 0) {
      dropPriority = false;
      draggedOver = false;
    }
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    // Only allow non-empty chapters
    if (draggedEntry?.kind === 'chapter' && draggedEntry?.id >= 0) {
      if (!inChapters(draggedEntry)) {
        addChapter(draggedEntry);
      }
    } else {
      dropPriority = false;
    }
    dragDepth--;
    draggedOver = false;
  }

  function handleChapterDragStart(chapter: ChapterEntryRaw, index: number) {
    draggedChapter = chapter;
    draggedIndex = index;
  }

  function handleChapterDragOver(e: DragEvent, targetIndex: number) {
    e.preventDefault();

    if (draggedIndex === -1 || draggedIndex === targetIndex) return;

    const newOrder = [...chapters];
    const [removed_entry] = newOrder.splice(draggedIndex, 1);
    newOrder.splice(targetIndex, 0, removed_entry);
    chapters = newOrder;
    chapter_ids = chapters.map(c => c.id);

    draggedIndex = targetIndex;
  }

  let chapterWillBeRemoved = $state(false);
  function handleChapterDrag() {
    if (parentDraggedOver) {
      chapterWillBeRemoved = false;
    } else if (!parentDraggedOver) {
      chapterWillBeRemoved = true;
    }
  }

  function handleChapterDragEnd(chapter: ChapterEntryRaw) {
    if (chapterWillBeRemoved) {
      removeChapter(chapter);
    }
    chapterWillBeRemoved = false;
    draggedChapter = null;
    draggedIndex = -1;
  }
  async function fetchChapters() {
    let res = await fetch(
      `${API_URL}/edit/chapter/from_${entry.kind}/${entry.id}`
    );
    if (res.ok) {
      chapters = await res.json();
    } else {
      serverMessage.show(res);
    }
  }

  $effect(() => {
    if (entry.id >= 0) fetchChapters();
  });
</script>

<div
  role="status"
  class="chapters-container"
  class:parent-dragged={parentDraggedOver}
  class:available={draggedEntry?.kind === 'chapter'}
  class:droppable={draggedEntry?.kind === 'chapter' && draggedOver}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
>
  <h3 class="chapter-header">Chapters</h3>
  {#each chapters as chapter, i (chapter.id)}
    <button
      class="chapter-entry no-select"
      class:parent-dragged={parentDraggedOver}
      class:available={draggedEntry?.kind === 'chapter'}
      id={chapter.name}
      draggable="true"
      in:fly={{ y: 40, duration: 400, delay: 20 * i }}
      ondrag={handleChapterDrag}
      ondragend={() => handleChapterDragEnd(chapter)}
      ondragstart={() => handleChapterDragStart(chapter, i)}
      ondragover={e => handleChapterDragOver(e, i)}
    >
      {chapter.desc.sv}
    </button>
  {/each}
</div>

<style>
  .chapters-container {
    grid-row: 1 / 3;
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

  .chapter-header {
    margin-bottom: 1rem;
  }

  .chapter-entry {
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
