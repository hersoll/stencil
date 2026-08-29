type WithoutKind<T> = Omit<T, 'kind'>;

export function stripKind<T extends { kind: string }>(
  entry: T
): WithoutKind<T> {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { kind, ...rest } = entry;
  return rest;
}

export type Kind = 'problem' | 'topic' | 'chapter' | 'course' | 'prefix';

//==========================================
//  "Enum"
//==========================================

export type Entry =
  CourseEntry | ChapterEntry | TopicEntry | ProblemEntry | PrefixEntry;

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
  question: string;
  answer: string;
  solution: string;
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
//  Entries (raw, backend copies)
//==========================================

export type CourseEntryRaw = {
  id: number;
  name: string;
  desc: DescriptionTranslations;
  chapter_ids: number[];
  public: boolean;
  is_new: boolean;
};

export type ChapterEntryRaw = {
  id: number;
  name: string;
  desc: DescriptionTranslations;
  course_ids: number[];
  topic_ids: number[];
  public: boolean;
  is_new: boolean;
};

export type ProblemIdsAndDifficulties = {
  problem_id: number;
  topic_id: number;
  absolute_difficulty: number;
  relative_difficulty: number;
};

export type TopicEntryRaw = {
  id: number;
  name: string;
  desc: DescriptionTranslations;
  chapter_ids: number[];
  problems: ProblemIdsAndDifficulties[];
  public: boolean;
  is_new: boolean;
};

export type TopicDifficultyData = {
  topic_id: number;
  absolute_difficulty: number;
  relative_difficulty: number;
};

export type ProblemEntryRaw = {
  id: number;
  name: string;
  desc: DescriptionTranslations;
  module: string;
  prefix_id: number | null;
  translations: ProblemTranslations;
  topic_data: TopicDifficultyData[];
  public: boolean;
  is_new: boolean;
};

export type PrefixEntryRaw = {
  id: number;
  name: string;
  translations: PrefixTranslations;
  public: boolean;
};

//==========================================
//  Entries (enriched with kind for discrimination)
//==========================================

export type CourseEntry = CourseEntryRaw & {
  kind: 'course';
};

export type ChapterEntry = ChapterEntryRaw & {
  kind: 'chapter';
};

export type TopicEntry = TopicEntryRaw & {
  kind: 'topic';
};

export type ProblemEntry = ProblemEntryRaw & {
  kind: 'problem';
};

export type PrefixEntry = PrefixEntryRaw & {
  kind: 'prefix';
};

//==========================================
//  Entries (enriched with kind for discrimination)
//==========================================

export const defaultDescriptionTranslations = {
  sv: '',
  en: ''
};

export const defaultTranslatedProblem = {
  question: '',
  answer: '',
  solution: ''
};

export const defaultProblemTranslations = {
  sv: { ...defaultTranslatedProblem },
  en: { ...defaultTranslatedProblem }
};

export const defaultTranslatedPrefix = {
  text: '',
  group_text: ''
};

export const defaultPrefixTranslations = {
  sv: { ...defaultTranslatedPrefix },
  en: { ...defaultTranslatedPrefix }
};

export const defaultCourseEntry = {
  kind: 'course',
  id: -1,
  name: '',
  desc: { ...defaultDescriptionTranslations },
  chapter_ids: [],
  public: false,
  is_new: false
} satisfies CourseEntry;

export const defaultChapterEntry = {
  kind: 'chapter',
  id: -1,
  name: '',
  desc: { ...defaultDescriptionTranslations },
  course_ids: [],
  topic_ids: [],
  public: false,
  is_new: false
} satisfies ChapterEntry;

export const defaultTopicEntry = {
  kind: 'topic',
  id: -1,
  name: '',
  desc: { ...defaultDescriptionTranslations },
  chapter_ids: [],
  problems: [],
  public: false,
  is_new: false
} satisfies TopicEntry;

export const defaultProblemEntry = {
  kind: 'problem',
  id: -1,
  module: '',
  name: '',
  desc: { ...defaultDescriptionTranslations },
  prefix_id: null,
  translations: {
    sv: { ...defaultTranslatedProblem },
    en: { ...defaultTranslatedProblem }
  },
  topic_data: [],
  public: false,
  is_new: false
} satisfies ProblemEntry;

export const defaultPrefixEntry = {
  kind: 'prefix',
  id: -1,
  name: '',
  translations: {
    sv: { ...defaultTranslatedPrefix },
    en: { ...defaultTranslatedPrefix }
  },
  public: false
} satisfies PrefixEntry;
