export interface DashboardStats {
  account_count: number;
  active_contacts: number;
  journal_entry_count: number;
  active_projects: number;
  revenue_mtd: string;
  expenses_mtd: string;
  cash_balance: string;
  open_invoices_count: number;
  total_outstanding: string;
  recent_entries: RecentJournalEntry[];
}

export interface RecentJournalEntry {
  id: string;
  date: string;
  reference: string | null;
  description: string;
  status: string;
}
