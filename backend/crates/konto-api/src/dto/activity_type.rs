use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ActivityTypeResponse {
    pub id: String,
    pub name: String,
    pub is_active: bool,
    pub unit_type: String,
    #[schema(value_type = Option<String>)]
    pub default_rate: Option<Decimal>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateActivityTypeRequest {
    pub name: String,
    pub unit_type: Option<String>,
    #[schema(value_type = Option<String>)]
    pub default_rate: Option<Decimal>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateActivityTypeRequest {
    pub name: String,
    pub is_active: bool,
    pub unit_type: Option<String>,
    #[schema(value_type = Option<Option<String>>)]
    pub default_rate: Option<Option<Decimal>>,
}

impl From<konto_db::entities::activity_type::Model> for ActivityTypeResponse {
    fn from(m: konto_db::entities::activity_type::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            is_active: m.is_active,
            unit_type: m.unit_type,
            default_rate: m.default_rate,
        }
    }
}
