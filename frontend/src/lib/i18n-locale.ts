import { normalizeLanguage } from '@/lib/language';

export function localeForLanguage(language: string | undefined): string {
  const normalized = normalizeLanguage(language);
  switch (normalized) {
    case 'de':
      return 'de-CH';
    case 'fr':
      return 'fr-CH';
    case 'it':
      return 'it-CH';
    case 'en':
    default:
      return 'en-US';
  }
}

export function formatMonthLabel(month: string, language: string | undefined): string {
  const [yearPart, monthPart] = month.split('-');
  const year = Number(yearPart);
  const monthIndex = Number(monthPart) - 1;
  const date = new Date(Date.UTC(year, monthIndex, 1));
  return date.toLocaleDateString(localeForLanguage(language), {
    month: 'short',
    year: '2-digit',
  });
}
