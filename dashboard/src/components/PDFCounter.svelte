<script lang="ts">
  import { API_URL } from '$src/main';
  import { borderColor, chartColors, labelColors } from '$src/colors';

  let error_message = $state('');
  let count = $state('...');

  async function fetchData() {
    const res = await fetch(`${API_URL}/stats/pdf`);
    if (!res.ok) {
      error_message = `Status code ${res.status} \n ${await res.text()}`;
    }
    count = await res.text();
  }

  $effect(() => {
    fetchData();
  });
</script>

<div class="pdf-count-container">
  <p class="count">{count}</p>
  <p class="count-description">Total PDFs generated</p>
</div>

<style>
  .count {
    color: purple;
    font-weight: 700;
    font-size: 2.5rem;
  }
  .count-description {
    font-size: 1.1rem;
  }
</style>
