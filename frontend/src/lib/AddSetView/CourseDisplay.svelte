<script lang="ts">
  import { API_URL } from '$src/main';
  import i18n from '$src/i18n.svelte';
  import { error, startLoading, stopLoading } from '$src/globalStates.svelte';
  import { type ChapterWithTopics } from '$src/types';
  import ChapterDisplay from './ChapterDisplay.svelte';

  let { course_name }: { course_name: string } = $props();

  let chapters_with_topics: ChapterWithTopics[] = $state([]);
  let container = $state<HTMLElement | undefined>();

  async function loadCourseData() {
    startLoading();
    const res = await fetch(
      `${API_URL}/${i18n.currentLanguage}/course/${course_name}`
    );
    if (!res.ok) {
      error.message = `Status code ${res.status} \n ${await res.text()}`;
    }
    chapters_with_topics = await res.json();
    stopLoading();
  }

  $effect(() => {
    if (course_name && i18n.currentLanguage) {
      loadCourseData();
    }
  });

  function deselectAllTopics() {
    const chapterContainer = document.getElementById('chapter-container');
    chapterContainer
      ?.querySelectorAll<HTMLInputElement>('input[type="checkbox"]')
      .forEach(box => {
        box.checked = false;
        box.dispatchEvent(new Event('change', { bubbles: true }));
      });
  }

  function selectAllTopics() {
    const chapterContainer = document.getElementById('chapter-container');
    chapterContainer
      ?.querySelectorAll<HTMLInputElement>('input[type="checkbox"]')
      .forEach(box => {
        box.checked = true;
        box.dispatchEvent(new Event('change', { bubbles: true }));
      });
  }
</script>

<div class="chapter-container" id="chapter-container" bind:this={container}>
  {#each chapters_with_topics?.filter(c => c.topics.length > 0) as chapter}
    <ChapterDisplay {chapter} />
  {/each}
</div>

<style>
  .chapter-container {
    display: grid;
    grid-auto-flow: column;
    grid-template-rows: repeat(4, auto);
    gap: 1rem;
  }
</style>
