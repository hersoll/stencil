<script lang="ts">
  let theme = $state(localStorage.getItem('theme') || 'system');
  import i18n from '$src/i18n.svelte';
  import ThemeDarkIcon from '../SVGIcons/ThemeDarkIcon.svelte';
  import ThemeLightIcon from '../SVGIcons/ThemeLightIcon.svelte';
  import ThemeSystemIcon from '../SVGIcons/ThemeSystemIcon.svelte';

  function setTheme(newTheme: string) {
    theme = newTheme;
    localStorage.setItem('theme', newTheme);
    document.documentElement.setAttribute('data-theme', newTheme);
  }
</script>

<div class="theme-switch">
  <p class="theme-label">{i18n.t('theme')}</p>

  <div class="theme-buttons">
    <button
      id="theme-light"
      class:active_theme={theme === 'light'}
      onclick={() => setTheme('light')}
      aria-label="Light"
    >
      <ThemeLightIcon />
    </button>
    <button
      id="theme-dark"
      class:active_theme={theme === 'dark'}
      onclick={() => setTheme('dark')}
      aria-label="Dark"
    >
      <ThemeDarkIcon />
    </button>
    <button
      id="theme-system"
      class:active_theme={theme === 'system'}
      onclick={() => setTheme('system')}
      aria-label="System"
    >
      <ThemeSystemIcon />
    </button>
  </div>
</div>

<style>
  .theme-switch {
    display: flex;
    align-items: center;
    background: none;
    padding: 0;
    gap: 0.75rem;
  }
  .theme-label {
    font-weight: 600;
  }

  .theme-buttons {
    display: flex;
    align-items: center;
    border-radius: 1rem;
    border: 1px solid gray;
    overflow: hidden;
  }
  button {
    margin: 0;
    min-width: 2rem; /* Keeps same width when icons are loading in */
    padding: 0.5rem;
    padding-bottom: 0.35rem;
    background: none;
    border: none;
    border-radius: 0;

    &:hover {
      background-color: var(--highlight);
    }
    &:active {
      background: none;
    }

    &.active_theme {
      background-color: var(--highlight);
      cursor: default;
    }
  }
</style>
