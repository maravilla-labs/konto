export type CreditNoteStatus = 'draft' | 'issued' | 'applied' | 'cancelled';

export interface CreditNote {
  id: string;
  credit_note_number: string | null;
  invoice_id: string | null;
  contact_id: string;
  status: CreditNoteStatus;
  issue_date: string;
  currency_id: string | null;
  subtotal: string;
  vat_amount: string;
  total: string;
  notes: string | null;
  journal_entry_id: string | null;
  created_by: string | null;
}

export interface CreditNoteLine {
  id: string;
  credit_note_id: string;
  sort_order: number;
  description: string;
  quantity: string;
  unit_price: string;
  vat_rate_id: string | null;
  vat_amount: string;
  line_total: string;
  account_id: string;
}

export interface CreditNoteDetail extends CreditNote {
  lines: CreditNoteLine[];
  contact_name: string | null;
  invoice_number: string | null;
}

export interface CreateCreditNoteLine {
  description: string;
  quantity: number;
  unit_price: number;
  vat_rate_id?: string;
  account_id: string;
}

export interface CreateCreditNote {
  contact_id: string;
  invoice_id?: string;
  issue_date: string;
  currency_id?: string;
  notes?: string;
  lines: CreateCreditNoteLine[];
}

export type UpdateCreditNote = CreateCreditNote;

export interface CreditNoteListParams {
  page?: number;
  per_page?: number;
  status?: string;
  search?: string;
}
