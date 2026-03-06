export type SupportedLanguage = 'en' | 'de' | 'fr' | 'it';

export interface LanguageOption {
  code: SupportedLanguage;
  label: string;
}

export const SUPPORTED_LANGUAGES: LanguageOption[] = [
  { code: 'en', label: 'English' },
  { code: 'de', label: 'Deutsch' },
  { code: 'fr', label: 'Francais' },
  { code: 'it', label: 'Italiano' },
];

const languageSet = new Set<SupportedLanguage>(
  SUPPORTED_LANGUAGES.map((l) => l.code),
);

export function normalizeLanguage(value: string | null | undefined): SupportedLanguage | undefined {
  if (!value) return undefined;
  const base = value.trim().toLowerCase().split(/[-_]/)[0] as SupportedLanguage;
  return languageSet.has(base) ? base : undefined;
}
