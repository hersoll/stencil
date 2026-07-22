<script lang="ts">
  import CourseDisplay from './CourseDisplay.svelte';
  import CreateSetButton from './CreateSetButton.svelte';
  import SetOptions from './SetOptions.svelte';
  import SelectAllButton from './SelectAllButton.svelte';
  import { setState } from '$src/globalStates.svelte';

  let { courseName }: { courseName: string } = $props();
</script>

<CourseDisplay {courseName} />
<div class="footer">
  <SetOptions bind:problemOptions={setState.pendingSet} />
  <SelectAllButton />
  <CreateSetButton />
</div>

<style>
  .footer {
    display: grid;
    grid-template-columns: 1fr auto auto;
    align-items: end;
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

  /* Mobile layout*/
  @container body (width < 50rem) {
    .footer {
      flex: 1 1;
      position: relative;
      left: 0;
      padding: 0.5rem;
    }
  }
</style>
