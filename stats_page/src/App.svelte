<script lang="ts">
  import { onMount } from 'svelte';
  import PieChart from './charts/PieChart.svelte';
  import LineChart from './charts/LineChart.svelte';
  import { API_URL } from './main';
  import { type DataType, type Duration } from './types';
  import TimeSwitcher from './components/TimeSwitcher.svelte';
  import PDFCounter from './components/PDFCounter.svelte';
  import Leaderboard from './charts/Leaderboard.svelte';
  import PercentileShowcase from './charts/PercentileShowcase.svelte';

  let duration = $state<Duration>('week');

  let userPrefOptions: { path: string; dataType: DataType; label: string }[] = [
    { path: 'course', dataType: 'string', label: 'Course selection' },
    { path: 'lang', dataType: 'string', label: 'Website language' }
  ];
  let selectedUserPrefOption = $state(userPrefOptions[0]);
  let userPrefPath = $derived(selectedUserPrefOption.path);
  let userPrefDataType = $derived(selectedUserPrefOption.dataType);

  let PDFAttributeOptions: {
    path: string;
    dataType: DataType;
    label: string;
  }[] = [
    { path: 'pdf/title', dataType: 'boolean', label: 'Includes title?' },
    { path: 'pdf/subtitle', dataType: 'boolean', label: 'Includes subtitle?' },
    {
      path: 'pdf/name_field',
      dataType: 'boolean',
      label: 'Includes name field?'
    },
    { path: 'pdf/lang', dataType: 'string', label: 'Problem language' },
    {
      path: 'pdf/par_spacing',
      dataType: 'number',
      label: 'Spacing between sets'
    },
    {
      path: 'pdf/max_prefix_group',
      dataType: 'number',
      label: 'Prefix group length'
    },
    {
      path: 'pdf/write_solutions',
      dataType: 'string',
      label: 'Write out solutions?'
    },
    {
      path: 'pdf/answer_columns',
      dataType: 'number',
      label: 'Columns in answer'
    },
    {
      path: 'pdf/page_break_before_answers',
      dataType: 'boolean',
      label: 'Page break before answers?'
    },
    { path: 'pdf/color', dataType: 'boolean', label: 'Color' },
    { path: 'pdf/font_size', dataType: 'number', label: 'Font size' },
    { path: 'pdf/paper_size', dataType: 'string', label: 'Paper size' },
    { path: 'pdf/x_margin', dataType: 'number', label: 'Margin (x)' },
    { path: 'pdf/y_margin', dataType: 'number', label: 'Margin (y)' }
  ];
  let selectedPDFAttributeOption = $state(PDFAttributeOptions[6]);
  let PDFAttributePath = $derived(selectedPDFAttributeOption.path);
  let PDFAttributeDataType = $derived(selectedPDFAttributeOption.dataType);

  let problemSetAttributeOptions: {
    path: string;
    dataType: DataType;
    label: string;
  }[] = [
    { path: 'set/columns', dataType: 'number', label: 'Columns' },
    {
      path: 'set/heading',
      dataType: 'boolean',
      label: 'Includes custom header?'
    },
    {
      path: 'set/spacing',
      dataType: 'number',
      label: 'Spacing between problems'
    },
    {
      path: 'set/page_break',
      dataType: 'boolean',
      label: 'Page break after set'
    },
    {
      path: 'set/starting_difficulty',
      dataType: 'number',
      label: 'Starting difficulty'
    },
    {
      path: 'set/ending_difficulty',
      dataType: 'number',
      label: 'Ending difficulty'
    }
  ];
  let selectedProblemSetAttributeOption = $state(problemSetAttributeOptions[0]);
  let problemSetAttributePath = $derived(
    selectedProblemSetAttributeOption.path
  );
  let problemSetAttributeDataType = $derived(
    selectedProblemSetAttributeOption.dataType
  );

  onMount(async () => {
    await fetch(`${API_URL}/login`);
  });
</script>

<main>
  <TimeSwitcher bind:duration />
  <div class="header-container">
    <h1>Stats - stencil.nu</h1>
    <PDFCounter />
  </div>
  <LineChart path="pdf" {duration} />
  <div class="leaderboards">
    <Leaderboard path="topics" {duration} title="Most used topics" />
    <Leaderboard path="exclusions" {duration} title="Most excluded problems" />
  </div>

  <div class="percentile-container">
    <PercentileShowcase path="renders" {duration} title="Render time (ms)" />
    <PercentileShowcase path="topics" {duration} title="Topics per set" />
    <PercentileShowcase
      path="problem_count"
      {duration}
      title="Problems per set"
    />
    <PercentileShowcase
      path="exclusions"
      {duration}
      title="Exclusions per set"
    />
  </div>

  <div class="pie-container">
    <div class="user-pref pie">
      <h2>User preferences</h2>
      <select
        name="user_pref"
        id="user_pref"
        bind:value={selectedUserPrefOption}
      >
        {#each userPrefOptions as option}
          <option value={option}>{option.label}</option>
        {/each}
      </select>
      <PieChart
        path={userPrefPath}
        {duration}
        dataType={userPrefDataType}
        showTotal={true}
      />
    </div>
    <div class="pdf-attribute pie">
      <h2>PDF Attributes</h2>
      <select
        name="pdf-attributes"
        id="pdf-attributes"
        bind:value={selectedPDFAttributeOption}
      >
        {#each PDFAttributeOptions as option}
          <option value={option}>{option.label}</option>
        {/each}
      </select>
      <PieChart
        path={PDFAttributePath}
        {duration}
        dataType={PDFAttributeDataType}
        showTotal={true}
      />
    </div>
    <div class="problem-set pie">
      <h2>Problem set attributes</h2>
      <select
        name="problem-set-attributes"
        id="problem-set-attributes"
        bind:value={selectedProblemSetAttributeOption}
      >
        {#each problemSetAttributeOptions as option}
          <option value={option}>{option.label}</option>
        {/each}
      </select>
      <PieChart
        path={problemSetAttributePath}
        {duration}
        dataType={problemSetAttributeDataType}
        showTotal={true}
      />
    </div>
  </div>
</main>

<style>
  main {
    display: flex;
    height: 100%;
    overflow-y: auto;
    flex-direction: column;
    align-items: center;

    margin-top: 4rem;
    gap: 10rem;
    padding: 2rem;
    padding-bottom: 10rem;
  }

  .header-container {
    display: flex;
    width: 100%;
    max-width: 90rem;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid gray;
    padding-bottom: 2rem;
    margin-bottom: -7rem;
    h1 {
      font-size: 3.5rem;
    }
  }
  .percentile-container {
    width: 90%;
    max-width: 100rem;
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 2rem;
  }

  .leaderboards {
    width: 90%;
    max-width: 90rem;
    display: flex;
    justify-content: center;
    gap: 4rem;
  }

  .pie-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 4rem;

    .pie {
      width: 26rem;
      display: flex;
      flex-direction: column;
      align-items: center;
    }

    select {
      align-self: start;
      background: none;
      border: none;
      font-size: 1.5rem;
      margin-bottom: 1.5rem;
    }

    h2 {
      width: 100%;
      align-self: start;
      border-bottom: 1px solid gray;
      padding-bottom: 0.5rem;
      margin-bottom: 1rem;
    }
  }
</style>
