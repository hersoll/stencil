<script lang="ts">
  import type { View } from '$src/types';
  import NavButton from './NavButton.svelte';

  let {
    view = $bindable()
  }: {
    view: View;
  } = $props();
</script>

<NavButton
  onclick={() => (view = 'addSet')}
  class={view === 'addSet' ? 'selected' : ''}
>
  <img src="add.svg" alt={'Plus icon'} />
  <p>Lägg till sektioner</p>
</NavButton>

<style>
  img {
    height: 0.9rem;
  }
</style>
