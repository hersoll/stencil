import { defaultProblemSet, type ProblemSetSpec } from './lib/coursePage/types';

export let error = $state<{ message: string | null }>({ message: null });
export let problems = $state<{
  sets: ProblemSetSpec[];
  current_set: ProblemSetSpec;
}>({ sets: [], current_set: defaultProblemSet });
