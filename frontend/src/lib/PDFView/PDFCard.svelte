<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { PDFState } from '$src/globalStates.svelte';
  import { fly } from 'svelte/transition';

  function downloadPDF(e: Event) {
    e.preventDefault();
    if (!PDFState.url) return;
    const link = document.createElement('a');
    link.href = PDFState.url;
    link.download = 'stencil.pdf';
    link.click();
  }
</script>

<form action="" in:fly={{ y: -60, duration: 400 }}>
  <div class="iframe-container">
    <iframe src={PDFState.url} title="PDF Viewer"></iframe>
  </div>
  <button
    disabled={!PDFState.url}
    onclick={downloadPDF}
    type="button"
    class="download-btn"
  >
    {i18n.t('download')}
  </button>
</form>

<style>
  button {
    font-size: 1.2rem;
    width: 15rem;
    box-shadow: var(--shadow-elevation-medium);
    &:disabled {
      background-color: var(--secondary);
      color: var(--text);
    }
  }
  .download-btn {
    position: absolute;
    right: 3rem;
    bottom: 3rem;
  }

  .iframe-container {
    position: relative;
    width: 100%;
    height: 100vh;
    padding: 1rem;

    iframe {
      width: 100%;
      height: 100%;
      border-radius: 1rem;
    }
  }
</style>
