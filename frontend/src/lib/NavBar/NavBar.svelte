<script lang="ts">
  import CourseSelector from './CourseSelector.svelte';
  import type { View } from '../../types.ts';
  import SetDisplay from './SetDisplay.svelte';
  import PDFButton from './PDFButton.svelte';
  import AddSetsButton from './AddSetsButton.svelte';
  import LayoutButton from './LayoutButton.svelte';

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
  <AddSetsButton bind:view />
  <LayoutButton bind:view />
  <PDFButton bind:view />
  <SetDisplay />
</nav>

<style>
  nav {
    position: fixed;
    left: 0;
    top: 0;
    bottom: 0;
    width: var(--navbar-margin);
    padding: 0.5rem;

    display: flex;
    flex-direction: column;
    gap: 0.1rem;

    background-color: var(--bg);
    border-right: 2px solid lightgray;
  }
  .nav-header {
    flex: 0 0 auto;
    display: flex;
    padding-left: 0.5rem;
    justify-content: space-between;
    margin-bottom: 0.5rem;
    .home-link {
      margin: 0;
      color: var(--primary-text);
      font-size: 1.25rem;
      font-weight: 700;
      text-decoration: none;
    }
  }
</style>
