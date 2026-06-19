<script lang="ts">
  import { loadingState, PDFState } from '$src/globalStates.svelte';
  import PDFCard from './PDFCard.svelte';
</script>

<main>
  {#if PDFState.url}
    <PDFCard />
  {:else if !loadingState.loading}
    <div class="text-container">
      <h2>Du har inte skapat någon PDF än!</h2>
      <p>
        Lägg till minst en sektion och klicka på <strong>Skapa PDF</strong>
      </p>
    </div>
  {/if}
</main>

<style>
  .text-container {
    display: flex;
    height: 100vh;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }
</style>
