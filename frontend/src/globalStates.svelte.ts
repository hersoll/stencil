import {
  initialDocumentOptions,
  initialFormattingOptions,
  initialProblemOptions,
  type ChapterWithTopics,
  type DocumentOptions,
  type FormattingOptions,
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
  pendingSet: initialProblemOptions,
  currentEditedSetID: null,
  setCount: 0,
  draggedSetIndex: null
});
export let documentOptions = $state<DocumentOptions>(initialDocumentOptions);
export let defaultFormattingOptions = $state<FormattingOptions>(
  initialFormattingOptions
);
export let defaultProblemOptions = $state<ProblemOptions>(
  initialProblemOptions
);
export function setDocumentOptions(next: DocumentOptions) {
  Object.assign(documentOptions, next);
}
export function setDefaultFormattingOptions(next: FormattingOptions) {
  Object.assign(defaultFormattingOptions, next);
}
export function setDefaultProblemOptions(next: ProblemOptions) {
  setState.pendingSet = next;
  Object.assign(defaultProblemOptions, next);
}

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
    let parsed_id = id ? parseInt(id, 10) : null;
    // If we received -1 from the backend it was an unlogged PDF and should be discarded
    PDFState.previous_pdf = parsed_id == -1 ? null : parsed_id;
    PDFState.url = URL.createObjectURL(blob);
  } catch (e) {
    error.message = e instanceof Error ? e.message : 'An error occurred';
  } finally {
    stopLoading();
  }
};
