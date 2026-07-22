<script lang="ts">
  import ThemeSwitch from './ThemeSwitch.svelte';
  import LanguageSwitch from './LanguageSwitch.svelte';
  import SettingsIcon from '../SVGIcons/SettingsIcon.svelte';

  let { name }: { name: string } = $props();
</script>

<button
  class="settings-icon"
  aria-label="Settings"
  popovertarget="settings-popover-{name}"
  style="anchor-name: --icon-{name};"
>
  <SettingsIcon />
</button>

<div
  popover
  id="settings-popover-{name}"
  class="settings-popover"
  style="position-anchor: --icon-{name};"
>
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
    z-index: 25;

    transition:
      opacity 0.25s,
      display 0.25s allow-discrete;
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

  /* Mobile layout: The settings icon will be at the top*/
  @container body (width < 50rem) {
    .settings-popover {
      position-anchor: --icon;
      top: 0;
      bottom: 0;
      position-area: bottom;
    }
  }
</style>
