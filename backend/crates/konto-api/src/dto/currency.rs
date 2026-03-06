use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CurrencyResponse {
    pub id: String,
    pub code: String,
    pub name: String,
    pub symbol: String,
    pub is_primary: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCurrencyRequest {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCurrencyRequest {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

impl From<konto_db::entities::currency::Model> for CurrencyResponse {
    fn from(m: konto_db::entities::currency::Model) -> Self {
        Self {
            id: m.id,
            code: m.code,
            name: m.name,
            symbol: m.symbol,
            is_primary: m.is_primary,
        }
    }
}
