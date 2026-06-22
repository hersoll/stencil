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
</script>

<NavBar bind:course={activeCourseName} bind:view />

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

<style>
  h2 {
    font-size: 2rem;
    margin: 0;
    margin-bottom: 2rem;
  }
</style>
