<script lang="ts">
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
    System
  </button>
  <button
    id="theme-dark"
    class:active_theme={theme === 'dark'}
    onclick={() => setTheme('dark')}
  >
    Dark
  </button>
  <button
    id="theme-light"
    class:active_theme={theme === 'light'}
    onclick={() => setTheme('light')}
  >
    Light
  </button>
</div>

<style>
  div {
    position: absolute;
    right: 2em;
    background-color: var(--bg-light);
    padding: 1rem;
    border-radius: 1rem;
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
      color: var(--primary);
      cursor: default;
    }
  }
</style>
