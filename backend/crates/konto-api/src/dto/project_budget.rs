use rust_decimal::Decimal;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct BudgetAnalyticsResponse {
    #[schema(value_type = String)]
    pub total_hours: Decimal,
    #[schema(value_type = String)]
    pub billable_hours: Decimal,
    #[schema(value_type = String)]
    pub non_billable_hours: Decimal,
    #[schema(value_type = String)]
    pub billed_hours: Decimal,
    #[schema(value_type = String)]
    pub unbilled_hours: Decimal,
    #[schema(value_type = Option<String>)]
    pub budget_hours: Option<Decimal>,
    #[schema(value_type = String)]
    pub actual_amount: Decimal,
    #[schema(value_type = String)]
    pub invoiced_amount: Decimal,
    pub per_member: Vec<MemberBudgetRow>,
    pub per_activity: Vec<ActivityBudgetRow>,
    pub timeline: Vec<WeeklyBudgetPoint>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MemberBudgetRow {
    pub user_id: String,
    pub user_name: String,
    #[schema(value_type = Option<String>)]
    pub budget_hours: Option<Decimal>,
    #[schema(value_type = String)]
    pub actual_hours: Decimal,
    #[schema(value_type = Option<String>)]
    pub rate: Option<Decimal>,
    #[schema(value_type = String)]
    pub actual_amount: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ActivityBudgetRow {
    pub activity_type_id: String,
    pub activity_name: String,
    #[schema(value_type = Option<String>)]
    pub budget_hours: Option<Decimal>,
    #[schema(value_type = String)]
    pub actual_hours: Decimal,
    pub chargeable: bool,
    #[schema(value_type = Option<String>)]
    pub rate: Option<Decimal>,
    #[schema(value_type = String)]
    pub actual_amount: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WeeklyBudgetPoint {
    pub week_start: String,
    #[schema(value_type = String)]
    pub cumulative_budget: Decimal,
    #[schema(value_type = String)]
    pub cumulative_actual: Decimal,
}
