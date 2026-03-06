export interface TrialBalanceEntry {
  account_id: string;
  account_number: number;
  account_name: string;
  account_type: string;
  total_debit: string;
  total_credit: string;
  balance: string;
}

export interface BalanceSheetResponse {
  assets: AccountBalance[];
  liabilities: AccountBalance[];
  equity: AccountBalance[];
  total_assets: string;
  total_liabilities_equity: string;
  as_of: string;
}

export interface AccountBalance {
  account_id: string;
  account_number: number;
  account_name: string;
  balance: string;
}

export interface ProfitLossResponse {
  revenue: AccountBalance[];
  expenses: AccountBalance[];
  total_revenue: string;
  total_expenses: string;
  net_income: string;
  from_date: string;
  to_date: string;
}

export interface AccountLedgerEntry {
  date: string;
  description: string;
  debit: string;
  credit: string;
  running_balance: string;
  entry_id: string;
}

export interface VatReportEntry {
  vat_code: string;
  vat_name: string;
  rate: string;
  vat_type: string;
  taxable_amount: string;
  vat_amount: string;
}

export interface VatReportResponse {
  vat_method: string;
  output_entries: VatReportEntry[];
  input_entries: VatReportEntry[];
  total_output_taxable: string;
  total_output_vat: string;
  total_input_taxable: string;
  total_input_vat: string;
  net_vat_owed: string;
  from_date: string;
  to_date: string;
  flat_rate_percentage?: string | null;
  gross_revenue?: string | null;
  flat_rate_vat_owed?: string | null;
  collected_vat?: string | null;
  saldo_ertrag?: string | null;
}

export interface VatPaymentResponse {
  journal_entry_id: string;
  description: string;
  vat_owed: string;
  saldo_ertrag: string;
  bank_payment: string;
}

export interface ExportVatXmlRequest {
  from_date: string;
  to_date: string;
  type_of_submission: number;
  form_of_reporting?: number;
  business_reference_id?: string;
  total_consideration: string;
  supplies_to_foreign?: string;
  supplies_abroad?: string;
  transfer_notification?: string;
  supplies_exempt?: string;
  reduction_of_consideration?: string;
  various_deduction?: string;
  subsidies?: string;
  donations?: string;
}
