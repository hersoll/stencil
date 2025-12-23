<script lang="ts">
  import { API_URL } from '$src/main';
  import i18n from '$src/i18n.svelte';
  import { type CourseData } from './types';
  import { applyMasonry } from './masonry';
  import ChapterDisplay from './ChapterCard.svelte';
  import InitialSetOptions from './InitialSetOptions.svelte';
  import PDFCard from './PDFCard.svelte';
  import ErrorPage from '../ErrorPage.svelte';
  import { error, sets } from '$src/states.svelte';
  import SetDisplay from './SetCard.svelte';
  import DocumentOptions from './DocumentOptions.svelte';
  import CreateSetButton from './CreateSetButton.svelte';
  import { fade, fly } from 'svelte/transition';

  let { course }: { course: string } = $props();
  let course_data: CourseData | null = $state(null);
  let show_loading_message = $state(false);

  let container = $state<HTMLElement | undefined>();

  const delay = setTimeout(() => {
    if (!course_data) show_loading_message = true;
  }, 600);

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
    show_loading_message = false;
    clearTimeout(delay);
    course_data = await res.json();
  }

  function deleteSet(id: number) {
    document.getElementById(`set-editor-${id}`)?.hidePopover();
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
{:else if show_loading_message}
  <h1 class="loading-message" in:fade={{ duration: 200 }}>
    {i18n.t('loading')}...
  </h1>
{:else if course_data}
  <main>
    <div class="col">
      <section class="main-container" in:fly={{ y: 60, duration: 600 }}>
        <h1>{i18n.t('mathematics')} - {course_data?.desc}</h1>
        <h2>{i18n.t('instructions')}</h2>
        <div class="chapter-container" bind:this={container}>
          {#each course_data?.chapters.filter(c => c.topics.length > 0) as chapter}
            <ChapterDisplay {chapter} />
          {/each}
        </div>
        <InitialSetOptions />
        <CreateSetButton />
      </section>
      {#if sets.set_states.length > 0}
        <PDFCard />
      {/if}
    </div>

    <div class="col">
      <div class="options-container" in:fly={{ y: 60, duration: 600 }}>
        <DocumentOptions />
      </div>
      {#if sets.set_states.length > 0}
        <aside class="set-container" in:fly={{ y: 60, duration: 600 }}>
          <div>
            <h2>{i18n.t('sets')}</h2>
            <p>{i18n.t('click_to_edit')}</p>
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
    grid-template-columns: auto 22rem;
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
    box-shadow: var(--shadow-elevation-low);
    display: flex;
    flex-direction: column;
    align-items: center;

    h1 {
      align-self: self-start;
      color: var(--text);
    }
    h2 {
      align-self: self-start;
      color: var(--text-muted);
    }
  }
  .set-container {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    background-color: var(--bg);
    border-radius: 2rem;
    padding: 1rem;
    box-shadow: var(--shadow-elevation-low);

    & h2 {
      margin: 0;
    }

    & p {
      margin: 0;
      color: var(--text-muted);
    }
  }

  .options-container {
    padding: 1rem;
    border-radius: 2rem;
    background-color: var(--bg);
    box-shadow: var(--shadow-elevation-low);
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
    color: var(--text);
  }

  .loading-message {
    margin-top: 10rem;
  }
</style>
