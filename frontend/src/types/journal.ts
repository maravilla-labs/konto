export interface JournalEntry {
  id: string;
  date: string;
  reference: string | null;
  description: string;
  status: string;
  currency_id: string | null;
  exchange_rate: string | null;
}

export interface JournalLine {
  id: string;
  account_id: string;
  debit_amount: string;
  credit_amount: string;
  description: string | null;
  vat_rate_id: string | null;
}

export interface JournalDetail {
  entry: JournalEntry;
  lines: JournalLine[];
}

export interface CreateJournalLine {
  account_id: string;
  debit_amount: string;
  credit_amount: string;
  description?: string;
  vat_rate_id?: string;
}

export interface CreateJournalEntry {
  date: string;
  description: string;
  reference?: string;
  lines: CreateJournalLine[];
}

export interface JournalAttachment {
  id: string;
  journal_entry_id: string;
  file_name: string;
  file_size: number;
  mime_type: string;
  uploaded_by: string | null;
  created_at: string;
}
