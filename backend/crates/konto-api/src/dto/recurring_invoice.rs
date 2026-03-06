use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct RecurringInvoiceResponse {
    pub id: String,
    pub contact_id: String,
    pub project_id: Option<String>,
    pub template_data: serde_json::Value,
    pub frequency: String,
    pub interval_days: Option<i32>,
    pub next_run_date: String,
    pub end_date: Option<String>,
    pub auto_send: bool,
    pub is_active: bool,
    pub last_generated_at: Option<String>,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRecurringInvoiceRequest {
    pub contact_id: String,
    pub project_id: Option<String>,
    pub frequency: String,
    pub interval_days: Option<i32>,
    pub next_run_date: String,
    pub end_date: Option<String>,
    pub auto_send: Option<bool>,
    pub language: Option<String>,
    pub currency_id: Option<String>,
    pub notes: Option<String>,
    pub payment_terms: Option<String>,
    pub lines: Vec<RecurringInvoiceLineRequest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RecurringInvoiceLineRequest {
    pub description: String,
    #[schema(value_type = String)]
    pub quantity: Decimal,
    #[schema(value_type = String)]
    pub unit_price: Decimal,
    pub vat_rate_id: Option<String>,
    pub account_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRecurringInvoiceRequest {
    pub contact_id: String,
    pub project_id: Option<String>,
    pub frequency: String,
    pub interval_days: Option<i32>,
    pub next_run_date: String,
    pub end_date: Option<String>,
    pub auto_send: Option<bool>,
    pub is_active: Option<bool>,
    pub language: Option<String>,
    pub currency_id: Option<String>,
    pub notes: Option<String>,
    pub payment_terms: Option<String>,
    pub lines: Vec<RecurringInvoiceLineRequest>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct RecurringInvoiceListParams {
    #[param(default = 1, minimum = 1)]
    pub page: Option<u64>,
    #[param(default = 50, minimum = 1, maximum = 200)]
    pub per_page: Option<u64>,
    pub is_active: Option<bool>,
    pub search: Option<String>,
    pub format: Option<String>,
}

impl RecurringInvoiceListParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(50).clamp(1, 200)
    }
}

impl From<konto_db::entities::recurring_invoice::Model> for RecurringInvoiceResponse {
    fn from(m: konto_db::entities::recurring_invoice::Model) -> Self {
        let template_data = serde_json::from_str(&m.template_data)
            .unwrap_or(serde_json::Value::Null);
        Self {
            id: m.id,
            contact_id: m.contact_id,
            project_id: m.project_id,
            template_data,
            frequency: m.frequency,
            interval_days: m.interval_days,
            next_run_date: m.next_run_date.to_string(),
            end_date: m.end_date.map(|d| d.to_string()),
            auto_send: m.auto_send,
            is_active: m.is_active,
            last_generated_at: m.last_generated_at.map(|d| d.to_string()),
            created_by: m.created_by,
            created_at: m.created_at.to_string(),
            updated_at: m.updated_at.to_string(),
        }
    }
}
