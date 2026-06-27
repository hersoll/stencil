<script lang="ts">
  import ThemeSwitch from './ThemeSwitch.svelte';
  import LanguageSwitch from './LanguageSwitch.svelte';
  import SettingsIcon from '../SVGIcons/SettingsIcon.svelte';
</script>

<button
  class="settings-icon"
  aria-label="Settings"
  popovertarget="settings-popover"
>
  <SettingsIcon />
</button>

<div popover id="settings-popover" class="settings-popover">
  <div class="settings-menu">
    <ThemeSwitch />
    <LanguageSwitch />
  </div>
</div>

<style>
  .settings-icon {
    background: none;
    border: none;
    padding: 0;
    &:hover {
      border: none;
    }
  }
  .settings-popover {
    inset: auto;
    justify-self: anchor-center;
    opacity: 0;
    bottom: anchor(top, 2rem);
    left: anchor(right, 2rem);

    margin: 0;
    padding: 0;
    border: none;
    background: none;

    transition: all 0.25s allow-discrete;
    .settings-menu {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      background-color: var(--bg-light);
      border: 1px solid var(--strong-border);
      padding: 0.5rem;
      border-radius: 0.5rem;
    }

    &:popover-open {
      opacity: 1;
    }

    @starting-style {
      &:popover-open {
        opacity: 0;
      }
    }
  }
</style>
