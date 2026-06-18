<script lang="ts">
  import { set_states } from '$src/globalStates.svelte';
  import SetCard from './SetCard.svelte';
</script>

<div class="sets-container">
  {#each set_states.added_sets as set}
    <SetCard set={set.set} id={set.id} />
  {/each}
</div>

<style>
  .sets-container {
    flex: 1 1 auto;
    overflow-y: auto;
    display: flex;
    margin: 0 -0.5rem;
    flex-direction: column;
    gap: 0.25rem;
  }
</style>
