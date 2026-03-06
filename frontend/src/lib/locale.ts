/**
 * Locale formatting utilities that respect company settings.
 * Supported number_format values: 'ch' (1'234.56), 'de' (1.234,56), 'en' (1,234.56)
 * Supported date_format values: 'dd.MM.yyyy', 'MM/dd/yyyy', 'yyyy-MM-dd'
 */

export type NumberFormat = 'ch' | 'de' | 'en';
export type DateFormat = 'dd.MM.yyyy' | 'MM/dd/yyyy' | 'yyyy-MM-dd';

export function normalizeNumberFormat(value: string | undefined): NumberFormat {
  if (value === 'de' || value === 'en' || value === 'ch') return value;
  return 'ch';
}

export function normalizeDateFormat(value: string | undefined): DateFormat {
  if (value === 'dd.MM.yyyy' || value === 'MM/dd/yyyy' || value === 'yyyy-MM-dd') return value;
  return 'dd.MM.yyyy';
}

export function formatNumber(
  value: number | string,
  numberFormat: NumberFormat | string = 'ch',
  decimals = 2,
): string {
  const num = typeof value === 'string' ? parseFloat(value) : value;
  if (isNaN(num)) return '—';
  const normalizedFormat = normalizeNumberFormat(typeof numberFormat === 'string' ? numberFormat : undefined);

  const parts = num.toFixed(decimals).split('.');
  const intPart = parts[0];
  const decPart = parts[1];

  const grouped = intPart.replace(/\B(?=(\d{3})+(?!\d))/g, getThousandsSep(normalizedFormat));
  return `${grouped}${getDecimalSep(normalizedFormat)}${decPart}`;
}

export function formatCurrency(
  value: number | string,
  currency = 'CHF',
  numberFormat: NumberFormat | string = 'ch',
): string {
  return `${currency} ${formatNumber(value, numberFormat)}`;
}

export function formatDate(
  dateStr: string,
  dateFormat: DateFormat | string = 'dd.MM.yyyy',
): string {
  if (!dateStr) return '—';
  const normalizedFormat = normalizeDateFormat(typeof dateFormat === 'string' ? dateFormat : undefined);
  const parts = dateStr.split('-');
  if (parts.length !== 3) return dateStr;

  const [yyyy, mm, dd] = parts;

  switch (normalizedFormat) {
    case 'dd.MM.yyyy':
      return `${dd}.${mm}.${yyyy}`;
    case 'MM/dd/yyyy':
      return `${mm}/${dd}/${yyyy}`;
    case 'yyyy-MM-dd':
      return dateStr;
    default:
      return dateStr;
  }
}

function getThousandsSep(format: NumberFormat): string {
  switch (format) {
    case 'ch': return '\u2019'; // right single quotation mark (Swiss standard)
    case 'de': return '.';
    case 'en': return ',';
  }
}

function getDecimalSep(format: NumberFormat): string {
  switch (format) {
    case 'ch': return '.';
    case 'de': return ',';
    case 'en': return '.';
  }
}
