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
  onclick={() => (view = 'layout')}
  class={view === 'layout' ? 'selected' : ''}
>
  <img src="layout.svg" alt={'Layout icon'} />
  <p>Se och ändra layout</p>
</NavButton>

<style>
  img {
    height: 0.9rem;
  }
</style>
