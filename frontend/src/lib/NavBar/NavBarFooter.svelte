<script lang="ts">
  import SettingsMenu from './SettingsMenu.svelte';
  import i18n from '$src/i18n.svelte';

  let { navbarOpen = $bindable() }: { navbarOpen: boolean } = $props();
</script>

<footer class={navbarOpen ? 'open' : 'closed'}>
  {#if navbarOpen}
    <SettingsMenu />
    <button style="color: var(--text-muted);">{i18n.t('support_me')}</button>
  {/if}
  <button
    onclick={() => (navbarOpen = !navbarOpen)}
    aria-label={navbarOpen ? 'Close navbar' : 'Open navbar'}
  >
    <div class="svg-icon {navbarOpen ? 'open' : 'closed'}"></div>
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

    .svg-icon {
      width: 1.2rem;
      height: 1.2rem;
      &.open {
        -webkit-mask: url('navbar_close.svg') center / contain no-repeat;
        mask: url('navbar_close.svg') center / contain no-repeat;
      }
      &.closed {
        -webkit-mask: url('navbar_open.svg') center / contain no-repeat;
        mask: url('navbar_open.svg') center / contain no-repeat;
      }
    }
  }
</style>
