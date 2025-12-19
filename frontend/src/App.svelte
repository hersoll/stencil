<script lang="ts">
  import './i18n.ts';
  import {
    availableLanguages,
    currentLanguage,
    setLanguage,
    t,
    translationLoading,
    initI18n
  } from './i18n';
  import FetchButton from './lib/FetchButton.svelte';
  import PDFButton from './lib/PDFButton.svelte';
  import { onMount } from 'svelte';

  onMount(async () => {
    await initI18n();
  });
</script>

<main>
  {#if $translationLoading}
    <p>Laddar...</p>
  {:else}
    <nav>
      <select
        value={$currentLanguage}
        onchange={e => setLanguage(e.currentTarget.value)}
      >
        {#each availableLanguages as lang}
          <option value={lang}>{lang.toLocaleUpperCase()}</option>
        {/each}
      </select>
      <p>Language test: {$t('document_option_color')}</p>
    </nav>
    <h1>Stencil</h1>

    <FetchButton />
    <PDFButton />
  {/if}
</main>

<style>
</style>
