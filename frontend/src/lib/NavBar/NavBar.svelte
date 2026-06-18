<script lang="ts">
  import CourseSelector from './CourseSelector.svelte';
  import type { View } from '../../types.ts';
  import { set_states } from '$src/globalStates.svelte';
  import SetCard from './SetCard.svelte';

  let {
    course = $bindable(),
    view = $bindable()
  }: {
    course: string | null;
    view: View;
  } = $props();
</script>

<nav>
  <div class="nav-header">
    <a class="home-link" href="/">Stencil</a>
    <CourseSelector bind:course />
  </div>
  <button onclick={() => (view = 'add_set')}>Add set</button>
  <div class="sets-container">
    {#each set_states.added_sets as set}
      <SetCard set={set.set} id={set.id} />
    {/each}
  </div>
  <div class="bottom-buttons">
    <button onclick={() => (view = 'layout')}>Layout</button>
    <button onclick={() => (view = 'pdf')}>View PDF</button>
  </div>
</nav>

<style>
  nav {
    position: fixed;
    left: 0;
    top: 0;
    bottom: 0;
    width: 20rem;
    padding: 1rem;

    display: flex;
    flex-direction: column;
    gap: 1rem;

    background-color: var(--bg-dark);
    box-shadow: var(--shadow-elevation-low);

    > :nth-child(1),
    > :nth-child(2),
    > :nth-child(4),
    > :nth-child(5) {
      flex: 0 0 auto;
    }
  }
  .nav-header {
    display: flex;
    justify-content: space-between;
    .home-link {
      margin: 0;
      color: var(--text);
      font-size: 1.2rem;
      font-weight: 700;
      text-decoration: none;
    }
  }

  .sets-container {
    flex: 1 1 auto;
    overflow-y: auto;
    background-color: var(--bg);
    border-radius: 0.5rem;
  }

  .bottom-buttons {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
