<script lang="ts">
  import { API_URL } from '$src/main';
  import i18n from '$src/i18n.svelte';
  import {
    error,
    startLoading,
    stopLoading,
    loadedCourseContents
  } from '$src/globalStates.svelte';
  import ChapterDisplay from './ChapterDisplay.svelte';

  let { course_name }: { course_name: string } = $props();

  async function loadCourseData() {
    startLoading();
    const res = await fetch(
      `${API_URL}/${i18n.currentLanguage}/course/${course_name}`
    );
    if (!res.ok) {
      error.message = `Status code ${res.status} \n ${await res.text()}`;
    }
    loadedCourseContents.chaptersWithTopics = await res.json();
    stopLoading();
  }

  $effect(() => {
    if (course_name && i18n.currentLanguage) {
      loadCourseData();
    }
  });
</script>

<div class="chapter-container" id="chapter-container">
  {#each loadedCourseContents.chaptersWithTopics?.filter(c => c.topics.length > 0) as chapter}
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
