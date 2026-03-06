use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, ToSchema)]
pub struct ExchangeRateResponse {
    pub id: String,
    pub from_currency_id: String,
    pub to_currency_id: String,
    #[schema(value_type = String)]
    pub rate: Decimal,
    pub valid_date: String,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateExchangeRateRequest {
    pub from_currency_id: String,
    pub to_currency_id: String,
    #[schema(value_type = String)]
    pub rate: Decimal,
    pub valid_date: String,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateExchangeRateRequest {
    #[schema(value_type = Option<String>)]
    pub rate: Option<Decimal>,
    pub valid_date: Option<String>,
    pub source: Option<Option<String>>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct LatestRateQuery {
    pub from_currency_id: String,
    pub to_currency_id: String,
}

impl From<konto_db::entities::exchange_rate::Model> for ExchangeRateResponse {
    fn from(m: konto_db::entities::exchange_rate::Model) -> Self {
        Self {
            id: m.id,
            from_currency_id: m.from_currency_id,
            to_currency_id: m.to_currency_id,
            rate: m.rate,
            valid_date: m.valid_date.to_string(),
            source: m.source,
        }
    }
}
