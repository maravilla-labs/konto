use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct FiscalYearResponse {
    pub id: String,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FiscalPeriodResponse {
    pub id: String,
    pub fiscal_year_id: String,
    pub name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub period_number: Option<i32>,
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FiscalYearDetailResponse {
    pub fiscal_year: FiscalYearResponse,
    pub periods: Vec<FiscalPeriodResponse>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateFiscalYearRequest {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateFiscalYearRequest {
    pub name: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl From<konto_db::entities::fiscal_year::Model> for FiscalYearResponse {
    fn from(m: konto_db::entities::fiscal_year::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            start_date: m.start_date.to_string(),
            end_date: m.end_date.to_string(),
            status: m.status,
        }
    }
}

impl From<konto_db::entities::fiscal_period::Model> for FiscalPeriodResponse {
    fn from(m: konto_db::entities::fiscal_period::Model) -> Self {
        Self {
            id: m.id,
            fiscal_year_id: m.fiscal_year_id,
            name: m.name,
            start_date: m.start_date.map(|d| d.to_string()),
            end_date: m.end_date.map(|d| d.to_string()),
            period_number: m.period_number,
            status: m.status,
        }
    }
}
