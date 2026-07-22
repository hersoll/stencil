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
      <p class="mobile-caution">
        {i18n.t('mobile_pdf_caution')}
      </p>
      <PDFNameInput />
      <PDFDownloadButton />
    </div>
  </form>
{:else if !loadingState.loading}
  <div class="text-container">
    <h2>{i18n.t('no_pdf_found')}</h2>
    <p class="instruction desktop">
      {i18n.t('create_pdf_instruction')} <strong>{i18n.t('create_pdf')}</strong>
    </p>
    <p class="instruction mobile">
      {i18n.t('create_pdf_instruction_mobile')}
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
    border-top: 2px solid var(--border);
    .mobile-caution {
      display: none;
    }
  }

  .text-container {
    padding: 1rem;
    display: flex;
    height: 100dvh;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
  }

  strong {
    color: var(--primary-text);
  }

  .mobile {
    display: none;
  }

  @container body (width < 50rem) {
    .footer {
      flex-direction: column;
      align-items: center;

      .mobile-caution {
        display: block;
        font-size: clamp(0.7rem, 0.5226rem + 0.75vw, 0.9rem);
        font-style: italic;
      }
    }

    .instruction {
      &.desktop {
        display: none;
      }
      &.mobile {
        display: block;
      }
    }
  }
</style>
