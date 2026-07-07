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
  import { fade } from 'svelte/transition';
  import { onMount } from 'svelte';

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
  let elementToScrollUp: HTMLElement;

  onMount(() => {
    elementToScrollUp.scrollTop = 0;
  });

  $effect(() => {
    if (courseName && i18n.currentLanguage) {
      loadCourseData();
    }
  });
</script>

<div class="course" in:fade={{ duration: 100 }} bind:this={elementToScrollUp}>
  <div class="align-wrapper">
    <div class="heading">
      <h1>{i18n.t('add_sets_heading')}</h1>
      <p class="description">
        {i18n.t('add_sets_instruction')}
        <strong>{i18n.t('add_set')}</strong>
      </p>
    </div>
    <div class="chapter-container" id="chapter-container">
      {#each loadedCourseContents.chaptersWithTopics?.filter(c => c.topics.length > 0) as chapter}
        <ChapterDisplay {chapter} />
      {/each}
    </div>
  </div>
</div>

<style>
  :global(:root) {
    --chapter-card-width: 24rem;
    --chapter-card-gap: 1rem;
  }
  .course {
    flex: 1 1 auto;
    min-height: 0;
    padding-top: 2rem;
    overflow: auto;
    display: block;
    height: 100%;
    width: 100%;

    .align-wrapper {
      display: flex;
      flex-direction: column;
      align-items: center;
      min-height: 100%;
      justify-content: center;
    }
  }

  .heading {
    margin-bottom: 1.5rem;
    /* Chapter column width + gap*/
    width: calc(3 * var(--chapter-card-width) + 2 * var(--chapter-card-gap));
    strong {
      color: var(--primary-text);
    }
  }

  .chapter-container {
    column-count: 3;
    column-gap: var(--chapter-card-gap);
  }

  @container main (width < 76rem) {
    .chapter-container {
      column-count: 2;
    }
    .heading {
      /* Chapter column width + gap */
      width: calc(2 * var(--chapter-card-width) + 1 * var(--chapter-card-gap));
    }
  }

  @container main (width < 56rem) {
    .chapter-container {
      column-count: 1;
    }
    .heading {
      /* Chapter column width + gap */
      width: 24rem;
    }
    .subtitle {
      width: 24rem;
    }
  }
</style>
