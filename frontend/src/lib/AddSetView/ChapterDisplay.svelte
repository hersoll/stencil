<script lang="ts">
  import type { ChapterWithTopics } from '$src/types';
  import TopicCheckbox from './TopicCheckbox.svelte';
  let { chapter }: { chapter: ChapterWithTopics } = $props();
</script>

<div class="card" id="chapter-card-{chapter.id}">
  <div class="card-header">
    <h2>
      {chapter.desc}
    </h2>
  </div>
  <div class="topic-grid">
    {#each chapter.topics as topic}
      <TopicCheckbox {topic} />
    {/each}
  </div>
</div>

<style>
  .topic-grid {
    display: grid;
  }

  .card {
    min-height: 0;
    width: 24rem;
    justify-self: stretch;
  }

  @container body (width < 50rem) {
    .card {
      width: 100%;
      flex: 1 1;
    }
  }
</style>
