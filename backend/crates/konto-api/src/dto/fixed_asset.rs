use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct FixedAssetResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub account_id: String,
    pub depreciation_account_id: String,
    pub acquisition_date: String,
    pub acquisition_cost: f64,
    pub residual_value: f64,
    pub useful_life_years: i32,
    pub depreciation_method: String,
    pub declining_rate: Option<f64>,
    pub status: String,
    pub disposed_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateFixedAssetRequest {
    pub name: String,
    pub description: Option<String>,
    pub account_id: String,
    pub depreciation_account_id: String,
    pub acquisition_date: String,
    pub acquisition_cost: f64,
    pub residual_value: f64,
    pub useful_life_years: i32,
    pub depreciation_method: String,
    pub declining_rate: Option<f64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateFixedAssetRequest {
    pub name: String,
    pub description: Option<String>,
    pub account_id: String,
    pub depreciation_account_id: String,
    pub acquisition_date: String,
    pub acquisition_cost: f64,
    pub residual_value: f64,
    pub useful_life_years: i32,
    pub depreciation_method: String,
    pub declining_rate: Option<f64>,
    pub status: String,
    pub disposed_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DepreciationEntryResponse {
    pub id: String,
    pub fixed_asset_id: String,
    pub fiscal_year_id: String,
    pub journal_entry_id: String,
    pub amount: f64,
    pub accumulated: f64,
    pub book_value: f64,
    pub period_date: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RunDepreciationRequest {
    pub fiscal_year_id: String,
}

impl From<konto_db::entities::fixed_asset::Model> for FixedAssetResponse {
    fn from(m: konto_db::entities::fixed_asset::Model) -> Self {
        use rust_decimal::prelude::ToPrimitive;
        Self {
            id: m.id,
            name: m.name,
            description: m.description,
            account_id: m.account_id,
            depreciation_account_id: m.depreciation_account_id,
            acquisition_date: m.acquisition_date.to_string(),
            acquisition_cost: m.acquisition_cost.to_f64().unwrap_or(0.0),
            residual_value: m.residual_value.to_f64().unwrap_or(0.0),
            useful_life_years: m.useful_life_years,
            depreciation_method: m.depreciation_method,
            declining_rate: m.declining_rate.and_then(|d| d.to_f64()),
            status: m.status,
            disposed_date: m.disposed_date.map(|d| d.to_string()),
            created_at: m.created_at.to_string(),
            updated_at: m.updated_at.to_string(),
        }
    }
}

impl From<konto_db::entities::depreciation_entry::Model> for DepreciationEntryResponse {
    fn from(m: konto_db::entities::depreciation_entry::Model) -> Self {
        use rust_decimal::prelude::ToPrimitive;
        Self {
            id: m.id,
            fixed_asset_id: m.fixed_asset_id,
            fiscal_year_id: m.fiscal_year_id,
            journal_entry_id: m.journal_entry_id,
            amount: m.amount.to_f64().unwrap_or(0.0),
            accumulated: m.accumulated.to_f64().unwrap_or(0.0),
            book_value: m.book_value.to_f64().unwrap_or(0.0),
            period_date: m.period_date.to_string(),
            created_at: m.created_at.to_string(),
        }
    }
}
