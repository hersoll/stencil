import {
  defaultDocumentOptions,
  defaultProblemOptions,
  type ChapterWithTopics,
  type DocumentOptions,
  type ProblemOptions,
  type SetState
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
  setCount: number;
  draggedSetIndex: number | null;
}>({
  addedSets: [],
  pendingSet: defaultProblemOptions,
  currentEditedSetID: null,
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
  previous_pdf: number | null;
}>({ url: '', fileName: 'stencil', previous_pdf: null });

export const fetchPdf = async (): Promise<void> => {
  startLoading();

  const mappedSets = setState.addedSets.map(set => set.options);
  try {
    const response: Response = await fetch(`${API_URL}/pdf`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        sets: mappedSets,
        document_options: documentOptions,
        previous_pdf: PDFState.previous_pdf
      })
    });

    if (!response.ok) {
      let text = await response.text();
      error.message = `Status: ${response.status} \n${text}`;
      return;
    }

    const id = response.headers.get('X-PDF-ID');
    const blob: Blob = await response.blob();
    PDFState.previous_pdf = id ? parseInt(id, 10) : null;
    PDFState.url = URL.createObjectURL(blob);
  } catch (e) {
    error.message = e instanceof Error ? e.message : 'An error occurred';
  } finally {
    stopLoading();
  }
};
