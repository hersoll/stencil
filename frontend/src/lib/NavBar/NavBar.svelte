<script lang="ts">
  import CourseSelector from './CourseSelector.svelte';
  import type { View } from '../../types.ts';
  import SetDisplay from './SetDisplay.svelte';
  import PDFButton from './PDFButton.svelte';
  import AddSetsButton from './AddSetsButton.svelte';
  import LayoutButton from './LayoutButton.svelte';
  import NavBarFooter from './NavBarFooter.svelte';

  let {
    course = $bindable(),
    view = $bindable(),
    navbarOpen = $bindable()
  }: {
    course: string;
    view: View;
    navbarOpen: boolean;
  } = $props();
</script>

<nav class={navbarOpen ? 'open' : 'closed'}>
  <div class="nav-header {navbarOpen ? 'open' : 'closed'}">
    <a class="home-link {navbarOpen ? 'open' : 'closed'}" href="/"
      >{navbarOpen ? 'stencil.nu' : 'stencil'}</a
    >
    {#if navbarOpen}
      <CourseSelector bind:course />
    {/if}
  </div>
  <AddSetsButton bind:view {navbarOpen} />
  <LayoutButton bind:view {navbarOpen} />
  <PDFButton bind:view {navbarOpen} />
  <SetDisplay bind:view {navbarOpen} />
  <NavBarFooter bind:navbarOpen />
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
    border-right: 2px solid var(--border);

    &.closed {
      width: var(--navbar-closed-margin);
    }
  }
  .nav-header {
    flex: 0 0 auto;
    display: flex;
    padding-left: 0.5rem;
    justify-content: space-between;
    margin-bottom: 0.5rem;

    &.closed {
      padding: 0;
      margin-left: -0.25rem;
      height: 2rem;
      align-items: center;
    }

    .home-link {
      margin: 0;
      color: var(--primary-text);
      font-size: 1.25rem;
      font-weight: 700;
      text-decoration: none;
      &.closed {
        font-size: 0.7rem;
      }
    }
  }
</style>
