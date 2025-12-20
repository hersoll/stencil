<script lang="ts">
  import { API_URL } from '$src/main';
  import i18n from '$src/i18n.svelte';
  import { onMount } from 'svelte';
  import {
    type CourseData,
    type ProblemSetSpec,
    type DocumentOptions,
    defaultProblemSet
  } from './types';
  import ChapterDisplay from './ChapterDisplay.svelte';
  import InitialSetOptions from './InitialSetOptions.svelte';
  import PDFButton from '../PDFButton.svelte';
  let { course }: { course: string } = $props();
  let course_data: CourseData | null = $state(null);

  let sets: ProblemSetSpec[] = $state([]);
  let current_set: ProblemSetSpec = $state(defaultProblemSet);

  async function loadCourseData() {
    const res = await fetch(
      `${API_URL}/${i18n.currentLanguage}/course/${course}`
    );
    if (!res.ok) {
      throw new Error(`HTTP error! status: ${res.status}`);
    }
    course_data = await res.json();
  }

  function submitSet() {
    sets.push({ ...current_set });
  }

  onMount(async () => {
    await loadCourseData();
  });

  $effect(() => {
    if (i18n.currentLanguage) {
      loadCourseData();
    }
  });
</script>

{#if !course_data}
  <h1>Laddar...</h1>
{:else}
  <main>
    <section>
      <h1>{i18n.t('mathematics')} - {course_data?.desc}</h1>
      <div class="chapter-container">
        {#each course_data?.chapters.filter(c => c.topics.length > 0) as chapter}
          <ChapterDisplay {chapter} {current_set} />
        {/each}
      </div>
    </section>
    <InitialSetOptions {submitSet} {...current_set} />
  </main>
  <div>
    {#each sets as set, i}
      <p>Set {i + 1}: {set.topics}</p>
    {/each}
  </div>
  <PDFButton {sets} />
{/if}

<style>
  main {
    display: grid;
    grid-template-columns: 3fr 1fr;
    gap: 2rem;
  }
  .chapter-container {
    margin-top: 2rem;
    display: grid;
    max-width: 900px;
    grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
    gap: 2rem;
  }
  section {
    background-color: var(--bg);
    padding: 2rem;
    margin-top: 2rem;
    border-radius: 2rem;
  }
  h1 {
    margin: 0;
  }
</style>
