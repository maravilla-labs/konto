import type { CreateInvoiceLine } from '@/types/invoice';
import type { VatRate } from '@/types/vat-rate';

export interface InvoiceFormData {
  contact_id: string;
  project_id: string;
  issue_date: string;
  due_date: string;
  language: string;
  notes: string;
  payment_terms: string;
  header_text: string;
  footer_text: string;
  contact_person_id: string;
  default_vat_rate_id: string;
  default_account_id: string;
  bank_account_id: string;
  lines: LineFormData[];
}

export interface LineFormData {
  _key: string;
  description: string;
  quantity: string;
  unit_price: string;
  vat_rate_id: string;
  account_id: string;
  discount_percent: string;
}

export function emptyLine(): LineFormData {
  return {
    _key: crypto.randomUUID(),
    description: '',
    quantity: '1',
    unit_price: '',
    vat_rate_id: '',
    account_id: '',
    discount_percent: '',
  };
}

export function toCreateLines(lines: LineFormData[], defaultVatRateId?: string, defaultAccountId?: string): CreateInvoiceLine[] {
  return lines.map((l) => ({
    description: l.description,
    quantity: parseFloat(l.quantity || '0'),
    unit_price: parseFloat(l.unit_price || '0'),
    vat_rate_id: l.vat_rate_id || defaultVatRateId || undefined,
    account_id: l.account_id || defaultAccountId || undefined,
    discount_percent: l.discount_percent ? parseFloat(l.discount_percent) : undefined,
  }));
}

export interface LineTotals {
  lineSubtotals: number[];
  subtotal: number;
  vatByRate: Map<string, { name: string; rate: number; amount: number }>;
  totalVat: number;
  grandTotal: number;
}

function computeLineTotal(line: LineFormData): number {
  const qty = parseFloat(line.quantity || '0');
  const price = parseFloat(line.unit_price || '0');
  const discount = parseFloat(line.discount_percent || '0');
  return qty * price * (1 - discount / 100);
}

export function computeLineTotals(
  lines: LineFormData[],
  vatRates: VatRate[],
  defaultVatRateId: string,
): LineTotals {
  const activeVatRates = vatRates.filter((v) => v.is_active);
  const lineSubtotals = lines.map(computeLineTotal);
  const subtotal = lineSubtotals.reduce((a, b) => a + b, 0);

  const vatByRate = new Map<string, { name: string; rate: number; amount: number }>();
  lines.forEach((line, i) => {
    const effectiveVatId = line.vat_rate_id || defaultVatRateId;
    if (!effectiveVatId) return;
    const vat = activeVatRates.find((v) => v.id === effectiveVatId);
    if (!vat) return;
    const vatAmount = lineSubtotals[i] * vat.rate / 100;
    const existing = vatByRate.get(effectiveVatId);
    if (existing) {
      existing.amount += vatAmount;
    } else {
      vatByRate.set(effectiveVatId, { name: vat.name, rate: vat.rate, amount: vatAmount });
    }
  });

  const totalVat = Array.from(vatByRate.values()).reduce((a, b) => a + b.amount, 0);
  const grandTotal = subtotal + totalVat;

  return { lineSubtotals, subtotal, vatByRate, totalVat, grandTotal };
}
