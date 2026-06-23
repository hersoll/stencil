<script lang="ts">
  import SettingsMenu from './SettingsMenu.svelte';

  let { navbarOpen = $bindable() }: { navbarOpen: boolean } = $props();
</script>

<footer class={navbarOpen ? 'open' : 'closed'}>
  {#if navbarOpen}
    <SettingsMenu />
    <button style="color: gray;">Stötta hemsidan</button>
  {/if}
  <button onclick={() => (navbarOpen = !navbarOpen)}
    ><img
      src={navbarOpen ? 'navbar_close.svg' : 'navbar_open.svg'}
      alt="Close navbar"
      style="width: 1.2rem;"
    /></button
  >
</footer>

<style>
  footer {
    flex: 0 0 auto;
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 0.5rem 0.25rem 0 0.25rem; /* NavBar adds padding to sides and bottom */

    border-top: 1px solid lightgray;

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
</style>
