<script lang="ts">
  import { loadingState, PDFState } from '$src/globalStates.svelte';
  import PDFDisplay from './PDFDisplay.svelte';
  import i18n from '$src/i18n.svelte';
  import PDFDownloadButton from './PDFDownloadButton.svelte';
  import PDFNameInput from './PDFNameInput.svelte';
</script>

<main>
  {#if PDFState.url}
    <PDFDisplay />
    <div class="footer">
      <PDFNameInput />
      <PDFDownloadButton />
    </div>
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
    display: flex;
    flex-direction: column;
  }

  .footer {
    display: flex;
    justify-content: center;
    flex: 0 0 auto;
    gap: 1rem;
    padding: 1rem;
    border-top: 2px solid lightgray;
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
