<script lang="ts">
  import i18n from './i18n.svelte';
  import FetchButton from './lib/FetchButton.svelte';
  import PDFButton from './lib/PDFButton.svelte';
  import { onMount } from 'svelte';
  import LanguageSwitch from './lib/LanguageSwitch.svelte';

  onMount(async () => {
    await i18n.init();
  });
</script>

<main>
  {#if i18n.loading}
    <p>Laddar...</p>
  {:else}
    <nav>
      <LanguageSwitch />
    </nav>
    <h1>Stencil</h1>

    <FetchButton />
    <PDFButton />
  {/if}
</main>

<style>
</style>
