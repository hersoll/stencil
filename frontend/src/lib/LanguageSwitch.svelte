<script>
  import i18n from '../i18n.svelte';
  let currentLang = $derived(i18n.currentLanguage);
</script>

<div>
  <button
    class={currentLang === 'sv' ? 'active_language' : ''}
    disabled={currentLang === 'sv'}
    onclick={() => i18n.setLanguage('sv')}>Svenska</button
  >
  <button
    class={currentLang === 'en' ? 'active_language' : ''}
    disabled={currentLang === 'en'}
    onclick={() => i18n.setLanguage('en')}>English</button
  >
</div>

<style>
  button {
    margin: 1em 0.25rem;
    padding: 0;
    background: none;
    color: var(--text);
    border: none;
    transition: color 0.25s;

    &:hover {
      color: var(--text-muted);
    }
    &:disabled {
      cursor: default;
    }
    &.active_language {
      color: var(--primary);
    }
  }
</style>
