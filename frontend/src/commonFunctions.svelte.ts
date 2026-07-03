import { error } from './globalStates.svelte';
import i18n from './i18n.svelte';
import { API_URL } from './main';
import type { TopicWithProblems } from './types';

export async function fetchProblemsForTopics(
  topics: number[]
): Promise<TopicWithProblems[]> {
  const res = await fetch(`${API_URL}/${i18n.currentLanguage}/problems`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(topics)
  });
  if (!res.ok) {
    error.message = `Status code ${res.status} \n ${await res.text()}`;
  }
  return await res.json();
}
