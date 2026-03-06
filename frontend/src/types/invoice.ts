export type InvoiceStatus = 'draft' | 'sent' | 'paid' | 'overdue' | 'cancelled';

export interface Invoice {
  id: string;
  invoice_number: string | null;
  contact_id: string;
  project_id: string | null;
  status: InvoiceStatus;
  issue_date: string;
  due_date: string;
  language: string | null;
  currency_id: string | null;
  subtotal: string;
  vat_amount: string;
  total: string;
  notes: string | null;
  payment_terms: string | null;
  journal_entry_id: string | null;
  payment_journal_entry_id: string | null;
  created_by: string | null;
  header_text: string | null;
  footer_text: string | null;
  contact_person_id: string | null;
  bank_account_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface InvoiceLine {
  id: string;
  invoice_id: string;
  position: number;
  description: string;
  quantity: string;
  unit_price: string;
  vat_rate_id: string | null;
  vat_amount: string;
  line_total: string;
  account_id: string;
  discount_percent: string | null;
}

export interface InvoiceDetail extends Invoice {
  lines: InvoiceLine[];
  contact_name: string | null;
  project_name: string | null;
  contact_person_name: string | null;
}

export interface CreateInvoiceLine {
  description: string;
  quantity: number;
  unit_price: number;
  vat_rate_id?: string;
  account_id?: string;
  discount_percent?: number;
}

export interface CreateInvoice {
  contact_id: string;
  project_id?: string;
  issue_date: string;
  due_date: string;
  language?: string;
  currency_id?: string;
  notes?: string;
  payment_terms?: string;
  header_text?: string;
  footer_text?: string;
  contact_person_id?: string;
  bank_account_id?: string;
  lines: CreateInvoiceLine[];
}

export type UpdateInvoice = CreateInvoice;

export interface PayInvoiceData {
  payment_date: string;
  payment_account_id: string;
}

export interface InvoiceListParams {
  page?: number;
  per_page?: number;
  status?: string;
  contact_id?: string;
  project_id?: string;
  search?: string;
}
