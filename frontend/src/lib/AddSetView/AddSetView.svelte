<script lang="ts">
  import CourseDisplay from './CourseDisplay.svelte';
  import CreateSetButton from './CreateSetButton.svelte';
  import InitialSetOptions from './InitialSetOptions.svelte';

  let { course_name }: { course_name: string } = $props();
</script>

<main>
  <CourseDisplay {course_name} />
  <div class="options">
    <InitialSetOptions />
    <CreateSetButton />
  </div>
</main>

<style>
  main {
    padding: 1rem;
  }

  .options {
    position: fixed;
    right: 0;
    bottom: 0;
    left: var(--navbar-margin);

    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 1rem;
    gap: 2rem;
  }
</style>
