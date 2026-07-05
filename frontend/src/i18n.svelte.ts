type Translations = Record<string, any>;
// Params are what will be replaced in strings with {}.
type TranslationParams = Record<string, string | number>;

const API_URL = import.meta.env.VITE_API_URL || '/api';
const DEFAULT_LANGUAGE = 'sv';

function interpolate(text: string, params: TranslationParams = {}): string {
  return text.replace(/{(\w+)}/g, (match, key) => {
    const value = params[key];
    return value !== undefined ? String(value) : match;
  });
}

class I18n {
  currentLanguage = $state(DEFAULT_LANGUAGE);
  translations = $state<Record<string, Translations>>({});
  loading = $state(false);

  t = $derived.by(() => {
    return (key: string, params: TranslationParams = {}): string => {
      const translationRecord = this.translations[this.currentLanguage];

      // Will happen during init
      if (!translationRecord) {
        return key;
      }
      const translation = translationRecord[key];

      if (!translation) {
        console.warn(
          `Translation not found for key: ${key} in lang: ${this.currentLanguage}`
        );
        return key;
      }

      return interpolate(translation, params);
    };
  });

  async fetchTranslation(lang: string) {
    try {
      this.loading = true;

      const res = await fetch(`${API_URL}/translations/${lang}`);

      if (!res.ok) {
        throw new Error(
          `Failed to load translations from server: ${res.status}`
        );
      }

      return await res.json();
    } catch (error) {
      console.error(`Error loading ${lang} translations:`, error);
      return {};
    } finally {
      this.loading = false;
    }
  }

  async setLanguage(lang: string) {
    const data = this.translations[lang] || (await this.fetchTranslation(lang));

    this.translations = {
      ...this.translations,
      [lang]: data
    };

    this.currentLanguage = lang;
    localStorage.setItem('lang', lang);
  }

  async init() {
    let cached = localStorage.getItem('lang');
    const lang = cached || DEFAULT_LANGUAGE;

    await this.setLanguage(lang);
  }
}

export default new I18n();
