use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams)]
pub struct MonthsParams {
    pub months: Option<u32>,
    pub format: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct LimitParams {
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MonthlyAmountResponse {
    pub month: String,
    #[schema(value_type = String)]
    pub amount: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InvoiceAgingBucketResponse {
    pub status: String,
    pub count: i64,
    #[schema(value_type = String)]
    pub total: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OutstandingContactResponse {
    pub contact_id: String,
    pub contact_name: String,
    #[schema(value_type = String)]
    pub outstanding_amount: Decimal,
    pub invoice_count: i64,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct OverviewParams {
    pub year: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OverviewMonthResponse {
    pub month: String,
    #[schema(value_type = String)]
    pub income: Decimal,
    #[schema(value_type = String)]
    pub expenses: Decimal,
    #[schema(value_type = String)]
    pub cumulative_income: Decimal,
    #[schema(value_type = String)]
    pub cumulative_expenses: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OverviewResponse {
    pub year: i32,
    pub months: Vec<OverviewMonthResponse>,
    #[schema(value_type = String)]
    pub total_income: Decimal,
    #[schema(value_type = String)]
    pub total_expenses: Decimal,
    #[schema(value_type = String)]
    pub difference: Decimal,
    pub available_years: Vec<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CashFlowItemResponse {
    pub description: String,
    #[schema(value_type = String)]
    pub amount: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CashFlowSectionResponse {
    pub label: String,
    #[schema(value_type = String)]
    pub inflows: Decimal,
    #[schema(value_type = String)]
    pub outflows: Decimal,
    #[schema(value_type = String)]
    pub net: Decimal,
    pub items: Vec<CashFlowItemResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CashFlowReportResponse {
    pub sections: Vec<CashFlowSectionResponse>,
    #[schema(value_type = String)]
    pub net_change: Decimal,
    #[schema(value_type = String)]
    pub opening_balance: Decimal,
    #[schema(value_type = String)]
    pub closing_balance: Decimal,
    #[schema(value_type = String)]
    pub reconciliation_difference: Decimal,
    pub from_date: String,
    pub to_date: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AgingBucketResponse {
    pub bucket: String,
    pub count: i64,
    #[schema(value_type = String)]
    pub total: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MonthlyCashFlowResponse {
    pub month: String,
    #[schema(value_type = String)]
    pub inflows: Decimal,
    #[schema(value_type = String)]
    pub outflows: Decimal,
    #[schema(value_type = String)]
    pub net: Decimal,
    #[schema(value_type = String)]
    pub cumulative_balance: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CashFlowMonthlyReportResponse {
    pub months: Vec<MonthlyCashFlowResponse>,
    #[schema(value_type = String)]
    pub initial_balance: Decimal,
    #[schema(value_type = String)]
    pub ending_balance: Decimal,
    #[schema(value_type = String)]
    pub total_inflows: Decimal,
    #[schema(value_type = String)]
    pub total_outflows: Decimal,
    #[schema(value_type = String)]
    pub net_variation: Decimal,
}
