<script lang="ts">
  import { API_URL } from '$src/main';
  import { Line } from 'svelte-chartjs';
  import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale
  } from 'chart.js';
  import type { Duration } from '$src/types';
  import { chartColors } from '$src/colors';

  ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale
  );

  let error_message = $state('');
  let counts = $state<number[]>([]);
  let labels = $state<Date[]>([]);
  //let total_count = $derived(counts.reduce((acc, cur) => acc + cur, 0));

  let {
    path,
    duration
  }: {
    path: string;
    duration: Duration;
  } = $props();

  async function fetchData() {
    const res = await fetch(`${API_URL}/stats/${path}/${duration}`);
    if (!res.ok) {
      error_message = `Status code ${res.status} \n ${await res.text()}`;
    }
    let return_obj = await res.json();
    counts = return_obj.map(({ count }: { count: number }) => count);
    labels = return_obj.map(({ time }: { time: string }) => {
      let date = new Date(time + 'Z');
      if (duration == 'day') {
        // 11:00
        return date.toLocaleTimeString('en-GB', {
          hour: '2-digit',
          minute: '2-digit'
        });
      } else if (duration == 'week' || duration == 'month') {
        // Sat Jul 11
        return date.toLocaleDateString('en-US', {
          weekday: 'short',
          month: 'short',
          day: 'numeric'
        });
      } else if (duration == 'three_months' || duration == 'year') {
        // Week number
        const start_date = new Date(
          Date.UTC(date.getFullYear(), date.getMonth(), date.getDate())
        );
        // Set to nearest Thursday: current date + 4 - current day number (Mon=1..Sun=7)
        const dayNum = start_date.getUTCDay() || 7;
        start_date.setUTCDate(start_date.getUTCDate() + 4 - dayNum);
        const yearStart = new Date(Date.UTC(start_date.getUTCFullYear(), 0, 1));
        return Math.ceil(
          ((start_date.getTime() - yearStart.getTime()) / 86400000 + 1) / 7
        );
      } else {
        // Month, year
        return date.toLocaleDateString('en-US', {
          year: 'numeric',
          month: 'long'
        });
      }
    });
  }

  let data = $derived({
    labels: labels,
    datasets: [
      {
        data: counts,
        fill: true,
        pointRadius: 5,
        pointBackgroundColor: chartColors[1],
        borderColor: chartColors[1],
        tension: 0.18
      }
    ]
  });

  $effect(() => {
    fetchData();
  });
</script>

<div class="background">
  {#if error_message}
    <pre>{error_message}</pre>
  {:else}
    <h2>PDFs generated</h2>
    <Line
      {data}
      options={{
        plugins: { legend: { display: false }, datalabels: { display: false } },
        aspectRatio: 3
      }}
    />
  {/if}
</div>

<style>
  .background {
    width: 90%;
    max-width: 90rem;
  }
</style>
