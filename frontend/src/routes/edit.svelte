<script lang="ts">
  import ProblemPage from '$src/lib/EditPage/ProblemPage.svelte';
  import ErrorPage from '$src/lib/ErrorPage.svelte';
  import { API_URL } from '$src/main';
  import { error } from '$src/states.svelte';
  import { onMount } from 'svelte';

  let data = $state('');

  async function accessAdminPage() {
    const res = await fetch(`${API_URL}/edit`);

    if (res.status === 401) {
      console.log('Logging in');
      login();
      return;
    }

    data = await res.text();
  }

  function login() {
    const returnTo = window.location.href;
    window.location.href = `${API_URL}/edit/login?return=${encodeURIComponent(returnTo)}`;
  }

  onMount(() => accessAdminPage());
</script>

{#if error.message}
  <ErrorPage />
{:else}
  <ProblemPage />
{/if}

<style>
</style>
