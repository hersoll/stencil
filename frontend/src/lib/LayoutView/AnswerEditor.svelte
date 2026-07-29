<script lang="ts">
  import { documentOptions } from '$src/globalStates.svelte';
  import i18n from '$src/i18n.svelte';

  const ANSWER_COLUMNS_MIN = 1;
  const ANSWER_COLUMNS_MAX = 5;
  function handleAnswerColumnsBlur(
    e: Event & { currentTarget: HTMLInputElement }
  ) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      documentOptions.answerColumns = Math.max(
        ANSWER_COLUMNS_MIN,
        Math.min(ANSWER_COLUMNS_MAX, value)
      );
    } else {
      documentOptions.answerColumns = ANSWER_COLUMNS_MIN;
    }
  }
</script>

<div
  class="card"
  id="answer-editor"
  style="position-anchor: --answer-skeleton;"
>
  <div class="card-header">
    <h2>{i18n.t('answer_key')}</h2>
  </div>

  <div class="options-container">
    <!-- WRITE SOLUTIONS  -->
    <div class="label-div">
      <label for="write_solutions">{i18n.t('document_option_solutions')}</label>
      <select
        id="write_solutions"
        class="write-solutions"
        bind:value={documentOptions.writeSolutions}
      >
        <option value="First"
          >{i18n.t('document_option_solutions_first')}</option
        >
        <option value="All">{i18n.t('document_option_solutions_all')}</option>
        <option value="None">{i18n.t('document_option_solutions_none')}</option>
      </select>
    </div>

    <div class="label-div">
      <label for="answer-columns"
        >{i18n.t('document_option_answer_columns')}</label
      >
      <input
        id="answer-columns"
        type="number"
        bind:value={documentOptions.answerColumns}
        min={ANSWER_COLUMNS_MIN}
        max={ANSWER_COLUMNS_MAX}
        onblur={handleAnswerColumnsBlur}
      />
    </div>
    <div class="label-div">
      <label for="pagebreak-before"
        >{i18n.t('document_option_answer_break')}</label
      >
      <input
        id="pagebreak-before"
        type="checkbox"
        checked={documentOptions.pageBreakBeforeAnswers}
        onchange={e =>
          (documentOptions.pageBreakBeforeAnswers = e.currentTarget.checked)}
      />
    </div>

    <div class="label-div">
      <label for="solution-decoration"
        >{i18n.t('document_option_solution_decoration')}</label
      >
      <select
        id="solution-decoration"
        class="solution-decoration"
        bind:value={documentOptions.solutionDecoration}
      >
        <option value="Fill"
          >{i18n.t('document_option_solution_decoration_fill')}</option
        >
        <option value="Border"
          >{i18n.t('document_option_solution_decoration_border')}</option
        >
        <option value="None"
          >{i18n.t('document_option_solution_decoration_none')}</option
        >
      </select>
    </div>
  </div>
</div>

<style>
  .card {
    position: absolute;
    top: anchor(top);
    left: anchor(right);
    width: 31rem;
    margin-left: 2rem;
  }

  h2 {
    font-size: 1.2rem;
  }

  .options-container {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    font-size: 0.9rem;
  }

  .label-div {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    color: var(--text-muted);
  }

  .write-solutions,
  .solution-decoration {
    border: none;
    font-size: 0.8rem;
  }

  @container main (width < 70rem) {
    .card {
      position: relative;
      margin-top: 1rem;
      margin-left: 0;
      width: 100%;
      max-width: 30rem;
    }

    /* Increasing the font size to prevent iOS zoom */
    label {
      font-size: 1rem;
    }
    input {
      font-size: 1rem;
    }
    .options-container {
      select {
        font-size: 1rem;
      }
    }
  }
</style>
