import {
  defaultDocumentOptions,
  defaultProblemOptions,
  type ChapterWithTopics,
  type DocumentOptions,
  type ProblemOptions,
  type SetState,
  type TopicWithProblems
} from '$src/types';
import { API_URL } from './main';

export let error = $state<{ message: string | null }>({ message: null });
export let loadedCourseContents = $state<{
  chaptersWithTopics: ChapterWithTopics[];
}>({ chaptersWithTopics: [] });

export let set_states = $state<{
  added_sets: SetState[];
  pending_set: ProblemOptions;
  current_edited_set_id: number | null;
  current_edited_set_contents: TopicWithProblems[] | null;
  set_count: number;
}>({
  added_sets: [],
  pending_set: defaultProblemOptions,
  current_edited_set_id: null,
  current_edited_set_contents: null,
  set_count: 0
});
export let document_options = $state<DocumentOptions>(defaultDocumentOptions);

export const loadingState = $state({ loading: false });
export function startLoading() {
  loadingState.loading = true;
}
export function stopLoading() {
  loadingState.loading = false;
}

export const PDFState = $state<{
  url: string;
  fileName: string;
}>({ url: '', fileName: 'stencil' });

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
