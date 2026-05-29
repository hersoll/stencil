<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { navigate } from 'sv-router/generated';
  import { API_URL } from '$src/main';
  import { onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  type CourseData = {
    id: number;
    name: string;
    desc: string;
  };

  let data: CourseData[] = $state([]);
  let loading = $state(false);

  async function loadCourses() {
    loading = true;
    const res = await fetch(`${API_URL}/${i18n.currentLanguage}/course`);

    if (!res.ok) {
      throw new Error(`HTTP error! status: ${res.status}`);
    }

    data = await res.json();
    loading = false;
  }

  onMount(async () => {
    await loadCourses();
  });

  // This will run whenever i18n.currentLanguage changes
  $effect(() => {
    if (i18n.currentLanguage) {
      loadCourses();
    }
  });
</script>

{#if i18n.loading}
  <p>Laddar...</p>
{:else}
  <div class="main-div" in:fly={{ y: 60, duration: 600 }}>
    <h2>{i18n.t('course_selector')}</h2>
    <div class="btn-container">
      <button onclick={() => navigate('/ma1b')}
        >{data.find(course => course.name == 'ma1b')?.desc}</button
      >
      <button onclick={() => navigate('/ma2b')}
        >{data.find(course => course.name == 'ma2b')?.desc}</button
      >
    </div>
  </div>
{/if}

<style>
  button {
    font-size: 2.5rem;
    &:hover {
      background-color: var(--primary);
    }
  }

  h2 {
    font-size: 3rem;
    margin: 0;
    margin-bottom: 2rem;
  }

  .main-div {
    margin: 10rem auto;
    padding: 2rem 4rem;
    max-width: 40rem;
    border-radius: 2rem;

    background-color: var(--bg);
    text-align: center;
  }

  .btn-container {
    display: grid;
    gap: 2rem;
  }
</style>
