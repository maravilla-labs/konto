use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct RateFunctionResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[schema(value_type = String)]
    pub hourly_rate: rust_decimal::Decimal,
    pub is_active: bool,
    pub sort_order: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRateFunctionRequest {
    pub name: String,
    pub description: Option<String>,
    #[schema(value_type = String)]
    pub hourly_rate: rust_decimal::Decimal,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRateFunctionRequest {
    pub name: String,
    pub description: Option<String>,
    #[schema(value_type = String)]
    pub hourly_rate: rust_decimal::Decimal,
    pub is_active: bool,
    pub sort_order: i32,
}

impl From<konto_db::entities::rate_function::Model> for RateFunctionResponse {
    fn from(m: konto_db::entities::rate_function::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            description: m.description,
            hourly_rate: m.hourly_rate,
            is_active: m.is_active,
            sort_order: m.sort_order,
        }
    }
}
