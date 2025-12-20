<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { API_URL } from '$src/main';
  let { sets } = $props();
  let loading = $state(false);
  let errorMessage = $state('');
  let pdfUrl = $state('');

  const fetchPdf = async (): Promise<void> => {
    loading = true;

    try {
      const response: Response = await fetch(`${API_URL}/pdf`, {
        method: 'POST',
        headers: {
          'Content-Type': 'applications/json'
        },
        body: JSON.stringify(sets)
      });

      if (!response.ok) {
        console.log(response.statusText);
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      // Get the PDF as a blob (binary data)
      const blob: Blob = await response.blob();
      pdfUrl = URL.createObjectURL(blob);
    } catch (e) {
      errorMessage = e instanceof Error ? e.message : 'An error occurred';
    } finally {
      loading = false;
    }
  };
</script>

<div class="pdf_container">
  <button onclick={fetchPdf}>
    {loading ? 'Loading...' : i18n.t('create_pdf')}
  </button>

  {#if pdfUrl != ''}
    <iframe src={pdfUrl} title="PDF Viewer"></iframe>
  {/if}
</div>

<style>
  .pdf_container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2em;

    padding: 2em;
    width: 600px;
    height: 900px;
  }

  iframe {
    width: 600px;
    height: 800px;
  }
</style>
