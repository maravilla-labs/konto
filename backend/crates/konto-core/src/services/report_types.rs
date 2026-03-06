use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::ch_account_groups::GroupedSection;

#[derive(Debug, Serialize, Clone)]
pub struct TrialBalanceRow {
    pub account_id: String,
    pub account_number: i32,
    pub account_name: String,
    pub account_type: String,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub balance: Decimal,
}

#[derive(Debug, Serialize)]
pub struct BalanceSheet {
    pub as_of: String,
    pub assets: Vec<TrialBalanceRow>,
    pub liabilities: Vec<TrialBalanceRow>,
    pub equity: Vec<TrialBalanceRow>,
    pub total_assets: Decimal,
    pub total_liabilities_equity: Decimal,
}

#[derive(Debug, Serialize)]
pub struct ProfitLoss {
    pub from_date: String,
    pub to_date: String,
    pub revenue: Vec<TrialBalanceRow>,
    pub expenses: Vec<TrialBalanceRow>,
    pub total_revenue: Decimal,
    pub total_expenses: Decimal,
    pub net_income: Decimal,
}

#[derive(Debug, Serialize)]
pub struct LedgerLine {
    pub date: String,
    pub entry_id: String,
    pub description: String,
    pub debit: Decimal,
    pub credit: Decimal,
    pub running_balance: Decimal,
}

#[derive(Debug, Serialize, Clone)]
pub struct VatReportEntry {
    pub vat_code: String,
    pub vat_name: String,
    pub rate: Decimal,
    pub vat_type: String,
    pub taxable_amount: Decimal,
    pub vat_amount: Decimal,
}

#[derive(Debug, Serialize)]
pub struct VatReport {
    pub vat_method: String,
    pub output_entries: Vec<VatReportEntry>,
    pub input_entries: Vec<VatReportEntry>,
    pub total_output_taxable: Decimal,
    pub total_output_vat: Decimal,
    pub total_input_taxable: Decimal,
    pub total_input_vat: Decimal,
    pub net_vat_owed: Decimal,
    /// SSS fields (only filled when vat_method = "flat_rate")
    pub flat_rate_percentage: Option<Decimal>,
    pub gross_revenue: Option<Decimal>,
    pub flat_rate_vat_owed: Option<Decimal>,
    pub collected_vat: Option<Decimal>,
    pub saldo_ertrag: Option<Decimal>,
}

// --- Swiss Grouped Report Types ---

#[derive(Debug, Serialize, Clone)]
pub struct SwissBalanceSheet {
    pub as_of: String,
    pub assets: Vec<GroupedSection>,
    pub liabilities: Vec<GroupedSection>,
    pub total_assets: Decimal,
    pub total_liabilities: Decimal,
}

#[derive(Debug, Serialize, Clone)]
pub struct SwissIncomeStatement {
    pub from_date: String,
    pub to_date: String,
    pub sections: Vec<GroupedSection>,
    pub subtotals: SwissIncomeSubtotals,
}

#[derive(Debug, Serialize, Clone)]
pub struct SwissIncomeSubtotals {
    pub operating_revenue: Decimal,
    pub gross_profit_material: Decimal,
    pub gross_profit_personnel: Decimal,
    pub ebitda: Decimal,
    pub ebit: Decimal,
    pub ebt: Decimal,
    pub net_result: Decimal,
}

/// Full data bundle for annual report PDF generation.
#[derive(Debug, Serialize, Clone)]
pub struct AnnualReportData {
    pub company_name: String,
    pub company_city: String,
    pub jurisdiction: String,
    pub legal_entity_type: String,
    pub fiscal_year_name: String,
    pub fiscal_year_end: String,
    pub fiscal_year_start: String,
    pub balance_sheet_current: SwissBalanceSheet,
    pub balance_sheet_prior: Option<SwissBalanceSheet>,
    pub income_statement_current: SwissIncomeStatement,
    pub income_statement_prior: Option<SwissIncomeStatement>,
    pub shareholders: Vec<ShareholderData>,
    pub notes: std::collections::HashMap<String, serde_json::Value>,
    pub ordered_notes: Vec<NoteEntry>,
    pub fx_rates: Vec<FxRateData>,
    pub prior_retained_earnings: Decimal,
    pub current_net_result: Decimal,
    pub audit_optout: bool,
}

/// A single note entry for data-driven PDF rendering.
#[derive(Debug, Serialize, Clone)]
pub struct NoteEntry {
    pub section_key: String,
    pub label: String,
    pub section_type: String,
    pub content: serde_json::Value,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct ShareholderData {
    pub name: String,
    pub city: String,
    pub role: String,
    pub signing_rights: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct FxRateData {
    pub currency_from: String,
    pub currency_to: String,
    pub rate: Decimal,
    pub valid_date: String,
}

/// Request to update proposal allocation amounts.
#[derive(Debug, Deserialize)]
pub struct ProposalAllocation {
    pub reserve_legal: Decimal,
    pub retained_earnings: Decimal,
    pub dividend: Decimal,
    pub carry_forward: Decimal,
}
