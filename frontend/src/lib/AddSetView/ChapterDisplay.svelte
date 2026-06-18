<script lang="ts">
  import type { ChapterWithTopics } from '$src/types';
  import { set_states } from '$src/globalStates.svelte';
  import { onMount } from 'svelte';
  import TopicCheckbox from './TopicCheckbox.svelte';
  let { chapter }: { chapter: ChapterWithTopics } = $props();

  onMount(() => {});
</script>

<div class="chapter-card">
  <h2>
    {chapter.desc}
  </h2>
  <div>
    {#each chapter.topics as topic}
      <TopicCheckbox {topic} />
    {/each}
  </div>
</div>

<style>
  div {
    display: grid;
  }
  h2 {
    margin: 0 0 1rem 0;
    color: var(--text);
  }

  .chapter-card {
    background-color: var(--bg);
    height: fit-content;
    min-height: 0;
    justify-self: stretch;
    break-inside: avoid;
    border-top: 1px solid gray;
    padding: 1rem 0;
  }
</style>
