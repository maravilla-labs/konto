use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProjectActivityTypeResponse {
    pub id: String,
    pub project_id: String,
    pub activity_type_id: String,
    pub activity_type_name: Option<String>,
    pub unit_type: Option<String>,
    #[schema(value_type = Option<String>)]
    pub default_rate: Option<Decimal>,
    #[schema(value_type = Option<String>)]
    pub rate: Option<Decimal>,
    #[schema(value_type = Option<String>)]
    pub effective_rate: Option<Decimal>,
    #[schema(value_type = Option<String>)]
    pub budget_hours: Option<Decimal>,
    pub chargeable: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectActivityTypeRequest {
    pub activity_type_id: String,
    #[schema(value_type = Option<String>)]
    pub rate: Option<Decimal>,
    #[schema(value_type = Option<String>)]
    pub budget_hours: Option<Decimal>,
    #[serde(default = "default_true")]
    pub chargeable: bool,
}

fn default_true() -> bool { true }

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectActivityTypeRequest {
    #[schema(value_type = Option<Option<String>>)]
    pub rate: Option<Option<Decimal>>,
    #[schema(value_type = Option<Option<String>>)]
    pub budget_hours: Option<Option<Decimal>>,
    pub chargeable: Option<bool>,
}
