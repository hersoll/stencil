<script lang="ts">
  import type { ProblemEntry } from '../types';

  let { problem = $bindable() }: { problem: ProblemEntry } = $props();

  function autoResize(e: Event) {
    const target = e.target;
    if (target instanceof HTMLTextAreaElement) {
      target.style.height = 'auto';
      target.style.height = target.scrollHeight + 'px';
    }
  }
</script>

<label for="question_sv">Question</label>
<textarea
  name="question_sv"
  class="editing-text-input"
  oninput={autoResize}
  bind:value={problem.translations.sv.question}
></textarea>
<textarea
  name="question_en"
  class="editing-text-input"
  oninput={autoResize}
  bind:value={problem.translations.en.question}
></textarea>

<label for="answer_sv">Answer</label>
<textarea
  name="answer_sv"
  class="editing-text-input"
  oninput={autoResize}
  bind:value={problem.translations.sv.answer}
></textarea>
<textarea
  name="answer_en"
  class="editing-text-input"
  oninput={autoResize}
  bind:value={problem.translations.en.answer}
></textarea>

<label for="solution_sv">Solution</label>
<textarea
  name="solution_sv"
  class="editing-text-input"
  oninput={autoResize}
  bind:value={problem.translations.sv.solution}
></textarea>
<textarea
  name="solution_en"
  class="editing-text-input"
  oninput={autoResize}
  bind:value={problem.translations.en.solution}
></textarea>

<style>
  textarea {
    resize: none;
    height: auto;
  }
</style>
