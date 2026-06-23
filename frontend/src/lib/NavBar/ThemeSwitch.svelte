<script lang="ts">
  let theme = $state(localStorage.getItem('theme') || 'system');
  import i18n from '$src/i18n.svelte';

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
      <div class="svg-icon light"></div>
    </button>
    <button
      id="theme-dark"
      class:active_theme={theme === 'dark'}
      onclick={() => setTheme('dark')}
      aria-label="Dark"
    >
      <div class="svg-icon dark"></div>
    </button>
    <button
      id="theme-system"
      class:active_theme={theme === 'system'}
      onclick={() => setTheme('system')}
      aria-label="System"
    >
      <div class="svg-icon system"></div>
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

    .svg-icon {
      width: 1rem;
      height: 1rem;
      &.light {
        -webkit-mask: url('theme_light.svg') center / contain no-repeat;
        mask: url('theme_light.svg') center / contain no-repeat;
      }
      &.dark {
        -webkit-mask: url('theme_dark.svg') center / contain no-repeat;
        mask: url('theme_dark.svg') center / contain no-repeat;
      }
      &.system {
        -webkit-mask: url('theme_system.svg') center / contain no-repeat;
        mask: url('theme_system.svg') center / contain no-repeat;
      }
    }
  }
</style>
