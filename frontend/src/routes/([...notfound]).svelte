<script lang="ts">
  import i18n from '$src/i18n.svelte';
</script>

<div class="not-found-container">
  <h1>{i18n.t('page_not_found_heading')}</h1>
  <p>{i18n.t('page_not_found_body')}</p>
  <a href="/">{i18n.t('page_not_found_link')}</a>
</div>

<style>
  .not-found-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
  }

  h1 {
    margin-bottom: 1rem;
  }
</style>
