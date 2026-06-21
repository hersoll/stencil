<script lang="ts">
  import CourseDisplay from './CourseDisplay.svelte';
  import CreateSetButton from './CreateSetButton.svelte';
  import InitialSetOptions from './InitialSetOptions.svelte';
  import SelectAllButton from './SelectAllButton.svelte';

  let { course_name }: { course_name: string } = $props();
</script>

<main>
  <div class="course">
    <div class="align-wrapper">
      <div class="heading">
        <h1>Lägg till sektioner</h1>
        <p class="subtitle">
          Välj ett eller flera områden att inkludera och klicka sedan på <strong
            >Lägg till sektion</strong
          >
        </p>
      </div>
      <CourseDisplay {course_name} />
    </div>
  </div>
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
    display: flex;
    height: 100%;
    flex-direction: column;
    justify-content: space-between;
  }
  .course {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    .align-wrapper {
      display: flex;
      flex-direction: column;
      align-items: start;
    }
  }

  .heading {
    margin-bottom: 1.5rem;
    h1 {
      font-size: 2rem;
      margin-bottom: 0.25rem;
    }
  }

  .options {
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
