<script lang="ts">
  import { set_states } from '$src/globalStates.svelte';
  import type { TopicData } from '$src/types';

  let { topic }: { topic: TopicData } = $props();

  function handleChange(event: Event) {
    const checkbox = event.target as HTMLInputElement;
    if (checkbox.checked) {
      set_states.pending_set.topics.push(Number(checkbox.value));
    } else {
      set_states.pending_set.topics = set_states.pending_set.topics.filter(
        t => t != Number(checkbox.value)
      );
    }
  }
</script>

<span>
  <input
    name={'topic_' + topic.id}
    id={'topic_' + topic.id}
    value={topic.id}
    type="checkbox"
    checked={set_states.pending_set.topics.includes(topic.id)}
    onchange={handleChange}
  />
  <label for={'topic_' + topic.id} class="no-select">{topic.desc}</label>
</span>

<style>
  label {
    color: var(--text-muted);
    cursor: pointer;
    transition: color 0.15s;
    &:hover {
      color: var(--primary-text);
    }
    &:active {
      color: var(--secondary-text);
    }
  }
</style>
