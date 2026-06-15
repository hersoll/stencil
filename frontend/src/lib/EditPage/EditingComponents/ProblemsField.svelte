<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from '../ServerMessage.svelte';
  import { type Entry, type ProblemEntryRaw, type TopicEntry } from '../types';

  let {
    topic = $bindable(),
    serverMessage,
    draggedEntry,
    dropPriority = $bindable(),
    parentDraggedOver = $bindable()
  }: {
    topic: TopicEntry;
    serverMessage: ServerMessage;
    draggedEntry: Entry | null;
    dropPriority: boolean;
    parentDraggedOver: boolean;
  } = $props();
  let draggedOver = $state(false);
  let dragDepth = $state(0);
  let draggedProblem = $state<ProblemEntryRaw | null>(null);
  let draggedIndex = $state(-1);
  let problems_to_show = $state<ProblemEntryRaw[]>([]);

  function inProblems(problem: ProblemEntryRaw): boolean {
    return problems_to_show.find(p => p.id == problem.id) !== undefined;
  }

  function addProblem(problem: ProblemEntryRaw) {
    problems_to_show.push(problem);
    topic.problems.push({
      problem_id: problem.id,
      topic_id: topic.id,
      absolute_difficulty: 4,
      relative_difficulty: 4
    });
  }

  function removeProblem(problem: ProblemEntryRaw) {
    problems_to_show = problems_to_show.filter(p => p.id !== problem.id);
    topic.problems = topic.problems.filter(
      data => data.problem_id !== problem.id
    );
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

    const newOrder = [...problems_to_show];
    const newOrder_in_data = [...topic.problems];
    const [removed_entry] = newOrder.splice(draggedIndex, 1);
    const [removed_data] = newOrder_in_data.splice(draggedIndex, 1);
    newOrder.splice(targetIndex, 0, removed_entry);
    newOrder_in_data.splice(targetIndex, 0, removed_data);
    problems_to_show = newOrder;
    topic.problems = newOrder_in_data;

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
    let res = await fetch(`${API_URL}/edit/problem/from_topic`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(topic.id)
    });

    if (res.ok) {
      problems_to_show = await res.json();
    } else {
      serverMessage.show(res);
    }
  }

  $effect(() => {
    if (topic.problems || problems_to_show.length == 0) fetchProblems();
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
  <div class="header-container">
    <h3>Problems</h3>
    <h4>Absolute</h4>
    <h4>Relative</h4>
  </div>
  {#each topic.problems as problem, i (problem.problem_id)}
    {@const problem_data = problems_to_show.find(
      problem_data => problem_data.id === problem.problem_id
    )}
    {#if problem_data}
      <button
        class="problem-entry no-select"
        class:parent-dragged={parentDraggedOver}
        class:available={draggedEntry?.kind === 'problem'}
        id={problem_data.name}
        draggable="true"
        in:fly={{ y: 40, duration: 400, delay: 20 * i }}
        ondrag={handleProblemDrag}
        ondragstart={() => handleProblemDragStart(problem_data, i)}
        ondragover={e => handleProblemDragOver(e, i)}
        ondragend={() => handleProblemDragEnd(problem_data)}
      >
        <p>
          {problem_data.desc.sv}
        </p>
        <input type="number" min="1" bind:value={problem.absolute_difficulty} />
        <input type="number" min="1" bind:value={problem.relative_difficulty} />
      </button>
    {/if}
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

  .header-container {
    display: grid;
    grid-template-columns: 1fr 5rem 3rem;
    text-align: left;
    margin-bottom: 1rem;
    padding-right: 1rem;
  }

  .problem-entry {
    display: grid;
    grid-template-columns: 1fr 4.5rem 3rem;
    text-align: left;
    justify-items: self-start;
    align-items: center;
    border: 2px solid var(--bg-dark);
    box-shadow: var(--shadow-elevation-low);
    width: 100%;
    padding: 0.5rem;

    p {
      font-size: 0.9rem;
      text-align: left;
      width: 100%;
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }

    input {
      width: 2.5rem;
      font-size: 0.9rem;
      padding: 0.1rem;
      border: none;
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
