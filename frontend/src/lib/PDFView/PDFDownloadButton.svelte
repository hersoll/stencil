<script lang="ts">
  import { PDFState } from '$src/globalStates.svelte';
  import i18n from '$src/i18n.svelte';

  function downloadPDF(e: Event) {
    e.preventDefault();
    if (!PDFState.url) return;
    const link = document.createElement('a');
    link.href = PDFState.url;
    link.download = PDFState.fileName;
    link.click();
  }
</script>

<button
  disabled={!PDFState.url}
  onclick={downloadPDF}
  type="button"
  class="download-btn"
>
  {i18n.t('download')}
</button>

<style>
  .download-btn {
    font-size: 1.2rem;
    width: 10rem;
    box-shadow: var(--shadow-elevation-medium);
    &:disabled {
      background-color: var(--secondary);
      color: var(--text);
    }
  }
</style>
