<script lang="ts">
  import { API_URL } from '$src/main';
  import { onMount } from 'svelte';

  let data = $state('');

  async function accessAdminPage() {
    const res = await fetch(`${API_URL}/admin`);

    if (res.status === 401) {
      console.log('Logging in');
      login();
      return;
    }

    data = await res.text();
  }

  function login() {
    const returnTo = window.location.href;
    window.location.href = `${API_URL}/admin/login?return=${encodeURIComponent(returnTo)}`;
  }

  onMount(() => accessAdminPage());
</script>

<p>{data}</p>

<style>
  p {
    margin: 5rem;
  }
</style>
