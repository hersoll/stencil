import { init, locale, register } from 'svelte-i18n';

const API_URL = import.meta.env.VITE_API_URL || '/api';
const DEFAULT_LANGUAGE = 'sv';

async function fetchTranslation(locale: string) {
  const cached = localStorage.getItem(`translations_${locale}`);
  if (cached) {
    const { data, timestamp } = JSON.parse(cached);
    if (Date.now() - timestamp < 60 * 60 * 1000) { // 60 minutes
      return data;
    }
  }

  try {
    const res = await fetch(`${API_URL}/translations/${locale}`);

    if (!res.ok) {
      throw new Error(`Failed to load translations from server: ${res.status}`);
    }
    const data = await res.json();

    localStorage.setItem(`translations_${locale}`, JSON.stringify({ data, timestamp: Date.now() }));
    localStorage.setItem('lang', locale);

    return data;
  } catch (error) {
    console.error(`Error loading ${locale} translations:`, error);

    return {};
  }
}

function getLanguageChoice() {
  let cached = localStorage.getItem('lang');
  return cached || DEFAULT_LANGUAGE;
}

register('sv', () => fetchTranslation('sv'));
register('en', () => fetchTranslation('en'));

init({
  fallbackLocale: 'sv',
  initialLocale: getLanguageChoice(),
  loadingDelay: 200,
});

export { locale };
export function setLanguage(lang: string) {
  locale.set(lang);
  localStorage.setItem('lang', lang);
}
export const availableLocales = ['en', 'sv'];
