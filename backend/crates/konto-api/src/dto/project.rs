use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectResponse {
    pub id: String,
    pub number: Option<String>,
    pub name: String,
    pub contact_id: Option<String>,
    pub language: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: String,
    pub description: Option<String>,
    pub project_type: Option<String>,
    #[schema(value_type = Option<String>)]
    pub budget_hours: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub budget_amount: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub hourly_rate: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub soft_budget_hours: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub hard_budget_hours: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub soft_budget_amount: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub hard_budget_amount: Option<rust_decimal::Decimal>,
    pub contact_person_id: Option<String>,
    pub invoicing_method: String,
    pub currency: String,
    pub rounding_method: Option<String>,
    pub rounding_factor_minutes: Option<i32>,
    #[schema(value_type = Option<String>)]
    pub flat_rate_total: Option<rust_decimal::Decimal>,
    pub owner_id: Option<String>,
    pub sub_status_id: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectSummaryResponse {
    #[serde(flatten)]
    pub project: ProjectResponse,
    pub contact_name: Option<String>,
    #[schema(value_type = String)]
    pub total_hours: rust_decimal::Decimal,
    #[schema(value_type = String)]
    pub billable_hours: rust_decimal::Decimal,
    #[schema(value_type = Option<String>)]
    pub budget_hours_remaining: Option<rust_decimal::Decimal>,
    #[schema(value_type = String)]
    pub total_invoiced: rust_decimal::Decimal,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectRequest {
    pub name: String,
    pub number: Option<String>,
    pub contact_id: Option<String>,
    pub language: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub description: Option<String>,
    #[schema(value_type = Option<String>)]
    pub budget_hours: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub budget_amount: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub hourly_rate: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub soft_budget_hours: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub hard_budget_hours: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub soft_budget_amount: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub hard_budget_amount: Option<rust_decimal::Decimal>,
    pub contact_person_id: Option<String>,
    pub owner_id: Option<String>,
    pub invoicing_method: Option<String>,
    pub currency: Option<String>,
    pub rounding_method: Option<String>,
    pub rounding_factor_minutes: Option<i32>,
    #[schema(value_type = Option<String>)]
    pub flat_rate_total: Option<rust_decimal::Decimal>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub status: Option<String>,
    pub contact_id: Option<Option<String>>,
    pub number: Option<Option<String>>,
    pub start_date: Option<Option<String>>,
    pub end_date: Option<Option<String>>,
    pub language: Option<Option<String>>,
    pub description: Option<Option<String>>,
    #[schema(value_type = Option<Option<String>>)]
    pub budget_hours: Option<Option<rust_decimal::Decimal>>,
    #[schema(value_type = Option<Option<String>>)]
    pub budget_amount: Option<Option<rust_decimal::Decimal>>,
    #[schema(value_type = Option<Option<String>>)]
    pub hourly_rate: Option<Option<rust_decimal::Decimal>>,
    #[schema(value_type = Option<Option<String>>)]
    pub soft_budget_hours: Option<Option<rust_decimal::Decimal>>,
    #[schema(value_type = Option<Option<String>>)]
    pub hard_budget_hours: Option<Option<rust_decimal::Decimal>>,
    #[schema(value_type = Option<Option<String>>)]
    pub soft_budget_amount: Option<Option<rust_decimal::Decimal>>,
    #[schema(value_type = Option<Option<String>>)]
    pub hard_budget_amount: Option<Option<rust_decimal::Decimal>>,
    pub contact_person_id: Option<Option<String>>,
    pub invoicing_method: Option<String>,
    pub currency: Option<String>,
    pub rounding_method: Option<Option<String>>,
    pub rounding_factor_minutes: Option<Option<i32>>,
    #[schema(value_type = Option<Option<String>>)]
    pub flat_rate_total: Option<Option<rust_decimal::Decimal>>,
    pub owner_id: Option<Option<String>>,
    pub sub_status_id: Option<Option<String>>,
}

impl From<konto_db::entities::project::Model> for ProjectResponse {
    fn from(m: konto_db::entities::project::Model) -> Self {
        Self {
            id: m.id,
            number: m.number,
            name: m.name,
            contact_id: m.contact_id,
            language: m.language,
            start_date: m.start_date.map(|d| d.to_string()),
            end_date: m.end_date.map(|d| d.to_string()),
            status: m.status,
            description: m.description,
            project_type: m.project_type,
            budget_hours: m.budget_hours,
            budget_amount: m.budget_amount,
            hourly_rate: m.hourly_rate,
            soft_budget_hours: m.soft_budget_hours,
            hard_budget_hours: m.hard_budget_hours,
            soft_budget_amount: m.soft_budget_amount,
            hard_budget_amount: m.hard_budget_amount,
            contact_person_id: m.contact_person_id,
            invoicing_method: m.invoicing_method,
            currency: m.currency,
            rounding_method: m.rounding_method,
            rounding_factor_minutes: m.rounding_factor_minutes,
            flat_rate_total: m.flat_rate_total,
            owner_id: m.owner_id,
            sub_status_id: m.sub_status_id,
        }
    }
}
