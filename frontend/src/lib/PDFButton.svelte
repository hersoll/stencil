<script lang="ts">
  let loading = $state(false);
  let errorMessage = $state('');
  let pdfUrl = $state('');
  const API_URL = import.meta.env.VITE_API_URL || '/api';

  const fetchPdf = async (): Promise<void> => {
    loading = true;

    try {
      const response: Response = await fetch(`${API_URL}/pdf/example`);

      if (!response.ok) {
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

<div style="display: flex; flex-direction: column;">
  <button onclick={fetchPdf}>
    {loading ? 'Loading...' : 'Get Example PDF'}
  </button>

  {#if pdfUrl != ''}
    <iframe style="width: 400px; height: 800px;" src={pdfUrl} title="PDF Viewer"
    ></iframe>
  {/if}
</div>
