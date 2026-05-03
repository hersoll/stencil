<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from '../ServerMessage.svelte';
  import { type Entry, type ProblemEntryRaw, type TopicEntry } from '../types';

  let {
    topic = $bindable(),
    problem_ids = $bindable(),
    serverMessage,
    draggedEntry,
    dropPriority = $bindable(),
    parentDraggedOver = $bindable()
  }: {
    topic: TopicEntry;
    problem_ids: number[];
    serverMessage: ServerMessage;
    draggedEntry: Entry | null;
    dropPriority: boolean;
    parentDraggedOver: boolean;
  } = $props();
  let draggedOver = $state(false);
  let dragDepth = $state(0);
  let draggedProblem = $state<ProblemEntryRaw | null>(null);
  let draggedIndex = $state(-1);
  let problems = $state<ProblemEntryRaw[]>([]);

  function inProblems(problem: ProblemEntryRaw): boolean {
    return problems.find(p => p.id == problem.id) !== undefined;
  }

  function addProblem(problem: ProblemEntryRaw) {
    problems.push(problem);
    problem_ids.push(problem.id);
  }

  function removeProblem(problem: ProblemEntryRaw) {
    problems = problems.filter(p => p.id !== problem.id);
    problem_ids = problem_ids.filter(id => id !== problem.id);
  }

  // The area is entered while dragging
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
    // Only allow non-empty problems
    if (draggedEntry?.kind === 'problem' && draggedEntry?.id >= 0) {
      if (!inProblems(draggedEntry)) {
        addProblem(draggedEntry);
      }
    } else {
      dropPriority = false;
    }
    dragDepth--;
    draggedOver = false;
  }

  function handleProblemDragStart(problem: ProblemEntryRaw, index: number) {
    draggedProblem = problem;
    draggedIndex = index;
  }

  function handleProblemDragOver(e: DragEvent, targetIndex: number) {
    e.preventDefault();

    if (draggedIndex === -1 || draggedIndex === targetIndex) return;

    const newOrder = [...problems];
    const [removed_entry] = newOrder.splice(draggedIndex, 1);
    newOrder.splice(targetIndex, 0, removed_entry);
    problems = newOrder;
    problem_ids = problems.map(p => p.id);

    draggedIndex = targetIndex;
  }

  let problemWillBeRemoved = $state(false);
  function handleProblemDrag() {
    if (parentDraggedOver) {
      problemWillBeRemoved = false;
    } else if (!parentDraggedOver) {
      problemWillBeRemoved = true;
    }
  }

  function handleProblemDragEnd(problem: ProblemEntryRaw) {
    if (problemWillBeRemoved) {
      removeProblem(problem);
    }
    problemWillBeRemoved = false;
    draggedProblem = null;
    draggedIndex = -1;
  }

  async function fetchProblems() {
    let res = await fetch(`${API_URL}/edit/problem/ids`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(problem_ids)
    });

    if (res.ok) {
      problems = await res.json();
    } else {
      serverMessage.show(res);
    }
  }

  $effect(() => {
    if (problems.length == 0) fetchProblems();
  });
</script>

<div
  role="status"
  class="problems-container"
  class:parent-dragged={parentDraggedOver}
  class:available={draggedEntry?.kind === 'problem'}
  class:droppable={draggedEntry?.kind === 'problem' && draggedOver}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
>
  <h3 class="problem-header">Problems</h3>
  {#each problems as problem, i (problem.id)}
    <button
      class="problem-entry no-select"
      class:parent-dragged={parentDraggedOver}
      class:available={draggedEntry?.kind === 'problem'}
      id={problem.name}
      draggable="true"
      in:fly={{ y: 40, duration: 400, delay: 20 * i }}
      ondrag={handleProblemDrag}
      ondragstart={() => handleProblemDragStart(problem, i)}
      ondragover={e => handleProblemDragOver(e, i)}
      ondragend={() => handleProblemDragEnd(problem)}
    >
      <p>{problem.difficulty}</p>
      <p>{problem.module}</p>
      <p>
        {problem.desc.sv}
      </p>
    </button>
  {/each}
</div>

<style>
  .problems-container {
    grid-row: 1/3;
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
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

  .problem-header {
    margin-bottom: 1rem;
  }

  .problem-entry {
    display: grid;
    grid-template-columns: 0.5rem min-content 1fr;
    gap: 1rem;
    justify-items: self-start;
    border: 2px solid var(--bg-dark);
    box-shadow: var(--shadow-elevation-low);
    width: 100%;
    padding: 0.5rem;

    p {
      font-size: 0.8rem;
      text-align: left;
      width: 100%;
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }

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
