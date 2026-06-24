<script lang="ts">
  import { fade } from 'svelte/transition';
  import ThemeSwitch from './ThemeSwitch.svelte';
  import LanguageSwitch from './LanguageSwitch.svelte';
  import SettingsIcon from '../SVGIcons/SettingsIcon.svelte';

  let settingsOpened = $state(false);
  // svelte-ignore non_reactive_update
  let settingsButton: HTMLButtonElement;

  function clickOutside(node: HTMLElement, trigger: HTMLElement) {
    const handleClick = (event: MouseEvent) => {
      const target = event.target as Node | null;

      if (!target) return;

      if (!node.contains(target) && !trigger.contains(target)) {
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
    bind:this={settingsButton}
    class="settings-icon"
    onclick={() => (settingsOpened = !settingsOpened)}
    aria-label="Settings"
  >
    <SettingsIcon />
  </button>
  {#if settingsOpened}
    <div
      class="settings-menu"
      use:clickOutside={settingsButton}
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
    display: flex;
    align-items: center;
  }
  .settings-icon {
    background: none;
    border: none;
    padding: 0;
    &:hover {
      border: none;
    }

    .svg-icon {
      width: 1.1rem;
      height: 1.1rem;
      -webkit-mask: url('settings.svg') center / contain no-repeat;
      mask: url('settings.svg') center / contain no-repeat;
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
