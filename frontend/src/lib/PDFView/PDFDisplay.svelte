<script lang="ts">
  import { PDFState } from '$src/globalStates.svelte';
  import { fly } from 'svelte/transition';
</script>

<form action="" in:fly={{ y: -60, duration: 400 }}>
  <div class="iframe-container">
    <iframe src={PDFState.url} title="PDF Viewer"></iframe>
  </div>
</form>

<style>
  form {
    height: 100%;
    flex: 1 1 auto;
  }

  .iframe-container {
    position: relative;
    width: 100%;
    height: 100%;
    padding: 1rem;

    iframe {
      width: 100%;
      height: 100%;
      border-radius: 1rem;
      border: none;
    }
  }
  /* TODO: 
      - Footer in PDF View with pdf renaming
      - Footer in navbar with settings + donate + close icon
      - Redesign EditSetView
 */
</style>
