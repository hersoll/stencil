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

<div class="answer-editor" style="position-anchor: --answer-skeleton;">
  <h2>{i18n.t('answer_key')}</h2>

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
  </div>
</div>

<style>
  .answer-editor {
    position: absolute;
    background: none;
    border-top: 1px solid var(--strong-border);
    padding: 0.5rem;
    padding-left: 0.2rem;
    top: anchor(top);
    left: anchor(right);
    width: 32rem;
    margin-left: 2rem;
  }

  h2 {
    font-size: 1.2rem;
    margin-bottom: 0.5rem;
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

  .write-solutions {
    border: none;
    font-size: 0.8rem;
  }
</style>
