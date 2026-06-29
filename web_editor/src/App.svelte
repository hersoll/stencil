<script lang="ts">
  import EditPage from '$src/lib/EditPage.svelte';
  import ErrorPage from '$src/lib/ErrorPage.svelte';
  import { error } from '$src/lib/error.svelte';
</script>

{#if error.message}
  <ErrorPage />
{:else}
  <EditPage />
{/if}
