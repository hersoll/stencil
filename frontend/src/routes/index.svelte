<script>
  import LanguageSwitch from '../lib/LanguageSwitch.svelte';
  import PDFButton from '../lib/PDFButton.svelte';
  import i18n from '../i18n.svelte';
</script>

<main>
  {#if i18n.loading}
    <p>Laddar...</p>
  {:else}
    <PDFButton />
  {/if}
</main>
