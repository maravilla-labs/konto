use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams)]
pub struct DateRangeParams {
    pub from_date: String,
    pub to_date: String,
    pub format: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct AsOfDateParams {
    pub as_of: Option<String>,
    pub format: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct LedgerParams {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TrialBalanceRowResponse {
    pub account_id: String,
    pub account_number: i32,
    pub account_name: String,
    pub account_type: String,
    #[schema(value_type = String)]
    pub total_debit: Decimal,
    #[schema(value_type = String)]
    pub total_credit: Decimal,
    #[schema(value_type = String)]
    pub balance: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BalanceSheetResponse {
    pub as_of: String,
    pub assets: Vec<TrialBalanceRowResponse>,
    pub liabilities: Vec<TrialBalanceRowResponse>,
    pub equity: Vec<TrialBalanceRowResponse>,
    #[schema(value_type = String)]
    pub total_assets: Decimal,
    #[schema(value_type = String)]
    pub total_liabilities_equity: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfitLossResponse {
    pub from_date: String,
    pub to_date: String,
    pub revenue: Vec<TrialBalanceRowResponse>,
    pub expenses: Vec<TrialBalanceRowResponse>,
    #[schema(value_type = String)]
    pub total_revenue: Decimal,
    #[schema(value_type = String)]
    pub total_expenses: Decimal,
    #[schema(value_type = String)]
    pub net_income: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LedgerLineResponse {
    pub date: String,
    pub entry_id: String,
    pub description: String,
    #[schema(value_type = String)]
    pub debit: Decimal,
    #[schema(value_type = String)]
    pub credit: Decimal,
    #[schema(value_type = String)]
    pub running_balance: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VatReportEntryResponse {
    pub vat_code: String,
    pub vat_name: String,
    #[schema(value_type = String)]
    pub rate: Decimal,
    pub vat_type: String,
    #[schema(value_type = String)]
    pub taxable_amount: Decimal,
    #[schema(value_type = String)]
    pub vat_amount: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VatReportResponse {
    pub vat_method: String,
    pub output_entries: Vec<VatReportEntryResponse>,
    pub input_entries: Vec<VatReportEntryResponse>,
    #[schema(value_type = String)]
    pub total_output_taxable: Decimal,
    #[schema(value_type = String)]
    pub total_output_vat: Decimal,
    #[schema(value_type = String)]
    pub total_input_taxable: Decimal,
    #[schema(value_type = String)]
    pub total_input_vat: Decimal,
    #[schema(value_type = String)]
    pub net_vat_owed: Decimal,
    pub from_date: String,
    pub to_date: String,
    /// SSS (Saldosteuersatz) fields — only present when vat_method = "flat_rate"
    #[schema(value_type = Option<String>)]
    pub flat_rate_percentage: Option<Decimal>,
    #[schema(value_type = Option<String>)]
    pub gross_revenue: Option<Decimal>,
    #[schema(value_type = Option<String>)]
    pub flat_rate_vat_owed: Option<Decimal>,
    #[schema(value_type = Option<String>)]
    pub collected_vat: Option<Decimal>,
    #[schema(value_type = Option<String>)]
    pub saldo_ertrag: Option<Decimal>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardStatsResponse {
    pub account_count: u64,
    pub active_contacts: u64,
    pub journal_entry_count: u64,
    pub active_projects: u64,
    #[schema(value_type = String)]
    pub revenue_mtd: Decimal,
    #[schema(value_type = String)]
    pub expenses_mtd: Decimal,
    #[schema(value_type = String)]
    pub cash_balance: Decimal,
    pub open_invoices_count: u64,
    #[schema(value_type = String)]
    pub total_outstanding: Decimal,
    pub recent_entries: Vec<RecentEntryResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecentEntryResponse {
    pub id: String,
    pub date: String,
    pub reference: Option<String>,
    pub description: String,
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateVatPaymentRequest {
    pub from_date: String,
    pub to_date: String,
    pub payment_date: String,
    pub bank_account_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VatPaymentResponse {
    pub journal_entry_id: String,
    pub description: String,
    #[schema(value_type = String)]
    pub vat_owed: Decimal,
    #[schema(value_type = String)]
    pub saldo_ertrag: Decimal,
    #[schema(value_type = String)]
    pub bank_payment: Decimal,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExportVatXmlRequest {
    pub from_date: String,
    pub to_date: String,
    /// 1 = Initial, 2 = Correction, 3 = Annual reconciliation
    pub type_of_submission: i32,
    /// 1 = Agreed (default), 2 = Received
    pub form_of_reporting: Option<i32>,
    pub business_reference_id: Option<String>,
    /// Turnover computation fields (Decimal as string)
    #[schema(value_type = String)]
    pub total_consideration: String,
    #[schema(value_type = Option<String>)]
    pub supplies_to_foreign: Option<String>,
    #[schema(value_type = Option<String>)]
    pub supplies_abroad: Option<String>,
    #[schema(value_type = Option<String>)]
    pub transfer_notification: Option<String>,
    #[schema(value_type = Option<String>)]
    pub supplies_exempt: Option<String>,
    #[schema(value_type = Option<String>)]
    pub reduction_of_consideration: Option<String>,
    #[schema(value_type = Option<String>)]
    pub various_deduction: Option<String>,
    /// Other flows
    #[schema(value_type = Option<String>)]
    pub subsidies: Option<String>,
    #[schema(value_type = Option<String>)]
    pub donations: Option<String>,
}
