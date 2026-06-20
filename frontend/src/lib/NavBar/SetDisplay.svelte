<script lang="ts">
  import { set_states } from '$src/globalStates.svelte';
  import type { View } from '$src/types';
  import SetCard from './SetCard.svelte';
  let {
    view = $bindable()
  }: {
    view: View;
  } = $props();
</script>

<div class="sets-container">
  <div class="set-header">
    <img src="section.svg" alt="Section icon" />
    <p>Sektioner</p>
  </div>
  {#each set_states.added_sets as set}
    <SetCard set={set.set} set_id={set.id} bind:view />
  {/each}
</div>

<style>
  .sets-container {
    flex: 1 1 auto;
    overflow-y: auto;
    display: flex;
    margin-top: 0.5rem;
    flex-direction: column;
    gap: 0.25rem;
    border-top: 2px solid lightgray;
    padding-top: 0.8rem;
  }

  .set-header {
    margin-left: 0.5rem;
    display: flex;
    gap: 0.4rem;
    align-items: center;
    font-size: 1em;
    font-weight: 400;
    font-family: inherit;
    margin-bottom: 0.25rem;
  }

  img {
    height: 0.8rem;
  }
</style>
