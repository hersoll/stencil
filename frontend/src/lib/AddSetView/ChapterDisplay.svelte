<script lang="ts">
  import type { ChapterWithTopics } from '$src/types';
  import TopicCheckbox from './TopicCheckbox.svelte';
  let { chapter }: { chapter: ChapterWithTopics } = $props();
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
    color: var(--text);
    padding: 0.5rem 0;
    border-top: 1px solid var(--strong-border);
  }

  .chapter-card {
    background-color: var(--bg);
    height: fit-content;
    min-height: 0;
    width: 24rem;
    justify-self: stretch;
    break-inside: avoid;
    padding-bottom: 1rem;
  }
</style>
