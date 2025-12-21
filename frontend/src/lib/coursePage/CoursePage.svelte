<script lang="ts">
  import { API_URL } from '$src/main';
  import i18n from '$src/i18n.svelte';
  import { type CourseData } from './types';
  import ChapterDisplay from './ChapterDisplay.svelte';
  import InitialSetOptions from './InitialSetOptions.svelte';
  import PDFButton from '../PDFButton.svelte';
  import ErrorPage from '../ErrorPage.svelte';
  import { error, problems } from '$src/states.svelte';
  import SetDisplay from './SetDisplay.svelte';

  let { course }: { course: string } = $props();
  let course_data: CourseData | null = $state(null);

  async function loadCourseData() {
    const res = await fetch(
      `${API_URL}/${i18n.currentLanguage}/course/${course}`
    );
    if (!res.ok) {
      error.message = `Status code ${res.status} \n ${await res.text()}`;
    }
    course_data = await res.json();
  }

  $effect(() => {
    if (i18n.currentLanguage) {
      loadCourseData();
    }
  });
</script>

{#if error.message}
  <ErrorPage />
{:else if !course_data}
  <h1>{i18n.t('loading')}...</h1>
{:else}
  <main>
    <section class="main-container">
      <h1>{i18n.t('mathematics')} - {course_data?.desc}</h1>
      <div class="chapter-container">
        {#each course_data?.chapters.filter(c => c.topics.length > 0) as chapter}
          <ChapterDisplay {chapter} />
        {/each}
      </div>
    </section>
    <aside class="options-container">
      <InitialSetOptions />
    </aside>
    {#if problems.sets.length > 0}
      <aside class="set-container">
        <h2>{i18n.t('sets')}</h2>
        {#each problems.sets as set}
          <SetDisplay {set} />
        {/each}
      </aside>
    {/if}
  </main>
  <PDFButton />
{/if}

<style>
  main {
    margin-top: 2rem;
    display: grid;
    grid-template-columns: auto 1fr;
    grid-template-rows: min-content 1fr;
    gap: 2rem;
  }
  .main-container {
    grid-row: 1 / 3;
    padding: 2rem;
    align-self: start;
  }
  .options-container {
    background-color: var(--bg);
    border-radius: 2rem;
    padding: 1rem;
    align-self: start;
  }
  .set-container {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    background-color: var(--bg);
    border-radius: 2rem;
    padding: 1rem;
    align-self: start;

    & h2 {
      margin: 0 0 0.5rem 0;
    }
  }
  .chapter-container {
    margin-top: 2rem;

    column-count: 3;
    column-gap: 1rem;

    max-width: 1100px;
  }

  section {
    background-color: var(--bg);
    border-radius: 2rem;
  }
  h1 {
    margin: 0;
  }
</style>
