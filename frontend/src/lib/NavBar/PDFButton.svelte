<script lang="ts">
  import {
    fetchPdf,
    loadingState,
    PDFState,
    set_states
  } from '$src/globalStates.svelte';
  import type { View } from '$src/types';
  import i18n from '$src/i18n.svelte';
  import NavButton from './NavButton.svelte';

  let {
    view = $bindable()
  }: {
    view: View;
  } = $props();
</script>

<NavButton
  onclick={() => (view = 'pdf')}
  class={view === 'pdf' ? 'selected' : ''}
>
  <img src="pdf.svg" alt={'PDF icon'} />
  <p>PDF</p>
  <button
    disabled={loadingState.loading || set_states.added_sets.length == 0}
    onclick={() => {
      // To prevent the old PDF from flickering when switching to PDF view, we "deload" it first.
      // If we are already in PDF view, it looks weird if we deload it since the entire iframe disappears.
      if (view != 'pdf') {
        PDFState.url = '';
      }
      fetchPdf();
      view = 'pdf';
    }}
  >
    {i18n.t('create_pdf')}
  </button>
</NavButton>

<style>
  img {
    height: 0.9rem;
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
