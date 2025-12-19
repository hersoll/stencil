<script lang="ts">
  import { onMount } from 'svelte';
  import i18n from './i18n.svelte';
  import PDFButton from './lib/PDFButton.svelte';
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

    <PDFButton />
  {/if}
</main>

<style>
</style>
