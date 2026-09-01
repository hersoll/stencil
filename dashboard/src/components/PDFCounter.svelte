<script lang="ts">
  import { API_URL } from '$src/main';
  import { borderColor, chartColors, labelColors } from '$src/colors';

  let error_message = $state('');
  let totalCount = $state('...');
  let uniqueCount = $state('...');

  async function fetchTotal() {
    const res = await fetch(`${API_URL}/stats/pdf`);
    if (!res.ok) {
      error_message = `Status code ${res.status} \n ${await res.text()}`;
    }
    totalCount = await res.text();
  }

  async function fetchUnique() {
    const res = await fetch(`${API_URL}/stats/pdf/unique`);
    if (!res.ok) {
      error_message = `Status code ${res.status} \n ${await res.text()}`;
    }
    uniqueCount = await res.text();
  }

  $effect(() => {
    fetchTotal();
    fetchUnique();
  });
</script>

<div class="pdf-count-container">
  <div>
    <p class="count">{totalCount}</p>
    <p class="count-description">Total PDFs generated</p>
  </div>
  <div>
    <p class="count">{uniqueCount}</p>
    <p class="count-description">Unique PDFs</p>
  </div>
</div>

<style>
  .pdf-count-container {
    display: flex;
    gap: 2rem;
  }
  .count {
    color: var(--primary);
    font-weight: 700;
    font-size: 2.5rem;
  }
  .count-description {
    font-size: 1.1rem;
  }
</style>
