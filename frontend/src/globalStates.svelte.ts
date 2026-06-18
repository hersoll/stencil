import {
  defaultDocumentOptions,
  defaultProblemOptions,
  type DocumentOptions,
  type ProblemOptions,
  type SetState
} from '$src/types';

export let set_id = $state({ count: 0 });
export let error = $state<{ message: string | null }>({ message: null });

export let set_states = $state<{
  added_sets: SetState[];
  pending_set: ProblemOptions;
}>({ added_sets: [], pending_set: defaultProblemOptions });
export let document_options = $state<DocumentOptions>(defaultDocumentOptions);

export const loadingState = $state({ loading: false });
export function startLoading() {
  loadingState.loading = true;
}
export function stopLoading() {
  loadingState.loading = false;
}
