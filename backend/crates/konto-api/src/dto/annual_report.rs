use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// --- Shareholder DTOs ---

#[derive(Debug, Serialize, ToSchema)]
pub struct ShareholderResponse {
    pub id: String,
    pub name: String,
    pub city: String,
    pub role: String,
    pub signing_rights: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateShareholderRequest {
    pub name: String,
    pub city: String,
    pub role: String,
    pub signing_rights: Option<String>,
    #[serde(default)]
    pub sort_order: i32,
}

pub type UpdateShareholderRequest = CreateShareholderRequest;

impl From<konto_db::entities::shareholder::Model> for ShareholderResponse {
    fn from(m: konto_db::entities::shareholder::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            city: m.city,
            role: m.role,
            signing_rights: m.signing_rights,
            sort_order: m.sort_order,
        }
    }
}

// --- Annual Report Note DTOs ---

#[derive(Debug, Serialize, ToSchema)]
pub struct AnnualReportNoteResponse {
    pub id: String,
    pub fiscal_year_id: String,
    pub section_key: String,
    pub content: serde_json::Value,
    pub sort_order: i32,
    pub label: String,
    pub section_type: String,
}

impl From<konto_db::entities::annual_report_note::Model> for AnnualReportNoteResponse {
    fn from(m: konto_db::entities::annual_report_note::Model) -> Self {
        let content = serde_json::from_str(&m.content_json)
            .unwrap_or(serde_json::Value::Object(Default::default()));
        Self {
            id: m.id,
            fiscal_year_id: m.fiscal_year_id,
            section_key: m.section_key,
            content,
            sort_order: m.sort_order,
            label: m.label,
            section_type: m.section_type,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateNoteRequest {
    pub content: serde_json::Value,
    pub label: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNoteRequest {
    pub label: String,
    pub sort_order: Option<i32>,
}

// --- Annual Report DTOs ---

#[derive(Debug, Serialize, ToSchema)]
pub struct AnnualReportResponse {
    pub id: String,
    pub fiscal_year_id: String,
    pub status: String,
    pub generated_at: Option<String>,
    pub generated_by: Option<String>,
    pub pdf_path: Option<String>,
}

impl From<konto_db::entities::annual_report::Model> for AnnualReportResponse {
    fn from(m: konto_db::entities::annual_report::Model) -> Self {
        Self {
            id: m.id,
            fiscal_year_id: m.fiscal_year_id,
            status: m.status,
            generated_at: m.generated_at.map(|d| d.to_string()),
            generated_by: m.generated_by,
            pdf_path: m.pdf_path,
        }
    }
}

// --- Swiss Report Query Params ---

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct BalanceSheetParams {
    pub as_of: String,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct IncomeStatementParams {
    pub from_date: String,
    pub to_date: String,
}
