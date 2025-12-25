<script lang="ts">
  import i18n from '$src/i18n.svelte';
  let theme = $state(localStorage.getItem('theme') || 'system');

  function setTheme(newTheme: string) {
    theme = newTheme;
    localStorage.setItem('theme', newTheme);
    document.documentElement.setAttribute('data-theme', newTheme);
  }
</script>

<div>
  <button
    id="theme-system"
    class:active_theme={theme === 'system'}
    onclick={() => setTheme('system')}
  >
    {i18n.t('theme_system')}
  </button>
  <button
    id="theme-dark"
    class:active_theme={theme === 'dark'}
    onclick={() => setTheme('dark')}
  >
    {i18n.t('theme_dark')}
  </button>
  <button
    id="theme-light"
    class:active_theme={theme === 'light'}
    onclick={() => setTheme('light')}
  >
    {i18n.t('theme_light')}
  </button>
</div>

<style>
  div {
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
    &.active_theme {
      color: var(--primary-text);
      cursor: default;
    }
  }
  @media (max-width: 50rem) {
    div {
      display: none;
    }
  }
</style>
