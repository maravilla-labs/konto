export type RecurringFrequency = 'monthly' | 'quarterly' | 'semi_annual' | 'annual' | 'custom';

export interface RecurringInvoiceLineTemplate {
  description: string;
  quantity: number;
  unit_price: number;
  vat_rate_id?: string;
  account_id: string;
}

export interface RecurringInvoiceTemplateData {
  language?: string;
  currency_id?: string;
  notes?: string;
  payment_terms?: string;
  lines: RecurringInvoiceLineTemplate[];
}

export interface RecurringInvoice {
  id: string;
  contact_id: string;
  project_id: string | null;
  template_data: RecurringInvoiceTemplateData;
  frequency: RecurringFrequency;
  interval_days: number | null;
  next_run_date: string;
  end_date: string | null;
  auto_send: boolean;
  is_active: boolean;
  last_generated_at: string | null;
  created_by: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateRecurringInvoice {
  contact_id: string;
  project_id?: string;
  frequency: RecurringFrequency;
  interval_days?: number;
  next_run_date: string;
  end_date?: string;
  auto_send?: boolean;
  language?: string;
  currency_id?: string;
  notes?: string;
  payment_terms?: string;
  lines: RecurringInvoiceLineTemplate[];
}

export type UpdateRecurringInvoice = CreateRecurringInvoice & {
  is_active?: boolean;
};

export interface RecurringInvoiceListParams {
  page?: number;
  per_page?: number;
  is_active?: boolean;
  search?: string;
}
