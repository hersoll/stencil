<script lang="ts">
  import { setState } from '$src/globalStates.svelte';
  import type { TopicData } from '$src/types';

  let { topic }: { topic: TopicData } = $props();

  function handleChange(event: Event) {
    const checkbox = event.target as HTMLInputElement;
    if (checkbox.checked) {
      setState.pendingSet.topics.push(Number(checkbox.value));
    } else {
      setState.pendingSet.topics = setState.pendingSet.topics.filter(
        t => t != Number(checkbox.value)
      );
    }
  }
</script>

<div>
  <input
    name={'topic_' + topic.id}
    id={'topic_' + topic.id}
    value={topic.id}
    type="checkbox"
    checked={setState.pendingSet.topics.includes(topic.id)}
    onchange={handleChange}
  />
  <label for={'topic_' + topic.id} class="no-select">{topic.desc}</label>
</div>

<style>
  div {
    display: flex;
    gap: 0.25rem;
    align-items: center;
    label {
      color: var(--text-muted);
    }
    &:hover {
      cursor: pointer;
      label {
        color: var(--primary-text);
      }
      input {
        cursor: pointer;
      }
    }
  }
  input {
    margin-top: 0.1rem;
  }
  label {
    cursor: pointer;
    transition: color 0.2s;
    &:hover {
      color: var(--primary-text);
      cursor: pointer;
    }

    &:active {
      color: var(--secondary-text);
    }
  }
</style>
