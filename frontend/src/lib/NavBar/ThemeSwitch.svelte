<script lang="ts">
  let theme = $state(localStorage.getItem('theme') || 'system');

  function setTheme(newTheme: string) {
    theme = newTheme;
    localStorage.setItem('theme', newTheme);
    document.documentElement.setAttribute('data-theme', newTheme);
  }
</script>

<div class="theme-switch">
  <p class="theme-label">Tema</p>

  <div class="theme-buttons">
    <button
      id="theme-light"
      class:active_theme={theme === 'light'}
      onclick={() => setTheme('light')}
    >
      <img src="theme_light.svg" alt="Light" />
    </button>
    <button
      id="theme-dark"
      class:active_theme={theme === 'dark'}
      onclick={() => setTheme('dark')}
    >
      <img src="theme_dark.svg" alt="Dark" />
    </button>
    <button
      id="theme-system"
      class:active_theme={theme === 'system'}
      onclick={() => setTheme('system')}
    >
      <img src="theme_system.svg" alt="System" />
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
    padding-bottom: 0.3rem;
    padding-top: 0.5rem;
    background: none;
    border: none;
    border-radius: 0;

    &:hover {
      background-color: var(--bg-dark);
    }
    &:active {
      background: none;
    }

    &.active_theme {
      background-color: var(--bg-dark);
      cursor: default;
    }

    & img {
      height: 1rem;
    }
  }
</style>
