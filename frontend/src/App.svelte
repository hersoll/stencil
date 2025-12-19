<script lang="ts">
  import './i18n.ts';
  import { _, locale, isLoading } from 'svelte-i18n';
  import { setLanguage, availableLocales } from './i18n';
  import FetchButton from './lib/FetchButton.svelte';
  import PDFButton from './lib/PDFButton.svelte';
</script>

<main>
  {#if $isLoading}
    <p>Laddar...</p>
  {:else}
    <nav>
      <select
        value={$locale}
        onchange={e => setLanguage(e.currentTarget.value)}
      >
        {#each availableLocales as lang}
          <option value={lang}>{lang.toLocaleUpperCase()}</option>
        {/each}
      </select>
      <p>Language test: {$_('document_option_color')}</p>
    </nav>
    <h1>Stencil</h1>

    <FetchButton />
    <PDFButton />
  {/if}
</main>

<style>
</style>
