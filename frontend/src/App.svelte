<script lang="ts">
  import { onMount } from 'svelte';
  import i18n from './i18n.svelte';
  import { Router } from 'sv-router';
  import 'sv-router/generated';

  onMount(async () => {
    await i18n.init();
  });
</script>

<Router />
