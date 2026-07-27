<script lang="ts">
  let { entry, entryIsCopy } = $props();
</script>

{#if entryIsCopy}
  <h3 class="heading new">New {entry.kind}</h3>
{:else}
  <h3 class="heading existing">Editing {entry.kind}</h3>
{/if}

<style>
  .heading {
    text-align: right;
    &.existing {
      color: var(--secondary);
    }
    &.new {
      color: var(--primary);
    }
  }
</style>
