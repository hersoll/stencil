<script lang="ts">
  import { API_URL } from '$src/main';
  import type { Duration } from '$src/types';

  type PercentileData = {
    min: number;
    max: number;
    p10: number;
    p25: number;
    median: number;
    mean: number;
    p75: number;
    p90: number;
  };

  let {
    path,
    duration,
    title
  }: { path: string; duration: Duration; title: string } = $props();
  let error_message = $state('');
  let data = $state<PercentileData | null>();

  async function fetchData() {
    const res = await fetch(`${API_URL}/stats/boxplots/${path}/${duration}`);
    if (!res.ok) {
      error_message = `Status code ${res.status} \n ${await res.text()}`;
    }
    data = await res.json();
  }

  $effect(() => {
    fetchData();
  });
</script>

<div class="percentile-showcase" id="percentile-showcase-{path}">
  <h2>{title}</h2>
  <div class="median-container">
    <p class="median">{data ? data.median : '...'}</p>
    <p class="median-description">median</p>
  </div>
  <div class="minmax-container">
    <div class="min-container">
      <p class="min value">{data ? data.min : '...'}</p>
      <p class="min description">min</p>
    </div>
    <div class="mean-container">
      <p class="mean value">{data?.mean.toFixed(1) || '...'}</p>
      <p class="mean description">mean</p>
    </div>
    <div class="max-container">
      <p class="max value">{data ? data.max : '...'}</p>
      <p class="max description">max</p>
    </div>
  </div>
  <div class="percentiles-row">
    <div class="p10-container">
      <p class="p10 value">{data?.p10.toFixed(1) || '...'}</p>
      <p class="p10 description">P10</p>
    </div>
    <div class="p25-container">
      <p class="p25 value">{data?.p25.toFixed(1) || '...'}</p>
      <p class="p25 description">P25</p>
    </div>
    <div class="p75-container">
      <p class="p75 value">{data?.p75.toFixed(1) || '...'}</p>
      <p class="p75 description">P75</p>
    </div>
    <div class="p90-container">
      <p class="p90 value">{data?.p90.toFixed(1) || '...'}</p>
      <p class="p90 description">P90</p>
    </div>
  </div>
</div>

<style>
  .percentile-showcase {
    width: 21rem;
  }
  h2 {
    padding-bottom: 0.5rem;
    border-bottom: 1px solid gray;
  }

  .median-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: 1.4rem;

    .median {
      font-size: 2.8rem;
      font-weight: 600;
      color: purple;
      margin-bottom: -0.5rem;
    }
    .median-description {
      font-size: 1.3rem;
    }
  }

  .minmax-container {
    display: flex;
    justify-content: center;
    gap: 2rem;
    margin-bottom: 1rem;

    > div {
      display: flex;
      flex-direction: column;
      align-items: center;

      > .value {
        font-size: 1.8rem;
        font-weight: 600;
      }

      > .description {
        font-size: 1.2rem;
      }
    }
  }

  .percentiles-row {
    display: flex;
    justify-content: center;
    gap: 1.5rem;

    > div {
      display: flex;
      flex-direction: column;
      align-items: center;

      > .value {
        font-size: 1.2rem;
        font-weight: 400;

        &.p25,
        &.p75 {
          font-weight: 500;
          font-size: 1.25rem;
        }
      }

      > .description {
        font-size: 1rem;
      }
    }
  }
</style>
