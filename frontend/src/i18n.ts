import { derived, get, writable } from "svelte/store";

export type Translations = Record<string, any>;
// Params are what will be replaced in strings with {}.
export type TranslationParams = Record<string, string | number>;
export interface TranslationFunction {
  (key: string, params?: TranslationParams): string;
}

const API_URL = import.meta.env.VITE_API_URL || '/api';
const DEFAULT_LANGUAGE = 'sv';

export const currentLanguage = writable(DEFAULT_LANGUAGE);
const translations = writable<Record<string, Translations>>({});
export const translationLoading = writable(false);

function interpolate(text: string, params: TranslationParams = {}): string {
  return text.replace(/{(\w+)}/g, (match, key) => {
    const value = params[key];
    return value !== undefined ? String(value) : match;
  });
}

export const t = derived(
  [currentLanguage, translations],
  ([$currentLanguage, $translations]): TranslationFunction => {
    return (key: string, params: TranslationParams = {}): string => {
      const translationRecord = $translations[$currentLanguage];

      // Will happen during init
      if (!translationRecord) {
        return key;
      }
      const translation = translationRecord[key];

      if (!translation) {
        console.warn(`Translation not found for key: ${key} in lang: ${$currentLanguage}`);
        return key;
      }

      return interpolate(translation, params);
    };
  }
);

async function fetchTranslation(lang: string) {
  const cached = localStorage.getItem(`translations_${lang}`);
  if (cached) {
    const { data, timestamp } = JSON.parse(cached);
    if (Date.now() - timestamp < 60 * 60 * 1000) { // 60 minutes
      return data;
    }
  }

  try {
    translationLoading.set(true);

    const res = await fetch(`${API_URL}/translations/${lang}`);

    if (!res.ok) {
      throw new Error(`Failed to load translations from server: ${res.status}`);
    }

    const data = await res.json();
    localStorage.setItem(`translations_${lang}`, JSON.stringify({ data, timestamp: Date.now() }));

    return data;
  } catch (error) {
    console.error(`Error loading ${lang} translations:`, error);
    return {};
  } finally {
    translationLoading.set(false);
  }
}

export async function setLanguage(lang: string) {
  const loadedTranslations = get(translations);

  const data = loadedTranslations[lang] || await fetchTranslation(lang);

  translations.update(current => ({
    ...current,
    [lang]: data
  }));

  currentLanguage.set(lang);
  localStorage.setItem('lang', lang);
}

export async function initI18n() {
  let cached = localStorage.getItem('lang');
  const lang = cached || DEFAULT_LANGUAGE;

  await setLanguage(lang);
}

export const availableLanguages = ['sv', 'en'];
