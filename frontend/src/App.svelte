<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { onDestroy, onMount } from 'svelte';
  import { loadingState, error } from '$src/globalStates.svelte';
  import NavBar from '$src/lib/NavBar/NavBar.svelte';
  import type { View } from '$src/types';
  import ErrorView from './lib/ErrorView.svelte';
  import AddSetView from './lib/AddSetView/AddSetView.svelte';
  import PDFView from './lib/PDFView/PDFView.svelte';
  import EditSetView from './lib/EditSetView/EditSetView.svelte';
  import LoadingView from './lib/LoadingView.svelte';
  import StartUpView from './lib/StartUpView.svelte';

  let activeCourseName: string | null = $state(
    localStorage.getItem('course') || null
  );

  // Keeps track of which page to show
  let view: View = $state('addSet');
  let navbarOpen = $state(true);

  let showLoadingMessage = $state(false);
  let loadingTimeout: ReturnType<typeof setTimeout> | null = $state(null);
  const LOADING_DELAY = 600;

  $effect(() => {
    if (!loadingState.loading) {
      showLoadingMessage = false;
      return;
    }

    const timeout = setTimeout(() => {
      showLoadingMessage = true;
    }, LOADING_DELAY);

    return () => {
      clearTimeout(timeout);
      showLoadingMessage = false;
    };
  });
  onDestroy(() => {
    if (loadingTimeout) clearTimeout(loadingTimeout);
  });

  onMount(async () => {
    await i18n.init();
  });

  // Newly added strings:
  // - startup_message
  // - create_set_instruction
  // - no_pdf_found
  // - support_me
  // Probably missed some...
</script>

<NavBar bind:course={activeCourseName} bind:view bind:navbarOpen />

<main class={navbarOpen ? 'nav-open' : 'nav-closed'}>
  {#if error.message}
    <ErrorView />
  {:else if showLoadingMessage}
    <LoadingView />
  {:else if !activeCourseName}
    <StartUpView />
  {:else if view === 'addSet'}
    <AddSetView courseName={activeCourseName} />
  {:else if view === 'layout'}
    <h2>Layout View</h2>
  {:else if view === 'pdf'}
    <PDFView />
  {:else if view === 'editSet'}
    <EditSetView />
  {/if}
</main>

<style>
  main {
    display: flex;
    height: 100%;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    container: main / inline-size;

    &.nav-open {
      margin-left: var(--navbar-margin);
    }
    &.nav-closed {
      margin-left: var(--navbar-closed-margin);
    }
  }

  h2 {
    font-size: 2rem;
    margin: 0;
    margin-bottom: 2rem;
  }
</style>
