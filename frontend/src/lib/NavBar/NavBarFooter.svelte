<script lang="ts">
  import SettingsMenu from './SettingsMenu.svelte';
  import NavbarCloseIcon from '../SVGIcons/NavbarCloseIcon.svelte';
  import NavbarOpenIcon from '../SVGIcons/NavbarOpenIcon.svelte';
  import ContactMe from './ContactMe.svelte';

  let { navbarOpen = $bindable() }: { navbarOpen: boolean } = $props();
</script>

<footer class={navbarOpen ? 'open' : 'closed'}>
  {#if navbarOpen}
    <SettingsMenu name="footer" />
    <ContactMe />
  {/if}
  <button
    onclick={() => (navbarOpen = !navbarOpen)}
    aria-label={navbarOpen ? 'Close navbar' : 'Open navbar'}
  >
    {#if navbarOpen}
      <NavbarCloseIcon />
    {:else}
      <NavbarOpenIcon />
    {/if}
  </button>
</footer>

<style>
  footer {
    flex: 0 0 auto;
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 0.5rem 0.25rem 0 0.25rem; /* NavBar adds padding to sides and bottom */

    border-top: 1px solid var(--border);

    &.closed {
      justify-content: center;
    }
  }

  button {
    padding: 0;
    background: none;
    border: none;
    font-size: 0.7rem;
    &:hover {
      border: none;
    }
  }

  /* Hide in mobile layout */
  @container body (width < 50rem) {
    footer {
      display: none;
    }
  }
</style>
