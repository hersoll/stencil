<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from './ServerMessage.svelte';
  import type { Entry, ProblemEntry } from './types';

  let {
    problem = $bindable(),
    draggedEntry,
    draggedOver
  }: {
    problem: ProblemEntry;
    draggedOver: boolean;
    draggedEntry: Entry | null;
  } = $props();

  let serverMessage: ServerMessage;

  async function handleSubmit() {
    const method =
      problem.id < 0
        ? // New problem
          'POST'
        : // Existing problem
          'PATCH';
    const response = await fetch(`${API_URL}/edit/problem`, {
      method,
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(problem)
    });

    serverMessage.show(response);
  }
</script>

<ServerMessage bind:this={serverMessage} />

<div
  class="container"
  class:dragged-over={draggedOver}
  in:fly={{ y: -15, duration: 200 }}
  out:fly={{ y: 15, duration: 200 }}
>
  {#if problem.id >= 0}
    <h3 class="heading existing">Editing problem</h3>
  {:else}
    <h3 class="heading new">New problem</h3>
  {/if}

  <!-- TRANSLATIONS -->
  <div class="translation-grid">
    <span></span>
    <label for="module">Module</label>
    <label for="name">Name</label>

    <span></span>

    <input
      name="module"
      type="text"
      class="text-input"
      bind:value={problem.module}
    />
    <input
      name="name"
      type="text"
      class="text-input"
      bind:value={problem.name}
    />

    <span></span>
    <h4 class="language-label">Svenska</h4>
    <h4 class="language-label">English</h4>

    <label for="desc_sv">Description</label>
    <input
      name="desc_sv"
      type="text"
      class="text-input"
      bind:value={problem.desc.sv}
    />
    <input
      name="desc_en"
      type="text"
      class="text-input"
      bind:value={problem.desc.en}
    />

    <label for="question_sv">Question</label>
    <input
      name="question_sv"
      type="text"
      class="text-input"
      bind:value={problem.translations.sv.question}
    />
    <input
      name="question_en"
      type="text"
      class="text-input"
      bind:value={problem.translations.en.question}
    />

    <label for="answer_sv">Answer</label>
    <input
      name="answer_sv"
      type="text"
      class="text-input"
      bind:value={problem.translations.sv.answer}
    />
    <input
      name="answer_en"
      type="text"
      class="text-input"
      bind:value={problem.translations.en.answer}
    />

    <label for="solution_sv">Solution</label>
    <input
      name="solution_sv"
      type="text"
      class="text-input"
      bind:value={problem.translations.sv.solution}
    />
    <input
      name="solution_en"
      type="text"
      class="text-input"
      bind:value={problem.translations.en.solution}
    />
  </div>
  <div
    class="prefix-container"
    class:available={draggedEntry?.kind == 'prefix'}
  >
    <!-- TODO: Grayed out if no prefix -->
    <!-- TODO: Drag functions on prefix div -->
    {#if problem.prefix_id}
      <p>Yep this guy sure has a prefix ({problem.prefix_id})</p>
    {:else}
      <p>No prefix</p>
    {/if}
  </div>
  <button class="submit-btn primary" onclick={handleSubmit}>Submit</button>
</div>

<style>
  .container {
    display: grid;
    &.dragged-over {
      input {
        color: var(--text-muted);
        background-color: var(--bg);
      }

      .prefix-container:not(.available) {
        background-color: var(--bg);
      }
    }
  }

  .heading {
    text-align: right;
    &.existing {
      color: var(--secondary);
    }
    &.new {
      color: var(--primary);
    }
  }

  .language-label {
    margin-top: 1rem;
  }

  .translation-grid {
    display: grid;
    align-items: center;
    grid-template-columns: 6rem 1fr 1fr;
    column-gap: 1rem;
    row-gap: 0.5rem;
  }

  .prefix-container {
    display: grid;
    align-items: center;
    justify-items: center;
    margin-top: 2rem;
    height: 4rem;
    background-color: var(--bg-light);
    border-radius: 1rem;
    box-shadow: var(--shadow-elevation-medium);
    border: 2px dashed transparent;
    transition:
      border-color 0.2s ease,
      background-color 0.4s;

    &.available {
      border-color: blueviolet;
    }
  }

  .text-input {
    background-color: var(--bg-light);
    font-size: 1rem;
    padding: 0.5rem;
    border-radius: 0.25rem;
    border: none;
    box-shadow: var(--shadow-elevation-low);
  }

  .submit-btn {
    margin: 2rem auto;
    width: 15rem;
    box-shadow: var(--shadow-elevation-medium);
  }
</style>
