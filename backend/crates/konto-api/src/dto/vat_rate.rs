use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct VatRateResponse {
    pub id: String,
    pub code: String,
    pub name: String,
    pub rate: f64,
    pub vat_type: String,
    pub vat_category: String,
    pub is_active: bool,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateVatRateRequest {
    pub code: String,
    pub name: String,
    pub rate: f64,
    #[serde(default = "default_output")]
    pub vat_type: String,
    #[serde(default = "default_standard")]
    pub vat_category: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
}

fn default_standard() -> Option<String> {
    Some("standard".to_string())
}

fn default_output() -> String {
    "output".to_string()
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateVatRateRequest {
    pub code: String,
    pub name: String,
    pub rate: f64,
    pub vat_type: String,
    pub vat_category: Option<String>,
    pub is_active: bool,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
}

impl From<konto_db::entities::vat_rate::Model> for VatRateResponse {
    fn from(m: konto_db::entities::vat_rate::Model) -> Self {
        use rust_decimal::prelude::ToPrimitive;
        Self {
            id: m.id,
            code: m.code,
            name: m.name,
            rate: m.rate.to_f64().unwrap_or(0.0),
            vat_type: m.vat_type,
            vat_category: m.vat_category,
            is_active: m.is_active,
            valid_from: m.valid_from.map(|d| d.to_string()),
            valid_to: m.valid_to.map(|d| d.to_string()),
        }
    }
}
