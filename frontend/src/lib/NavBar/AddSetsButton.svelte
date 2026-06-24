<script lang="ts">
  import type { View } from '$src/types';
  import i18n from '$src/i18n.svelte';
  import NavButton from './NavButton.svelte';
  import AddIcon from '../SVGIcons/AddIcon.svelte';

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
  <AddIcon />
  {#if navbarOpen}
    <p>{i18n.t('add_sets_nav')}</p>
  {/if}
</NavButton>

<style>
</style>
