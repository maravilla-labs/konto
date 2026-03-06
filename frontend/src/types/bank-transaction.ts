export interface BankTransaction {
  id: string;
  bank_account_id: string;
  transaction_date: string;
  value_date: string;
  amount: string;
  currency: string;
  description: string;
  counterparty_name: string | null;
  counterparty_iban: string | null;
  reference: string | null;
  bank_reference: string | null;
  status: 'unmatched' | 'matched' | 'ignored';
  matched_invoice_id: string | null;
  matched_expense_id: string | null;
  matched_journal_entry_id: string | null;
  import_batch_id: string | null;
}

export interface ManualMatchData {
  target_type: 'invoice' | 'expense';
  target_id: string;
}

export interface CreateJournalFromTx {
  debit_account_id: string;
  credit_account_id: string;
}

export interface AutoMatchResult {
  matched_count: number;
  unmatched_count: number;
}

export interface ImportResult {
  imported_count: number;
  batch_id: string;
}

export interface BankTransactionListParams {
  page?: number;
  per_page?: number;
  bank_account_id?: string;
  status?: string;
  format?: string;
}
