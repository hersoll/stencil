<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { API_URL } from '$src/main';
  import { onDestroy, onMount } from 'svelte';
  import {
    loadingState,
    startLoading,
    stopLoading,
    error
  } from '$src/globalStates.svelte';
  import NavBar from '$src/lib/NavBar/NavBar.svelte';
  import type { View } from '$src/types';
  import ErrorPage from './lib/ErrorPage.svelte';
  import AddSetView from './lib/AddSetView/AddSetView.svelte';
  import PDFView from './lib/PDFView/PDFView.svelte';

  type CourseData = {
    id: number;
    name: string;
    desc: string;
  };

  // Loaded from backend
  let courses: CourseData[] = $state([]);

  let active_course_string: string | null = $state(
    localStorage.getItem('course') || null
  );
  let active_course: CourseData | null = $state(null);

  // Keeps track of which page to show
  let view: View = $state('add_set');

  let showLoadingMessage = $state(false);
  let loadingTimeout: ReturnType<typeof setTimeout> | null = $state(null);
  const LOADING_DELAY = 600;

  $effect(() => {
    if (!loadingState.loading) {
      showLoadingMessage = false;
      return;
    }

    const timeout = setTimeout(() => {
      showLoadingMessage = true;
    }, LOADING_DELAY);

    return () => {
      clearTimeout(timeout);
      showLoadingMessage = false;
    };
  });
  onDestroy(() => {
    if (loadingTimeout) clearTimeout(loadingTimeout);
  });

  async function loadCourses() {
    startLoading();
    const res = await fetch(`${API_URL}/${i18n.currentLanguage}/course`);

    if (!res.ok) {
      throw new Error(`HTTP error! status: ${res.status}`);
    }

    courses = await res.json();
    stopLoading();
  }

  onMount(async () => {
    await i18n.init();
  });

  // This will run whenever i18n.currentLanguage changes
  $effect(() => {
    if (i18n.currentLanguage) {
      loadCourses();
    }
  });

  $effect(() => {
    if (active_course_string) {
      let found_course = courses.find(
        course => course.name === active_course_string
      );
      if (found_course) {
        active_course = found_course;
        localStorage.setItem('course', found_course.name);
      }
    }
  });
</script>

<NavBar bind:course={active_course_string} bind:view />

{#if error.message}
  <ErrorPage />
{:else if showLoadingMessage}
  <main>
    <p>Laddar...</p>
  </main>
{:else if !active_course_string}
  <main>
    <h2>Välj en kurs</h2>
  </main>
{:else if view === 'add_set'}
  <AddSetView course_name={active_course_string} />
{:else if view === 'layout'}
  <h2>Layout View</h2>
{:else if view === 'pdf'}
  <PDFView />
{/if}

<style>
  main {
    margin: 0 auto;
    padding: 1rem;
  }
  h2 {
    font-size: 2rem;
    margin: 0;
    margin-bottom: 2rem;
  }
</style>
