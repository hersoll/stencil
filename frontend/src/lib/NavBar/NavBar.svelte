<script lang="ts">
  import CourseSelector from './CourseSelector.svelte';
  import type { View } from '../../types.ts';
  import SetDisplay from './SetDisplay.svelte';
  import PDFButton from './PDFButton.svelte';
  import AddSetsButton from './AddSetsButton.svelte';
  import LayoutButton from './LayoutButton.svelte';
  import NavBarFooter from './NavBarFooter.svelte';
  import SettingsMenu from './SettingsMenu.svelte';

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
    <div class="mobile-settings">
      <SettingsMenu name="header" />
    </div>
    <a class="home-link {navbarOpen ? 'open' : 'closed'}" href="/"
      >{navbarOpen ? 'stencil.nu' : 'stencil'}</a
    >
    {#if navbarOpen}
      <CourseSelector bind:course />
    {/if}
  </div>
  <div class="btn-container">
    <AddSetsButton bind:view {navbarOpen} />
    <LayoutButton bind:view {navbarOpen} />
    <PDFButton bind:view {navbarOpen} />
  </div>
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
    z-index: 10;

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
    align-items: center;
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
  .btn-container {
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
  }
  .mobile-settings {
    display: none;
  }
  @container (width < 50rem) {
    nav {
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      width: 100%;
      height: 5.5rem;
      border-right: none;
      border-bottom: 2px solid var(--border);
    }

    .nav-header {
      width: 100%;
      display: grid;
      grid-template-columns: 1fr 10rem 1fr;
      padding-left: 0;
      .home-link {
        text-align: center;
        font-size: 1rem;
      }
    }

    .btn-container {
      display: grid;
      grid-template-columns: auto auto auto;
      gap: 0.1rem;
      width: 100%;
    }
    .mobile-settings {
      display: inline;
    }
  }
</style>
