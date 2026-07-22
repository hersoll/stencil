<script lang="ts">
  import { setState } from '$src/globalStates.svelte';
  import AnswerEditor from './AnswerEditor.svelte';
  import DocumentOptions from './DocumentOptions.svelte';
  import SetEditor from './SetEditor.svelte';
  import i18n from '$src/i18n.svelte';
  import type { View } from '$src/types';

  let { isActive, view = $bindable() }: { isActive: boolean; view: View } =
    $props();
</script>

<div class="editor-container {isActive ? 'open' : 'closed'}">
  {#each setState.addedSets, i}
    <SetEditor bind:set={setState.addedSets[i]} bind:view />
  {/each}
  <AnswerEditor />
  <div class="mobile document-options card">
    <h2 class="document-options-heading">{i18n.t('document_options')}</h2>
    <DocumentOptions />
  </div>
</div>

<style>
  .editor-container {
    flex: 0 0 auto;
    width: 32.5rem;
  }

  @container main (width < 70rem) {
    .editor-container.open {
      width: 100%;
    }
    .editor-container.closed {
      display: none;
    }

    .editor-container {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 2rem;
    }
  }

  .document-options {
    display: none;
  }

  @container body (width < 50rem) {
    .editor-container {
      padding: 1rem;
    }

    .document-options {
      display: flex;
      flex-direction: column;
      width: 100%;
      max-width: 30rem;
      row-gap: 0.3rem;

      .document-options-heading {
        margin-bottom: 0.4rem;
        font-size: 1.2rem;
      }
    }
  }
</style>
