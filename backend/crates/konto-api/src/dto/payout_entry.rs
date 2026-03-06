use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PayoutEntryResponse {
    pub id: String,
    pub payroll_run_id: String,
    pub employee_id: String,
    pub amount: f64,
    pub iban: String,
    pub bic: Option<String>,
    pub recipient_name: String,
    pub recipient_street: String,
    pub recipient_postal_code: String,
    pub recipient_city: String,
    pub recipient_country: String,
    pub status: String,
    pub paid_at: Option<String>,
    pub payment_reference: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GeneratePayoutsRequest {}

impl From<konto_db::entities::payout_entry::Model> for PayoutEntryResponse {
    fn from(m: konto_db::entities::payout_entry::Model) -> Self {
        use rust_decimal::prelude::ToPrimitive;
        Self {
            id: m.id,
            payroll_run_id: m.payroll_run_id,
            employee_id: m.employee_id,
            amount: m.amount.to_f64().unwrap_or(0.0),
            iban: m.iban,
            bic: m.bic,
            recipient_name: m.recipient_name,
            recipient_street: m.recipient_street,
            recipient_postal_code: m.recipient_postal_code,
            recipient_city: m.recipient_city,
            recipient_country: m.recipient_country,
            status: m.status,
            paid_at: m.paid_at.map(|d| d.to_string()),
            payment_reference: m.payment_reference,
            created_at: m.created_at.to_string(),
            updated_at: m.updated_at.to_string(),
        }
    }
}
