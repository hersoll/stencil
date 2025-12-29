<script lang="ts">
  let topic_ids = $state([]);
</script>

<div class="topics-container">
  <pre>The current topic ids are:</pre>
  {#each topic_ids as id}
    <pre>{id}</pre>
  {/each}
</div>

<style>
  .topics-container {
    grid-column: 1 / 3;
    width: 100%;
    height: 100%;

    background-color: var(--bg-light);
    border-radius: 1rem;
    box-shadow: var(--shadow-elevation-low);

    text-align: center;
  }
</style>
