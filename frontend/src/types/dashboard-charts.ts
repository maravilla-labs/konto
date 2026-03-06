export interface MonthlyAmount {
  month: string;
  amount: string;
}

export interface InvoiceAgingBucket {
  status: string;
  count: number;
  total: string;
}

export interface OutstandingContact {
  contact_id: string;
  contact_name: string;
  outstanding_amount: string;
  invoice_count: number;
}

export interface OverviewMonth {
  month: string;
  income: string;
  expenses: string;
  cumulative_income: string;
  cumulative_expenses: string;
}

export interface OverviewData {
  year: number;
  months: OverviewMonth[];
  total_income: string;
  total_expenses: string;
  difference: string;
  available_years: number[];
}

export interface CashFlowItem {
  description: string;
  amount: string;
}

export interface CashFlowSection {
  label: string;
  inflows: string;
  outflows: string;
  net: string;
  items: CashFlowItem[];
}

export interface CashFlowReport {
  sections: CashFlowSection[];
  net_change: string;
  opening_balance: string;
  closing_balance: string;
  reconciliation_difference: string;
  from_date: string;
  to_date: string;
}

export interface AgingBucket {
  bucket: string;
  count: number;
  total: string;
}

export interface MonthlyCashFlow {
  month: string;
  inflows: string;
  outflows: string;
  net: string;
  cumulative_balance: string;
}

export interface CashFlowMonthlyReport {
  months: MonthlyCashFlow[];
  initial_balance: string;
  ending_balance: string;
  total_inflows: string;
  total_outflows: string;
  net_variation: string;
}
