<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { API_URL } from '$src/main';
  import { error, sets } from '$src/states.svelte';

  let loading = $state(false);
  let errorMessage = $state('');
  let pdfUrl = $state('');

  const fetchPdf = async (): Promise<void> => {
    loading = true;

    const mapped_sets = sets.set_states.map(set => set.set);
    try {
      const response: Response = await fetch(`${API_URL}/pdf`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ sets: mapped_sets })
      });

      if (!response.ok) {
        let text = await response.text();
        error.message = `Status: ${response.status} \n${text}`;
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
  <button onclick={fetchPdf} disabled={sets.set_states.length == 0}>
    {loading ? 'Loading...' : i18n.t('create_pdf')}
  </button>

  {#if pdfUrl != ''}
    <iframe src={pdfUrl} title="PDF Viewer"></iframe>
  {/if}
</div>

<style>
  button {
    background-color: var(--primary);
    color: var(--text);
    color-scheme: light;
    transition:
      color 0.4s,
      background-color 0.4s;
    &:disabled {
      color-scheme: dark;
      cursor: not-allowed;
      background-color: var(--bg-light);
      color: gray;
    }
  }
  .pdf_container {
    display: flex;
    margin: 0 auto;
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
