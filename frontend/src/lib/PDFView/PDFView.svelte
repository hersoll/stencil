<script lang="ts">
  import { loadingState, PDFState } from '$src/globalStates.svelte';
  import PDFDisplay from './PDFDisplay.svelte';
  import i18n from '$src/i18n.svelte';
  import PDFDownloadButton from './PDFDownloadButton.svelte';
  import PDFNameInput from './PDFNameInput.svelte';
</script>

{#if PDFState.url}
  <form action="">
    <PDFDisplay />
    <div class="footer">
      <PDFNameInput />
      <PDFDownloadButton />
    </div>
  </form>
{:else if !loadingState.loading}
  <div class="text-container">
    <h2>{i18n.t('no_pdf_found')}</h2>
    <p>
      {i18n.t('create_pdf_instruction')} <strong>{i18n.t('create_pdf')}</strong>
    </p>
  </div>
{/if}

<style>
  form {
    height: 100%;
    width: 100%;
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
