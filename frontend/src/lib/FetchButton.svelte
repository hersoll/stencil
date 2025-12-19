<script lang="ts">
  let data = $state(null);
  let loading = $state(false);
  let lang_code = 'sv';

  const API_URL = import.meta.env.VITE_API_URL || '/api';

  async function loadTranslations() {
    loading = true;
    const res = await fetch(`${API_URL}/translations/${lang_code}`);

    if (!res.ok) {
      throw new Error(`HTTP error! status: ${res.status}`);
    }

    data = await res.json();
    loading = false;
  }

  // Loading on startup
  // $effect(() => {
  //   fetch(`${API_URL}/translation/sv`)
  //     .then(res => res.json())
  //     .then(json => {
  //       data = json;
  //       loading = false;
  //     });
  // });
</script>

<button onclick={loadTranslations}> Fetch </button>

{#if loading}
  <p>Loading...</p>
{:else if data}
  <pre>{JSON.stringify(data, null, 2)}</pre>
{/if}
