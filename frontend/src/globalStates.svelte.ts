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

export let setState = $state<{
  addedSets: SetState[];
  pendingSet: ProblemOptions;
  currentEditedSetID: number | null;
  currentEditedSetContents: TopicWithProblems[] | null;
  setCount: number;
  draggedSetIndex: number | null;
}>({
  addedSets: [],
  pendingSet: defaultProblemOptions,
  currentEditedSetID: null,
  currentEditedSetContents: null,
  setCount: 0,
  draggedSetIndex: null
});
export let documentOptions = $state<DocumentOptions>(defaultDocumentOptions);

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

  const mappedSets = setState.addedSets.map(set => set.set);
  try {
    const response: Response = await fetch(`${API_URL}/pdf`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        sets: mappedSets,
        document_options: documentOptions
      })
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
