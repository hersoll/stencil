<script lang="ts">
  import CourseDisplay from './CourseDisplay.svelte';
  import CreateSetButton from './CreateSetButton.svelte';
  import InitialSetOptions from './InitialSetOptions.svelte';
  import SelectAllButton from './SelectAllButton.svelte';

  let { courseName }: { courseName: string } = $props();
</script>

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
    <CourseDisplay {courseName} />
  </div>
</div>
<div class="footer">
  <InitialSetOptions />
  <SelectAllButton />
  <CreateSetButton />
</div>

<style>
  .course {
    flex: 1 1 auto;
    min-height: 0;
    overflow: auto;
    height: 100%;
    width: 100%;
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

  .footer {
    flex: 0 0 auto;
    background-color: var(--bg);
    border-top: 2px solid lightgray;
    left: var(--navbar-margin);

    display: grid;
    grid-template-columns: 1fr auto auto;
    width: 100%;
    align-items: end;
    padding: 1rem;
    column-gap: 2rem;
    row-gap: 1rem;
  }

  /* Too wide for all items in one row */
  @container main (width < 73rem) {
    .footer {
      grid-template-columns: auto auto;
      justify-content: center;
    }
  }
  /* Too wide for three items in top row */
  @container main (width < 56rem) {
    .footer {
      grid-template-columns: auto auto;
      justify-content: center;
    }
  }
  @container main (width < 52rem) {
    .course {
      padding-top: 2rem;
    }
    .subtitle {
      width: 24rem;
    }
  }
</style>
