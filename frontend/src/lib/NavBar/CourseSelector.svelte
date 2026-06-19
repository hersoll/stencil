<script lang="ts">
  import {
    set_states,
    startLoading,
    stopLoading
  } from '$src/globalStates.svelte';
  import { API_URL } from '$src/main';
  import i18n from '$src/i18n.svelte';
  import { defaultProblemOptions, type CourseData } from '$src/types';

  let {
    course = $bindable()
  }: {
    course: string | null;
  } = $props();

  // Loaded from backend
  let courses: CourseData[] = $state([]);

  async function loadCourses() {
    startLoading();
    const res = await fetch(`${API_URL}/${i18n.currentLanguage}/course`);

    if (!res.ok) {
      throw new Error(`HTTP error! status: ${res.status}`);
    }

    courses = await res.json();
    stopLoading();
  }

  // This will run whenever i18n.currentLanguage changes
  $effect(() => {
    if (i18n.currentLanguage) {
      loadCourses();
    }
  });

  function getCourseDesc(course_name: string) {
    return courses.find(course => course.name === course_name)?.desc;
  }
</script>

<select
  name="course"
  id="course"
  bind:value={course}
  onchange={() => (set_states.pending_set = defaultProblemOptions)}
>
  <option value="ma1b">{getCourseDesc('ma1b')}</option>
  <option value="ma2b">{getCourseDesc('ma2b')}</option>
  <!-- <optgroup label="Nivå 1"> -->
  <!--   <option value="ma1a">{getCourseDesc('ma1a')}</option> -->
  <!--   <option value="ma1b">{getCourseDesc('ma1b')}</option> -->
  <!--   <option value="ma1c">{getCourseDesc('ma1c')}</option> -->
  <!-- </optgroup> -->
  <!-- <optgroup label="Nivå 2"> -->
  <!--   <option value="ma2a">{getCourseDesc('ma2a')}</option> -->
  <!--   <option value="ma2b">{getCourseDesc('ma2b')}</option> -->
  <!--   <option value="ma2c">{getCourseDesc('ma2c')}</option> -->
  <!-- </optgroup> -->
  <!-- <optgroup label="Fortsättning, nivå 1"> -->
  <!--   <option value="maf1b">{getCourseDesc('maf1b')}</option> -->
  <!--   <option value="maf1c">{getCourseDesc('maf1c')}</option> -->
  <!-- </optgroup> -->
  <!-- <optgroup label="Fortsättning, nivå 2"> -->
  <!--   <option value="maf2">{getCourseDesc('maf2')}</option> -->
  <!-- </optgroup> -->
  <!-- <optgroup label="Fördjupning"> -->
  <!--   <option value="mafd">{getCourseDesc('mafd')}</option> -->
  <!-- </optgroup> -->
</select>

<style>
  select {
    font-size: 1rem;
    background: none;
    border: none;
  }
</style>
