<script lang="ts">
  import {
    fetchPdf,
    loadingState,
    PDFState,
    setState
  } from '$src/globalStates.svelte';
  import type { View } from '$src/types';
  import i18n from '$src/i18n.svelte';
  import NavButton from './NavButton.svelte';

  let {
    view = $bindable(),
    navbarOpen
  }: {
    view: View;
    navbarOpen: boolean;
  } = $props();

  function switchView() {
    view = 'pdf';
  }

  function generateAndSwitch() {
    // To prevent the old PDF from flickering when switching to PDF view, we "deload" it first.
    // If we are already in PDF view, it looks weird if we deload it since the entire iframe disappears.
    if (view != 'pdf') {
      PDFState.url = '';
    }
    fetchPdf();
    view = 'pdf';
  }
</script>

<NavButton
  onclick={navbarOpen ? switchView : generateAndSwitch}
  class="{view === 'pdf' ? 'selected' : ''} {navbarOpen
    ? 'nav-open'
    : 'nav-closed'}"
  disabled={!navbarOpen &&
    (loadingState.loading || setState.addedSets.length == 0)}
>
  <div class="svg-icon"></div>
  {#if navbarOpen}
    <p>PDF</p>
    <button
      disabled={loadingState.loading || setState.addedSets.length == 0}
      onclick={generateAndSwitch}
    >
      {i18n.t('create_pdf')}
    </button>
  {/if}
</NavButton>

<style>
  .svg-icon {
    -webkit-mask: url('pdf.svg') center / contain no-repeat;
    mask: url('pdf.svg') center / contain no-repeat;
  }

  button {
    position: absolute;
    right: 0;

    width: 7rem;
    height: 100%;
    padding: 0 0.8rem;

    background: none;
    border-radius: 1rem;

    font-size: 0.9rem;
    transition: box-shadow 0.3s background-color 0.15s;

    &:disabled {
      border: 1px solid var(--bg-dark);
    }

    &:enabled {
      background-color: var(--primary);
      box-shadow: var(--shadow-elevation-low);
      color: var(--text-in-primary);
      border: none;
      &:active {
        background-color: var(--secondary);
        box-shadow: none;
      }
      &:hover {
        box-shadow: var(--shadow-elevation-medium);
      }
    }
  }
</style>
