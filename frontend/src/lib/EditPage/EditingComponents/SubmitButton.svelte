<script lang="ts">
  let { handleSubmit } = $props();
</script>

<button class="editing-submit-btn primary" onclick={handleSubmit}>Submit</button
>

<style>
  .editing-submit-btn {
    margin: 0 auto;
    width: 15rem;
    box-shadow: var(--shadow-elevation-medium);
    align-self: center;
    justify-self: center;
  }
</style>
