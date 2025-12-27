import {
  defaultDocumentOptions,
  defaultProblemSet,
  type DocumentOptions,
  type ProblemSetSpec,
  type SetState
} from '$lib/CoursePage/types';

export let set_id = $state({ count: 0 });
export let error = $state<{ message: string | null }>({ message: null });

export let sets = $state<{
  set_states: SetState[];
  current_set: ProblemSetSpec;
}>({ set_states: [], current_set: defaultProblemSet });
export let document_options = $state<DocumentOptions>(defaultDocumentOptions);
