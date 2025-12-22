<script>
  import i18n from '$src/i18n.svelte';
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
  div {
    position: absolute;
    left: 2em;
    background-color: var(--bg-light);
    padding: 1rem;
    border-radius: 1rem;
    box-shadow: var(--shadow-elevation-low);
  }
  button {
    margin: 0 0.25rem;
    padding: 0;
    background: none;
    color: var(--text-muted);
    border: none;
    transition: color 0.25s;

    &:hover {
      color: var(--text);
    }
    &:disabled {
      cursor: default;
    }
    &.active_language {
      color: var(--primary);
    }
  }
</style>
