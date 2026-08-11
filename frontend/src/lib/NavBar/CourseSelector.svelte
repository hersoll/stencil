<script lang="ts">
  import {
    setState,
    startLoading,
    stopLoading,
    defaultProblemOptions
  } from '$src/globalStates.svelte';
  import { API_URL } from '$src/main';
  import i18n from '$src/i18n.svelte';
  import { type CourseData } from '$src/types';

  let {
    course = $bindable()
  }: {
    course: string;
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
  onchange={() => {
    setState.pendingSet = defaultProblemOptions;
    if (course) {
      localStorage.setItem('course', course);
    }
  }}
>
  <option value="" disabled selected hidden>{i18n.t('select_course')}</option>
  <option value="basic">{getCourseDesc('basic')}</option>
  <optgroup label={i18n.t('math_level_1')}>
    <option value="mat1a">{getCourseDesc('mat1a')}</option>
    <option value="mat1b">{getCourseDesc('mat1b')}</option>
    <option value="mat1c">{getCourseDesc('mat1c')}</option>
  </optgroup>
  <optgroup label={i18n.t('math_level_2')}>
    <option value="mat2a">{getCourseDesc('mat2a')}</option>
    <option value="mat2b">{getCourseDesc('mat2b')}</option>
    <option value="mat2c">{getCourseDesc('mat2c')}</option>
  </optgroup>
  <!-- <optgroup label={i18n.t('math_level_f1')}> -->
  <!--   <option value="maf1b">{getCourseDesc('maf1b')}</option> -->
  <!--   <option value="maf1c">{getCourseDesc('maf1c')}</option> -->
  <!-- </optgroup> -->
  <!-- <option value="maf2">{getCourseDesc('maf2')}</option> -->
  <!-- <option value="mafd">{getCourseDesc('mafd')}</option> -->
</select>

<style>
  select {
    font-size: 1rem;
    background: none;
    border: none;
    border-radius: 1rem;
    padding: 0 0.5rem;
    text-align: right;
    &:hover {
      background-color: var(--bg-dark);
    }
    &:has(option[value='']:checked) {
      color: black;
    }
  }

  @container body (width < 50rem) {
    select {
      font-size: clamp(0.8rem, 0.6226rem + 0.75vw, 1rem);
      justify-self: end;
    }
  }
</style>
