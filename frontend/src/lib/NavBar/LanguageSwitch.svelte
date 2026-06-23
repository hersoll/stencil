<script>
  import i18n from '$src/i18n.svelte';
  let currentLang = $derived(i18n.currentLanguage);
</script>

<div class="language-switch">
  <p class="language-label">{i18n.t('language')}</p>
  <div class="language-buttons">
    <button
      class={currentLang === 'sv' ? 'active_language' : ''}
      disabled={currentLang === 'sv'}
      onclick={() => i18n.setLanguage('sv')}>Sve</button
    >
    <button
      class={currentLang === 'en' ? 'active_language' : ''}
      disabled={currentLang === 'en'}
      onclick={() => i18n.setLanguage('en')}>Eng</button
    >
  </div>
</div>

<style>
  .language-switch {
    display: flex;
    align-items: center;
    background: none;
    padding: 0;
    gap: 0.75rem;
  }
  .language-label {
    font-weight: 600;
  }

  .language-buttons {
    display: flex;
    align-items: center;
    border-radius: 1rem;
    border: 1px solid gray;
    overflow: hidden;
  }
  button {
    margin: 0;
    padding: 0.45rem 0.6rem;
    background: none;
    border: none;
    border-radius: 0;

    &:hover {
      background-color: var(--highlight);
    }
    &:active {
      background: none;
    }

    &.active_language {
      background-color: var(--highlight);
      color: var(--text);
      cursor: default;
    }
  }
</style>
