<script lang="ts">
  let { handleSubmit } = $props();
</script>

<button class="editing-submit-btn primary" onclick={handleSubmit}>Submit</button
>

<style>
  .editing-submit-btn {
    margin: 2rem auto;
    width: 15rem;
    box-shadow: var(--shadow-elevation-medium);
  }
</style>
