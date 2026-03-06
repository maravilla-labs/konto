export interface InvoicePayment {
  id: string;
  invoice_id: string;
  amount: string;
  payment_date: string;
  payment_method: string | null;
  reference: string | null;
  bank_transaction_id: string | null;
  journal_entry_id: string | null;
  created_at: string;
}

export interface RecordPaymentData {
  amount: number;
  payment_date: string;
  payment_account_id: string;
  payment_method?: string;
  reference?: string;
}
