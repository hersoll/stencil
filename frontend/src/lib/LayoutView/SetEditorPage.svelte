<script lang="ts">
  import { setState } from '$src/globalStates.svelte';
  import AnswerEditor from './AnswerEditor.svelte';
  import SetEditor from './SetEditor.svelte';

  let { isActive }: { isActive: boolean } = $props();
</script>

<div class="editor-container {isActive ? 'open' : 'closed'}">
  {#each setState.addedSets, i}
    <SetEditor bind:set={setState.addedSets[i]} />
  {/each}
  <AnswerEditor />
</div>

<style>
  .editor-container {
    flex: 0 0 auto;
    width: 35rem;
  }

  @container main (width < 70rem) {
    .editor-container.open {
      width: 100%;
    }
    .editor-container.closed {
      display: none;
    }
  }
</style>
