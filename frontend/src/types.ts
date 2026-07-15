export type View = 'addSet' | 'layout' | 'pdf' | 'editSet';

export type ProblemData = {
  id: number;
  absoluteDifficulty: number;
  desc: string;
};

export type TopicWithProblems = {
  id: number;
  desc: string;
  problems: ProblemData[];
};

export type TopicData = {
  id: number;
  desc: string;
};

export type ChapterWithTopics = {
  id: number;
  desc: string;
  topics: TopicData[];
};

export type CourseData = {
  name: string;
  id: number;
  desc: string;
};

export type SetState = {
  id: number;
  options: ProblemSetSpec;
  topics: TopicWithProblems[];
};

export type Difficulty =
  | 'difficulty_intro'
  | 'difficulty_easy'
  | 'difficulty_medium'
  | 'difficulty_hard';

export function numToDifficultyStr(num: number): string {
  if (num <= 3) {
    return 'difficulty_intro';
  } else if (num <= 5) {
    return 'difficulty_easy';
  } else if (num <= 7) {
    return 'difficulty_medium';
  } else {
    return 'difficulty_hard';
  }
}

export function difficultyInRange(
  difficulty: number,
  starting: Difficulty,
  ending: Difficulty
): boolean {
  let startingNum: number, endingNum: number;
  switch (starting) {
    case 'difficulty_intro': {
      startingNum = 1;
      break;
    }
    case 'difficulty_easy': {
      startingNum = 4;
      break;
    }

    case 'difficulty_medium': {
      startingNum = 6;
      break;
    }

    case 'difficulty_hard': {
      startingNum = 8;
      break;
    }
  }
  switch (ending) {
    case 'difficulty_intro': {
      endingNum = 3;
      break;
    }
    case 'difficulty_easy': {
      endingNum = 5;
      break;
    }

    case 'difficulty_medium': {
      endingNum = 7;
      break;
    }

    case 'difficulty_hard': {
      endingNum = 10;
      break;
    }
  }

  return difficulty >= startingNum && difficulty <= endingNum;
}

export type FormattingOptions = {
  questionColumns: number;
  heading: string | null;
  spacing: number | null;
  pagebreakAfter: boolean;
};

export type ProblemOptions = {
  topics: number[];
  exclusions: number[];
  startingDifficulty: Difficulty;
  endingDifficulty: Difficulty;
  n: number;
};

/// Should match the ProblemSetSpec of the backend
export type ProblemSetSpec = {
  problem_options: ProblemOptions;
  formatting_options: FormattingOptions;
};

export type DocumentOptions = {
  fontSize: number;
  title: string | null;
  subtitle: string | null;
  nameField: boolean;
  answerColumns: number;
  lang: 'Sv' | 'En';
  writeSolutions: 'None' | 'First' | 'All';
  color: boolean;
  paperSize: 'A4' | 'A5';
  xMargin: number;
  yMargin: number;
  parSpacing: number | null;
  maxPrefixGroup: number;
  pageBreakBeforeAnswers: boolean;
};

// NOTE: Overridden by backend on defaults startup
export const initialProblemOptions: ProblemOptions = {
  topics: [],
  exclusions: [],
  startingDifficulty: 'difficulty_intro',
  endingDifficulty: 'difficulty_hard',
  n: 20
};

// NOTE: Overridden by backend on defaults startup
export const initialFormattingOptions: FormattingOptions = {
  questionColumns: 2,
  heading: null,
  spacing: null,
  pagebreakAfter: false
};

// NOTE: Overridden by backend defaults on startup
export const initialDocumentOptions: DocumentOptions = {
  fontSize: 10,
  title: null,
  subtitle: null,
  nameField: false,
  answerColumns: 3,
  lang: 'Sv',
  writeSolutions: 'First',
  color: true,
  paperSize: 'A4',
  xMargin: 20,
  yMargin: 20,
  parSpacing: null,
  maxPrefixGroup: 3,
  pageBreakBeforeAnswers: true
};
