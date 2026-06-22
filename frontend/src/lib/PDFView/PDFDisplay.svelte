<script lang="ts">
  import { PDFState } from '$src/globalStates.svelte';
  import { fly } from 'svelte/transition';
</script>

<div class="iframe-container" in:fly={{ y: -60, duration: 400 }}>
  <iframe src={PDFState.url} title="PDF Viewer"></iframe>
</div>

<style>
  .iframe-container {
    flex: 1 1 auto;
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
</style>
