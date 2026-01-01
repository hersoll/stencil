<script lang="ts">
  import { API_URL } from '$src/main';
  import i18n from '$src/i18n.svelte';
  import { type CourseData } from './types';
  import { applyMasonry } from './masonry';
  import ChapterDisplay from './ChapterCard.svelte';
  import InitialSetOptions from './InitialSetOptions.svelte';
  import PDFCard from './PDFCard.svelte';
  import ErrorPage from '../ErrorPage.svelte';
  import DocumentOptions from './DocumentOptions.svelte';
  import CreateSetButton from './CreateSetButton.svelte';
  import SetContainer from './SetContainer.svelte';
  import { fade, fly } from 'svelte/transition';
  import { error, sets } from '$src/states.svelte';

  let { course }: { course: string } = $props();
  let course_data: CourseData | null = $state(null);

  let container = $state<HTMLElement | undefined>();

  let show_loading_message = $state(false);
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

  $effect(() => {
    if (i18n.currentLanguage) {
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
        <h1 class="title">
          <span class="subject-prefix">{i18n.t('mathematics')} - {' '}</span
          >{course_data?.desc}
        </h1>
        <h2 class="subtitle">{i18n.t('instructions')}</h2>
        <button
          class="select-all-btn"
          onclick={sets.current_set.topics.length == 0
            ? selectAllTopics
            : deselectAllTopics}
          >{sets.current_set.topics.length == 0
            ? i18n.t('select_all')
            : i18n.t('clear')}</button
        >
        <div
          class="chapter-container"
          id="chapter-container"
          bind:this={container}
        >
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
      <DocumentOptions />
      {#if sets.set_states.length > 0}
        <SetContainer />
      {/if}
    </div>
  </main>
{/if}

<style>
  main {
    margin: 1rem;
    display: grid;
    grid-template-columns: full;
    align-items: start;
    gap: 2rem;
  }

  h1 {
    align-self: self-start;
    color: var(--text);
    font-size: 2rem;
  }
  .subject-prefix {
    display: none;
  }
  @media (min-width: 50rem) {
    main {
      margin: 2rem;

      grid-template-columns: auto 22rem;
    }
    h1 {
      font-size: 3rem;
    }
  }

  .col {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .main-container {
    position: relative;
    padding: 1rem;
    box-shadow: var(--shadow-elevation-low);
    display: flex;
    flex-direction: column;
    align-items: center;

    h2 {
      align-self: self-start;
      color: var(--text-muted);
    }
  }

  .chapter-container {
    margin-top: 2rem;
    align-self: stretch;

    display: grid;
    grid-template-columns: 1fr;
    grid-auto-rows: 1px;
    column-gap: 1rem;
    max-width: 1200px;
  }

  .subtitle {
    font-size: 1rem;
    max-width: 15rem;
  }

  @media (min-width: 50rem) {
    .main-container {
      padding: 2rem;
    }
    .chapter-container {
      grid-template-columns: repeat(1, 1fr);
    }
    .subtitle {
      font-size: 1.5rem;
      max-width: 28rem;
    }
  }

  @media (min-width: 75rem) {
    .chapter-container {
      grid-template-columns: repeat(2, 1fr);
    }
    .subtitle {
      max-width: 28rem;
    }
    .subject-prefix {
      display: inline;
    }
  }

  @media (min-width: 100rem) {
    .chapter-container {
      grid-template-columns: repeat(3, 1fr);
    }

    .subtitle {
      max-width: 28rem;
    }
  }

  section {
    background-color: var(--bg);
    border-radius: 2rem;
  }
  h1 {
    margin: 0;
    color: var(--text);
  }

  .select-all-btn {
    position: absolute;
    top: 1rem;
    right: 1rem;
    box-shadow: var(--shadow-elevation-low);
    font-size: 0.8rem;
    border: 2px solid transparent;
    &:hover {
      border-color: var(--primary);
    }
    &:active {
      background-color: var(--bg);
      transition: background-color 0.1s;
    }
  }

  @media (min-width: 50rem) {
    .select-all-btn {
      font-size: 1rem;
    }
  }

  @media (min-width: 75rem) {
    .select-all-btn {
      top: 2rem;
      right: 2rem;
    }
  }

  .loading-message {
    margin-top: 10rem;
  }

  :global(body) {
    padding-bottom: 1rem;
  }
</style>
