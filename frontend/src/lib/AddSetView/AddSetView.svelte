<script lang="ts">
  import CourseDisplay from './CourseDisplay.svelte';
  import CreateSetButton from './CreateSetButton.svelte';
  import InitialSetOptions from './InitialSetOptions.svelte';
  import SelectAllButton from './SelectAllButton.svelte';

  let { course_name }: { course_name: string } = $props();
</script>

<main>
  <CourseDisplay {course_name} />
  <div class="options">
    <InitialSetOptions />
    <div class="right-buttons">
      <SelectAllButton />
      <CreateSetButton />
    </div>
  </div>
</main>

<style>
  main {
    padding: 1rem;
  }

  .options {
    position: fixed;
    background-color: var(--bg);
    border-top: 2px solid lightgray;
    right: 0;
    bottom: 0;
    left: var(--navbar-margin);

    display: flex;
    justify-content: space-between;
    align-items: end;
    padding: 1rem;
    gap: 2rem;
  }

  .right-buttons {
    display: flex;
    gap: 2rem;
  }
</style>
