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

  // TODO: For a rainier day, when we need more dynamic loading of courses
  //
  // let grouped_courses = $derived.by(() => {
  //   const groups = new Map<string, CourseData[]>();
  //
  //   for (const course of courses) {
  //     const course_group = course.name.slice(0, 4);
  //
  //     if (!groups.has(course_group)) {
  //       groups.set(course_group, []);
  //     }
  //
  //     groups.get(course_group)!.push(course);
  //   }
  //
  //   for (const group of groups.values()) {
  //     group.sort((a, b) => a.name.localeCompare(b.name));
  //   }
  //
  //   return [...groups.entries()].sort(([a], [b]) => a.localeCompare(b));
  // });

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
  <option value="mat1b">{getCourseDesc('mat1b')}</option>
  <option value="mat2b">{getCourseDesc('mat2b')}</option>
  <!-- <optgroup label="Nivå 1">  MAKE THIS LANGUAGE DEPENDENT -->
  <!--   <option value="mat1a">{getCourseDesc('mat1a')}</option> -->
  <!--   <option value="mat1b">{getCourseDesc('mat1b')}</option> -->
  <!--   <option value="mat1c">{getCourseDesc('mat1c')}</option> -->
  <!-- </optgroup> -->
  <!-- <optgroup label="Nivå 2"> -->
  <!--   <option value="mat2a">{getCourseDesc('mat2a')}</option> -->
  <!--   <option value="mat2b">{getCourseDesc('mat2b')}</option> -->
  <!--   <option value="mat2c">{getCourseDesc('mat2c')}</option> -->
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
