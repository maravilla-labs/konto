use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SetupStatusResponse {
    pub setup_needed: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SetupCompleteRequest {
    // Admin account
    pub admin_email: String,
    pub admin_password: String,
    pub admin_full_name: String,
    pub admin_language: String,
    // Company info
    pub legal_name: String,
    pub trade_name: Option<String>,
    pub street: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub legal_entity_type: Option<String>,
    // Accounting config
    pub default_currency: Option<String>,
    pub vat_method: Option<String>,
    pub flat_rate_percentage: Option<f64>,
    pub date_format: Option<String>,
    pub fiscal_year_start_month: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SetupCompleteResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BrandingResponse {
    pub legal_name: Option<String>,
    pub trade_name: Option<String>,
    pub logo_url: Option<String>,
}
