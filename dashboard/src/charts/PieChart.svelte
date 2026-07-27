<script lang="ts">
  import { API_URL } from '$src/main';
  import { Doughnut } from 'svelte-chartjs';
  import annotationPlugin from 'chartjs-plugin-annotation';
  import ChartDataLabels from 'chartjs-plugin-datalabels';
  import {
    Chart as ChartJS,
    Tooltip,
    Legend,
    ArcElement,
    CategoryScale
  } from 'chart.js';
  import type { DataType, Duration } from '$src/types';
  import { borderColor, chartColors, labelColors } from '$src/colors';

  ChartJS.register(
    Tooltip,
    Legend,
    ArcElement,
    CategoryScale,
    annotationPlugin,
    ChartDataLabels
  );

  let error_message = $state('');
  let counts = $state<number[]>([]);
  let labels = $state<string[]>([]);

  let {
    path,
    duration,
    dataType,
    showTotal
  }: {
    path: string;
    duration: Duration;
    dataType: DataType;
    showTotal: boolean;
  } = $props();

  async function fetchData() {
    const res = await fetch(`${API_URL}/stats/${path}/${duration}`);
    if (!res.ok) {
      error_message = `Status code ${res.status} \n ${await res.text()}`;
    }
    let return_obj = await res.json();
    if (dataType == 'number') {
      return_obj = return_obj.sort((a: any, b: any) => a.value - b.value);
    } else if (dataType == 'string') {
      // Sort the values alphabetically
      return_obj = return_obj.sort(
        (
          a: { count: number; value: string },
          b: { count: number; value: string }
        ) => {
          const nameA = a.value.toUpperCase(); // ignore upper and lowercase
          const nameB = b.value.toUpperCase(); // ignore upper and lowercase
          if (nameA < nameB) {
            return -1;
          }
          if (nameA > nameB) {
            return 1;
          }

          // names must be equal
          return 0;
        }
      );
    } else if (dataType == 'boolean') {
      return_obj.sort((a: any, b: any) => Number(b.value) - Number(a.value));
    }
    counts = return_obj.map(({ count }: { count: number }) => count);
    labels = return_obj.map(({ value }: { value: string }) =>
      capitalizeFirst(value)
    );
  }

  let data = $derived({
    labels: labels,
    datasets: [
      {
        data: counts,
        backgroundColor: chartColors,
        borderColor
      }
    ]
  });

  $effect(() => {
    fetchData();
  });

  function capitalizeFirst(value: any): string {
    let str;
    if (value == null) {
      str = 'Default';
    } else {
      str = value.toString();
    }
    return str.charAt(0).toUpperCase() + str.slice(1);
  }
</script>

<div class="background">
  {#if error_message}
    <pre>{error_message}</pre>
  {:else}
    <Doughnut
      {data}
      options={{
        responsive: true,
        plugins: {
          legend: {
            labels: {
              font: { size: 16, weight: 'bold' },
              padding: 10,
              boxWidth: 20
            }
          },
          datalabels: {
            color: labelColors,
            font: { size: 22, weight: 'bold' }
          },
          annotation: {
            annotations: {
              dLabel: {
                type: 'doughnutLabel',
                // @ts-expect-error
                display: showTotal,
                content: ({ chart }) => [
                  'Total',
                  chart.getDatasetMeta(0).total
                ],
                font: [{ size: 22 }, { size: 28, weight: 'bold' }]
              }
            }
          }
        }
      }}
    />
  {/if}
</div>

<style>
  .background {
    width: 18rem;
    height: 18rem;
  }
</style>
