<script lang="ts">
  import CourseDisplay from './CourseDisplay.svelte';
  import CreateSetButton from './CreateSetButton.svelte';
  import InitialSetOptions from './InitialSetOptions.svelte';

  let { course_name }: { course_name: string } = $props();

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
</style>
