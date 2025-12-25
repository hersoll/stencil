export type ProblemData = {
  id: number;
  difficulty: number;
  desc: string;
};

export type TopicWithProblems = {
  id: number;
  desc: string;
  problems: ProblemData[];
};

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

export function num_to_difficulty_str(num: number): string {
  if (num <= 1) {
    return 'difficulty_intro';
  } else if (num <= 4) {
    return 'difficulty_easy';
  } else if (num <= 7) {
    return 'difficulty_medium';
  } else {
    return 'difficulty_hard';
  }
}

export function difficulty_to_string(difficulty: Difficulty): string {
  switch (difficulty) {
    case 'Intro': {
      return 'difficulty_intro';
    }
    case 'Easy': {
      return 'difficulty_easy';
    }
    case 'Medium': {
      return 'difficulty_medium';
    }
    case 'Hard': {
      return 'difficulty_hard';
    }
  }
}

export function difficulty_in_range(
  difficulty: number,
  starting: Difficulty,
  ending: Difficulty
): boolean {
  let starting_num: number, ending_num: number;
  switch (starting) {
    case 'Intro': {
      starting_num = 0;
      break;
    }
    case 'Easy': {
      starting_num = 2;
      break;
    }

    case 'Medium': {
      starting_num = 5;
      break;
    }

    case 'Hard': {
      starting_num = 8;
      break;
    }
  }
  switch (ending) {
    case 'Intro': {
      ending_num = 1;
      break;
    }
    case 'Easy': {
      ending_num = 4;
      break;
    }

    case 'Medium': {
      ending_num = 7;
      break;
    }

    case 'Hard': {
      ending_num = 10;
      break;
    }
  }

  return difficulty >= starting_num && difficulty <= ending_num;
}

/// Should match the ProblemSetSpec of the backend
export type ProblemSetSpec = {
  topics: number[];
  exclusions: number[];
  starting_difficulty: Difficulty;
  ending_difficulty: Difficulty;
  n: number;
  options: SetOptions;
};

export type SetState = { id: number; set: ProblemSetSpec };

export const defaultProblemSet: ProblemSetSpec = {
  topics: [],
  exclusions: [],
  starting_difficulty: 'Intro',
  ending_difficulty: 'Hard',
  n: 20,
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
  max_prefix_group: number;
};

export const defaultDocumentOptions: DocumentOptions = {
  font_size: 10,
  title: '',
  answer_columns: 3,
  lang: 'Sv',
  write_solutions: 'First',
  color: true,
  paper_size: 'A4',
  x_margin: 20,
  y_margin: 20,
  par_spacing: null,
  max_prefix_group: 3
};
