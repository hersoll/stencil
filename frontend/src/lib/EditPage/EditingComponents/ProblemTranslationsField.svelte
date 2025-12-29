<script lang="ts">
  let { problem } = $props();
</script>

<label for="question_sv">Question</label>
<input
  name="question_sv"
  type="text"
  class="editing-text-input"
  bind:value={problem.translations.sv.question}
/>
<input
  name="question_en"
  type="text"
  class="editing-text-input"
  bind:value={problem.translations.en.question}
/>

<label for="answer_sv">Answer</label>
<input
  name="answer_sv"
  type="text"
  class="editing-text-input"
  bind:value={problem.translations.sv.answer}
/>
<input
  name="answer_en"
  type="text"
  class="editing-text-input"
  bind:value={problem.translations.en.answer}
/>

<label for="solution_sv">Solution</label>
<input
  name="solution_sv"
  type="text"
  class="editing-text-input"
  bind:value={problem.translations.sv.solution}
/>
<input
  name="solution_en"
  type="text"
  class="editing-text-input"
  bind:value={problem.translations.en.solution}
/>
