<script lang="ts">
  import { defaultProblemEntry, type ProblemEntry } from './types';
  import { API_URL } from '$src/main';
  import ContextMenu from './ContextMenu.svelte';
  import ServerMessage from './ServerMessage.svelte';
  import ProblemList from './ProblemList.svelte';
  import EditingArea from './EditingArea.svelte';
  import { fly } from 'svelte/transition';

  let search = $state('');

  let emptyProblem: ProblemEntry = { ...defaultProblemEntry };

  let clickedProblem = $state<ProblemEntry>(emptyProblem);
  let activeProblem = $state<ProblemEntry | null>(null);

  let contextMenu: ContextMenu;
  let serverMessage: ServerMessage;

  function onClickOutsideList() {
    clickedProblem = emptyProblem;
  }

  function editProblem() {
    activeProblem = { ...clickedProblem };
  }

  function copyProblem() {
    activeProblem = { ...clickedProblem, id: -1 };
  }

  async function deleteProblem() {
    const response = await fetch(`${API_URL}/edit/problem`, {
      method: 'DELETE',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(clickedProblem)
    });

    serverMessage.show(response);
  }

  function handleProblemDrag(problem: ProblemEntry) {
    clickedProblem = problem;
  }

  function handleProblemClick(e: MouseEvent, problem: ProblemEntry) {
    clickedProblem = problem;
    contextMenu.show({ x: e.x, y: e.y });
    console.log(problem.id);
    console.log(clickedProblem.id);
  }
</script>

<main in:fly={{ y: 60, duration: 600 }}>
  <input
    class="search-bar"
    type="text"
    placeholder="Search"
    bind:value={search}
    onkeydown={e =>
      e.key === 'Enter' && (e.preventDefault(), e.currentTarget?.blur())}
  />
  <div class="major-grid">
    <ProblemList
      {handleProblemClick}
      {handleProblemDrag}
      {onClickOutsideList}
      {search}
    />
    <EditingArea {clickedProblem} bind:activeProblem />
  </div>
  <button class="clear-btn" onclick={(activeProblem = null)}>Clear</button>
</main>

<ContextMenu
  bind:this={contextMenu}
  editFunc={editProblem}
  copyFunc={copyProblem}
  deleteFunc={deleteProblem}
  {clickedProblem}
/>

<ServerMessage bind:this={serverMessage} />

<style>
  main {
    position: relative;
    margin-top: 2rem;
    padding: 2rem;
    border-radius: 2rem;
    background-color: var(--bg);
    box-shadow: var(--shadow-elevation-low);
  }

  .major-grid {
    display: grid;
    justify-content: start;
    grid-template-columns: auto auto;
    gap: 2rem;
  }

  .search-bar {
    width: 19rem;
    background-color: var(--bg-light);
    padding: 0.5rem;
    font-size: 1rem;
    border: none;
    border-radius: 0.5rem;
    margin-bottom: 2rem;
    box-shadow: var(--shadow-elevation-low);
  }

  .clear-btn {
    position: absolute;
    top: 2rem;
    right: 2rem;
    box-shadow: var(--shadow-elevation-low);
  }
</style>
