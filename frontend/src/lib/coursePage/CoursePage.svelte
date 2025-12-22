<script lang="ts">
  import { API_URL } from '$src/main';
  import i18n from '$src/i18n.svelte';
  import { type CourseData } from './types';
  import { applyMasonry } from './masonry';
  import ChapterDisplay from './ChapterDisplay.svelte';
  import InitialSetOptions from './InitialSetOptions.svelte';
  import PDFButton from '../PDFButton.svelte';
  import ErrorPage from '../ErrorPage.svelte';
  import { error, sets } from '$src/states.svelte';
  import SetDisplay from './SetDisplay.svelte';

  let { course }: { course: string } = $props();
  let course_data: CourseData | null = $state(null);

  let container = $state<HTMLElement | undefined>();

  // Ensures the chapter cards are in their proper layout
  $effect(() => {
    const el = container;
    if (el && course_data) {
      requestAnimationFrame(() => {
        if (el) {
          const cleanup = applyMasonry(el);
          return cleanup;
        }
      });
    }
  });

  async function loadCourseData() {
    const res = await fetch(
      `${API_URL}/${i18n.currentLanguage}/course/${course}`
    );
    if (!res.ok) {
      error.message = `Status code ${res.status} \n ${await res.text()}`;
    }
    course_data = await res.json();
  }

  function deleteSet(id: number) {
    sets.set_states = sets.set_states.filter(state => state.id !== id);
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
    <div class="col">
      <section class="main-container">
        <h1>{i18n.t('mathematics')} - {course_data?.desc}</h1>
        <div class="chapter-container" bind:this={container}>
          {#each course_data?.chapters.filter(c => c.topics.length > 0) as chapter}
            <ChapterDisplay {chapter} />
          {/each}
        </div>
      </section>
      <PDFButton />
    </div>

    <div class="col">
      <aside class="options-container">
        <InitialSetOptions />
      </aside>
      {#if sets.set_states.length > 0}
        <aside class="set-container">
          <div>
            <h2>{i18n.t('sets')}</h2>
            <p>Klicka för att redigera</p>
          </div>
          {#each sets.set_states as set_state}
            <SetDisplay
              bind:set={set_state.set}
              id={set_state.id}
              onDelete={() => deleteSet(set_state.id)}
            />
          {/each}
        </aside>
      {/if}
    </div>
  </main>
{/if}

<style>
  main {
    margin-top: 2rem;
    display: grid;
    grid-template-columns: auto 1fr;
    align-items: start;
    gap: 2rem;
  }

  .col {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .main-container {
    padding: 2rem;
  }
  .options-container {
    background-color: var(--bg);
    border-radius: 2rem;
    padding: 1rem;
  }
  .set-container {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    background-color: var(--bg);
    border-radius: 2rem;
    padding: 1rem;

    & h2 {
      margin: 0;
    }

    & p {
      margin: 0;
      color: var(--text-muted);
    }
  }
  .chapter-container {
    margin-top: 2rem;

    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    grid-auto-rows: 1px;
    column-gap: 2rem;
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
