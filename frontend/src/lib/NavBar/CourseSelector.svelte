<script lang="ts">
  let {
    course = $bindable()
  }: {
    course: string | null;
  } = $props();
</script>

<input type="text" bind:value={course} />

<style>
  input {
    width: 7rem;
  }
</style>
