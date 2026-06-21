<script lang="ts">
  import { loadingState, PDFState } from '$src/globalStates.svelte';
  import PDFCard from './PDFCard.svelte';
  import i18n from '$src/i18n.svelte';
</script>

<main>
  {#if PDFState.url}
    <PDFCard />
  {:else if !loadingState.loading}
    <div class="text-container">
      <h2>Du har inte skapat någon PDF än!</h2>
      <p>
        Lägg till minst en sektion och klicka på <strong
          >{i18n.t('create_pdf')}</strong
        >
      </p>
    </div>
  {/if}
</main>

<style>
  main {
    height: 100%;
  }
  .text-container {
    display: flex;
    height: 100dvh;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  strong {
    color: var(--primary-text);
  }
</style>
