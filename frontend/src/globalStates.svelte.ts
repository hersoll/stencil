import {
  defaultDocumentOptions,
  defaultProblemOptions,
  type DocumentOptions,
  type ProblemOptions,
  type SetState
} from '$src/types';
import { API_URL } from './main';

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

export const PDFState = $state({ url: '' });
export const fetchPdf = async (): Promise<void> => {
  startLoading();

  const mapped_sets = set_states.added_sets.map(set => set.set);
  try {
    const response: Response = await fetch(`${API_URL}/pdf`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ sets: mapped_sets, document_options })
    });

    if (!response.ok) {
      let text = await response.text();
      error.message = `Status: ${response.status} \n${text}`;
      return;
    }

    // Get the PDF as a blob (binary data)
    const blob: Blob = await response.blob();
    PDFState.url = URL.createObjectURL(blob);
  } catch (e) {
    error.message = e instanceof Error ? e.message : 'An error occurred';
  } finally {
    stopLoading();
  }
};
