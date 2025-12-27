//==========================================
//  "Enum"
//==========================================

export type Entry =
  | CourseEntry
  | ChapterEntry
  | TopicEntry
  | ProblemEntry
  | PrefixEntry;

//==========================================
//  Utility Types
//==========================================

export type DescriptionTranslations = {
  sv: string;
  en: string;
};

export type ProblemTranslations = {
  sv: TranslatedProblem;
  en: TranslatedProblem;
};

export type TranslatedProblem = {
  question: string | null;
  answer: string | null;
  solution: string | null;
};

export type PrefixTranslations = {
  sv: TranslatedPrefix;
  en: TranslatedPrefix;
};

export type TranslatedPrefix = {
  text: string;
  group_text: string;
};

//==========================================
//  Entries
//==========================================

export type CourseEntry = {
  id: number;
  name: string;
  desc: DescriptionTranslations;
};

export type ChapterEntry = {
  id: number;
  name: string;
  desc: DescriptionTranslations;
};

export type TopicEntry = {
  id: number;
  name: string;
  desc: DescriptionTranslations;
};

export type ProblemEntry = {
  id: number;
  module: string;
  name: string;
  desc: DescriptionTranslations;
  difficulty: number;
  prefix_id: number | null;
  translations: ProblemTranslations;
};

export type PrefixEntry = {
  id: number;
  name: string;
  translations: PrefixTranslations;
};

//==========================================
//  Defaults
//==========================================

export const defaultProblemEntry: ProblemEntry = {
  id: -1,
  module: '',
  name: '',
  difficulty: 0,
  prefix_id: null,
  desc: { sv: '', en: '' },
  translations: {
    sv: { question: null, answer: null, solution: null },
    en: { question: null, answer: null, solution: null }
  }
};
