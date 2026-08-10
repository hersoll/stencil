<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { onDestroy, onMount } from 'svelte';
  import {
    loadingState,
    error,
    setDocumentOptions,
    setDefaultFormattingOptions,
    setDefaultProblemOptions,
    documentOptions,
    setDefaultDocumentOptions
  } from '$src/globalStates.svelte';
  import NavBar from '$src/lib/NavBar/NavBar.svelte';
  import type { View, DocumentOptions } from '$src/types';
  import ErrorView from './lib/ErrorView.svelte';
  import AddSetView from './lib/AddSetView/AddSetView.svelte';
  import PDFView from './lib/PDFView/PDFView.svelte';
  import EditSetView from './lib/EditSetView/EditSetView.svelte';
  import LoadingView from './lib/LoadingView.svelte';
  import StartUpView from './lib/StartUpView.svelte';
  import LayoutView from './lib/LayoutView/LayoutView.svelte';
  import { API_URL } from './main';

  let activeCourseName: string = $state(
    localStorage.getItem('course') || 'mat1b'
  );

  // Keeps track of which page to show
  let view: View = $state('addSet');
  let navbarOpen = $state(true);

  let showLoadingMessage = $state(false);
  let loadingTimeout: ReturnType<typeof setTimeout> | null = $state(null);
  const LOADING_DELAY = 600;

  async function fetchDefaults() {
    const response: Response = await fetch(`${API_URL}/defaults`);

    if (!response.ok) {
      let text = await response.text();
      error.message = `Status: ${response.status} \n${text}`;
      return;
    }

    const { formatting_options, problem_options, document_options } =
      await response.json();
    setDefaultDocumentOptions(document_options);
    setDocumentOptions(document_options);
    setDefaultFormattingOptions(formatting_options);
    setDefaultProblemOptions(problem_options);
  }

  function loadLocalStorage() {
    let localDocumentOptionsString = localStorage.getItem('document_options');
    if (localDocumentOptionsString) {
      let localDocumentOptions: DocumentOptions = JSON.parse(
        localDocumentOptionsString
      );
      setDocumentOptions(localDocumentOptions);
    }
  }

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
    await fetchDefaults();
    loadLocalStorage();
  });
</script>

<svelte:head>
  <title>{i18n.t('html_title')}</title>
</svelte:head>
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
    <LayoutView bind:view />
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

  @container (width < 50rem) {
    main {
      padding-top: 5rem;
      &.nav-open {
        margin-left: 0;
      }
      &.nav-closed {
        margin-left: 0;
      }
    }
  }
</style>
