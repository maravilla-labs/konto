export interface CurrencyExchange {
  id: string;
  from_bank_account_id: string;
  to_bank_account_id: string;
  from_amount: string;
  to_amount: string;
  date: string;
  journal_entry_id: string;
  notes: string | null;
}

export interface RecordTransfer {
  from_bank_account_id: string;
  to_bank_account_id: string;
  from_amount: string;
  to_amount: string;
  date: string;
  notes?: string;
}
