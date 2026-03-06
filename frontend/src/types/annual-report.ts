export interface Shareholder {
  id: string;
  name: string;
  city: string;
  role: string;
  signing_rights: string | null;
  sort_order: number;
}

export interface CreateShareholder {
  name: string;
  city: string;
  role: string;
  signing_rights?: string | null;
  sort_order?: number;
}

export type UpdateShareholder = CreateShareholder;

export interface AnnualReportNote {
  id: string;
  fiscal_year_id: string;
  section_key: string;
  content: Record<string, unknown>;
  sort_order: number;
  label: string;
  section_type: string;
}

export interface UpdateNoteRequest {
  content: Record<string, unknown>;
  label?: string;
  sort_order?: number;
}

export interface CreateNoteRequest {
  label: string;
  sort_order?: number;
}

export interface AnnualReport {
  id: string;
  fiscal_year_id: string;
  status: string;
  generated_at: string | null;
  generated_by: string | null;
  pdf_path: string | null;
}

// Swiss Report Types
export interface AccountGroupResult {
  label: string;
  total_label: string;
  accounts: TrialBalanceRow[];
  subtotal: number;
}

export interface GroupedSection {
  key: string;
  label: string;
  groups: AccountGroupResult[];
  total: number;
}

export interface TrialBalanceRow {
  account_id: string;
  account_number: number;
  account_name: string;
  account_type: string;
  total_debit: number;
  total_credit: number;
  balance: number;
}

export interface SwissBalanceSheet {
  as_of: string;
  assets: GroupedSection[];
  liabilities: GroupedSection[];
  total_assets: number;
  total_liabilities: number;
}

export interface SwissIncomeSubtotals {
  operating_revenue: number;
  gross_profit_material: number;
  gross_profit_personnel: number;
  ebitda: number;
  ebit: number;
  ebt: number;
  net_result: number;
}

export interface SwissIncomeStatement {
  from_date: string;
  to_date: string;
  sections: GroupedSection[];
  subtotals: SwissIncomeSubtotals;
}

