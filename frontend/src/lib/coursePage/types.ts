export type TopicData = {
  name: string;
  id: number;
  desc: string;
};

export type ChapterData = {
  name: string;
  id: number;
  desc: string;
  topics: TopicData[];
};

export type CourseData = {
  name: string;
  id: number;
  desc: string;
  chapters: ChapterData[];
};

export type SetOptions = {
  question_columns: number;
  heading: string;
  spacing: number | null;
};

export type Difficulty = 'Intro' | 'Easy' | 'Medium' | 'Hard';
/// Should match the ProblemSetSpec of the backend
export type ProblemSetSpec = {
  topics: number[];
  exclusions: number[];
  starting_difficulty: Difficulty;
  ending_difficulty: Difficulty;
  n: number;
  options: SetOptions;
};

export const defaultProblemSet: ProblemSetSpec = {
  topics: [],
  exclusions: [],
  starting_difficulty: 'Intro',
  ending_difficulty: 'Hard',
  n: 10,
  options: { question_columns: 2, heading: '', spacing: null }
};

export type DocumentOptions = {
  font_size: number;
  title: string;
  answer_columns: number;
  lang: 'Sv' | 'En';
  write_solutions: 'None' | 'First' | 'All';
  color: boolean;
  paper_size: 'A4' | 'A5';
  x_margin: number;
  y_margin: number;
  par_spacing: number | null;
  max_prefix_group: number | null;
};
