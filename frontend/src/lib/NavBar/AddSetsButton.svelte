<script lang="ts">
  import type { View } from '$src/types';
  import NavButton from './NavButton.svelte';

  let {
    view = $bindable(),
    navbarOpen
  }: {
    view: View;
    navbarOpen: boolean;
  } = $props();
</script>

<NavButton
  onclick={() => (view = 'addSet')}
  class="{view === 'addSet' ? 'selected' : ''} {navbarOpen
    ? 'nav-open'
    : 'nav-closed'}"
>
  <img src="add.svg" alt={'Plus icon'} />
  {#if navbarOpen}
    <p>Lägg till sektioner</p>
  {/if}
</NavButton>

<style>
  img {
    height: 0.9rem;
  }
</style>
