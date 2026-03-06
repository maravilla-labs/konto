use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProjectMemberResponse {
    pub id: String,
    pub project_id: String,
    pub user_id: String,
    pub user_name: Option<String>,
    pub rate_function_id: Option<String>,
    pub rate_function_name: Option<String>,
    #[schema(value_type = Option<String>)]
    pub hourly_rate: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub resolved_rate: Option<rust_decimal::Decimal>,
    pub role_label: Option<String>,
    #[schema(value_type = Option<String>)]
    pub budget_hours: Option<rust_decimal::Decimal>,
    pub joined_at: String,
    pub left_at: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectMemberRequest {
    pub user_id: String,
    pub rate_function_id: Option<String>,
    #[schema(value_type = Option<String>)]
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub role_label: Option<String>,
    #[schema(value_type = Option<String>)]
    pub budget_hours: Option<rust_decimal::Decimal>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectMemberRequest {
    pub rate_function_id: Option<Option<String>>,
    #[schema(value_type = Option<Option<String>>)]
    pub hourly_rate: Option<Option<rust_decimal::Decimal>>,
    pub role_label: Option<Option<String>>,
    #[schema(value_type = Option<Option<String>>)]
    pub budget_hours: Option<Option<rust_decimal::Decimal>>,
}
