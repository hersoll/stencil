<script lang="ts">
  import SettingsMenu from './SettingsMenu.svelte';
</script>

<footer>
  <SettingsMenu />
  <button style="color: gray;">Stötta hemsidan</button>
  <button
    ><img
      src="navbar_close.svg"
      alt="Close navbar"
      style="width: 1.2rem;"
    /></button
  >
</footer>

<style>
  footer {
    flex: 0 0 auto;
    display: flex;
    justify-content: space-between;
    width: 100%;
    padding: 0.5rem 0.25rem 0 0.25rem; /* NavBar adds padding to sides and bottom */

    border-top: 1px solid lightgray;
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
