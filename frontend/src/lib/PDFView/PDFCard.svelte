<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { API_URL } from '$src/main';
  import {
    error,
    set_states,
    document_options
  } from '$src/globalStates.svelte';
  import { fade, fly } from 'svelte/transition';

  let loading = $state(false);
  let pdfUrl = $state('');

  //Variable for the scroll target to bind to
  let scrollElement: HTMLElement;

  const fetchPdf = async (): Promise<void> => {
    loading = true;

    const mapped_sets = set_states.added_sets.map(set => set.set);
    try {
      const response: Response = await fetch(`${API_URL}/pdf`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ sets: mapped_sets, document_options })
      });

      if (!response.ok) {
        let text = await response.text();
        error.message = `Status: ${response.status} \n${text}`;
        return;
      }

      // Get the PDF as a blob (binary data)
      const blob: Blob = await response.blob();
      pdfUrl = URL.createObjectURL(blob);
    } catch (e) {
      error.message = e instanceof Error ? e.message : 'An error occurred';
    } finally {
      loading = false;
    }
  };

  function downloadPDF(e: Event) {
    e.preventDefault();
    if (!pdfUrl) return;
    const link = document.createElement('a');
    link.href = pdfUrl;
    link.download = 'stencil.pdf';
    link.click();
  }

  $effect(() => {
    if (pdfUrl != '') {
      scrollElement.scrollIntoView({
        behavior: 'smooth',
        block: 'start'
      });
    }
  });
</script>

<form action="">
  <div
    class="pdf-container {loading || pdfUrl != '' ? 'expanded' : ''}"
    bind:this={scrollElement}
    in:fly={{ y: 60, duration: 600, delay: 150 }}
    out:fly={{ x: 60, duration: 600, delay: 100 }}
  >
    <div class="title-container">
      <label for="title">{i18n.t('document_option_title')}:</label>
      <input
        name="title"
        placeholder={i18n.t('title_placeholder')}
        type="text"
        bind:value={document_options.title}
      />
    </div>

    <button class="primary" onclick={fetchPdf} disabled={loading} type="submit">
      {i18n.t('create_pdf')}
    </button>
    {#if pdfUrl != ''}
      <button
        disabled={!pdfUrl}
        onclick={downloadPDF}
        type="button"
        class="download-btn"
      >
        {i18n.t('download')}
      </button>
      <div class="iframe-container">
        <iframe
          src={pdfUrl}
          title="PDF Viewer"
          in:fly={{ y: 20, duration: 600 }}
        ></iframe>

        {#if loading}
          <div
            class="loading-overlay"
            transition:fade={{ duration: 300 }}
          ></div>
        {/if}
      </div>
    {/if}
  </div>
</form>

<style>
  .pdf-container {
    position: relative;
    margin-top: 1rem;
    padding: 1rem;
    border-radius: 2rem;
    background-color: var(--bg);
    box-shadow: var(--shadow-elevation-low);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2rem;
    height: 12rem;
    transition: height 0.5s;

    &.expanded {
      height: 73rem;
    }
  }

  .title-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;

    label {
      font-size: 1.5rem;
      font-weight: 600;
    }

    input {
      font-size: 1.2rem;
      width: 20rem;
      padding: 0.5rem;
      border: none;
      border-radius: 0.5rem;
      box-shadow: var(--shadow-elevation-low);
    }
  }

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
    right: 2rem;
    top: 8rem;
  }

  .iframe-container {
    position: relative;
    width: 100%;
    height: 60rem;

    iframe {
      width: 100%;
      height: 100%;
      border-radius: 1rem;
    }

    .loading-overlay {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background-color: rgba(0, 0, 0, 0.4);
      pointer-events: none; /* Allows clicks through */
    }
  }
</style>
