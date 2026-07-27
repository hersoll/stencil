<script lang="ts">
  import { API_URL } from '$src/main';
  import type { Duration } from '$src/types';

  type Row = { value: any; count: number };
  type Leaderboard = Row[];

  let {
    path,
    duration,
    title
  }: { path: string; duration: Duration; title: string } = $props();
  let leaderboard = $state<Leaderboard>([]);
  let error_message = $state('');

  async function fetchData() {
    const res = await fetch(`${API_URL}/stats/leaderboard/${path}/${duration}`);
    if (!res.ok) {
      error_message = `Status code ${res.status} \n ${await res.text()}`;
    }
    leaderboard = await res.json();
  }

  $effect(() => {
    fetchData();
  });
</script>

<div class="leaderboard" id="leaderboard-{path}">
  <h2>{title}</h2>
  {#if leaderboard.length == 0}
    {#each { length: 10 }, i}
      <div
        class="leaderboard-row {i == 0 ? 'first' : i == 1 ? 'second' : 'small'}"
      >
        <p class="label">...</p>
        <p class="count">...</p>
      </div>
    {/each}
  {:else}
    {#each leaderboard as row, i}
      <div
        class="leaderboard-row {i == 0 ? 'first' : i == 1 ? 'second' : 'small'}"
      >
        <p class="label">
          {row.value}
        </p>
        <p class="count">
          {row.count}
        </p>
      </div>
    {/each}
  {/if}
</div>

<style>
  .leaderboard {
    width: 32rem;
  }
  h2 {
    padding-bottom: 0.5rem;
    border-bottom: 1px solid gray;
  }
  .leaderboard-row {
    display: flex;
    justify-content: space-between;

    .count {
      padding-left: 2rem;
    }

    &.first {
      font-size: 1.5rem;
      font-weight: 600;
      margin-bottom: 0.2rem;
    }
    &.second {
      font-size: 1.3rem;
      font-weight: 500;
      margin-bottom: 0.2rem;
    }
    &.small {
      font-size: 1.1rem;
      font-weight: 500;
    }
  }
</style>
