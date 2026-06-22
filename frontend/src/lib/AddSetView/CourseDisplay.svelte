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

  let { courseName }: { courseName: string } = $props();

  async function loadCourseData() {
    startLoading();
    const res = await fetch(
      `${API_URL}/${i18n.currentLanguage}/course/${courseName}`
    );
    if (!res.ok) {
      error.message = `Status code ${res.status} \n ${await res.text()}`;
    }
    loadedCourseContents.chaptersWithTopics = await res.json();
    stopLoading();
  }

  $effect(() => {
    if (courseName && i18n.currentLanguage) {
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
    grid-template-columns: repeat(3, auto);
    gap: 1rem;
  }

  @container main (width < 76rem) {
    .chapter-container {
      grid-template-columns: repeat(2, auto);
    }
  }

  @container main (width < 52rem) {
    .chapter-container {
      grid-template-columns: auto;
    }
  }
</style>
