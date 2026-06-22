<script lang="ts">
  import { fade } from 'svelte/transition';
  import ThemeSwitch from './ThemeSwitch.svelte';
  import LanguageSwitch from './LanguageSwitch.svelte';

  let settingsOpened = $state(false);

  function clickOutside(node: HTMLElement) {
    const handleClick = (event: MouseEvent) => {
      const target = event.target as Node | null;

      if (target && !node.contains(target)) {
        settingsOpened = false;
      }
    };

    document.addEventListener('click', handleClick, true);

    return {
      destroy() {
        document.removeEventListener('click', handleClick, true);
      }
    };
  }
</script>

<div class="settings-container">
  <button
    class="settings-icon"
    onclick={() => (settingsOpened = !settingsOpened)}
    ><img
      src="settings.svg"
      alt="Open settings"
      style="width: 1.2rem;"
    /></button
  >
  {#if settingsOpened}
    <div
      class="settings-menu"
      use:clickOutside
      transition:fade={{ duration: 100 }}
    >
      <ThemeSwitch />
      <LanguageSwitch />
    </div>
  {/if}
</div>

<style>
  .settings-container {
    position: relative;
  }
  .settings-icon {
    background: none;
    border: none;
    padding: 0;
    &:hover {
      border: none;
    }
  }
  .settings-menu {
    position: absolute;
    bottom: 1.75rem;
    left: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    background-color: var(--bg-light);
    border: 1px solid gray;
    padding: 0.5rem;
    border-radius: 0.5rem;
  }
</style>
