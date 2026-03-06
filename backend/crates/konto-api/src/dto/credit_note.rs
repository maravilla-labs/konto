use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CreditNoteResponse {
    pub id: String,
    pub credit_note_number: Option<String>,
    pub invoice_id: Option<String>,
    pub contact_id: String,
    pub status: String,
    pub issue_date: String,
    pub currency_id: Option<String>,
    #[schema(value_type = String)]
    pub subtotal: Decimal,
    #[schema(value_type = String)]
    pub vat_amount: Decimal,
    #[schema(value_type = String)]
    pub total: Decimal,
    pub notes: Option<String>,
    pub journal_entry_id: Option<String>,
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CreditNoteLineResponse {
    pub id: String,
    pub credit_note_id: String,
    pub sort_order: i32,
    pub description: String,
    #[schema(value_type = String)]
    pub quantity: Decimal,
    #[schema(value_type = String)]
    pub unit_price: Decimal,
    pub vat_rate_id: Option<String>,
    #[schema(value_type = String)]
    pub vat_amount: Decimal,
    #[schema(value_type = String)]
    pub line_total: Decimal,
    pub account_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreditNoteDetailResponse {
    #[serde(flatten)]
    pub credit_note: CreditNoteResponse,
    pub lines: Vec<CreditNoteLineResponse>,
    pub contact_name: Option<String>,
    pub invoice_number: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCreditNoteRequest {
    pub contact_id: String,
    pub invoice_id: Option<String>,
    pub issue_date: String,
    pub currency_id: Option<String>,
    pub notes: Option<String>,
    pub lines: Vec<CreateCreditNoteLineRequest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCreditNoteLineRequest {
    pub description: String,
    #[schema(value_type = String)]
    pub quantity: Decimal,
    #[schema(value_type = String)]
    pub unit_price: Decimal,
    pub vat_rate_id: Option<String>,
    pub account_id: String,
}

pub type UpdateCreditNoteRequest = CreateCreditNoteRequest;

#[derive(Debug, Deserialize, IntoParams)]
pub struct CreditNoteListParams {
    #[param(default = 1, minimum = 1)]
    pub page: Option<u64>,
    #[param(default = 50, minimum = 1, maximum = 200)]
    pub per_page: Option<u64>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub format: Option<String>,
}

impl CreditNoteListParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(50).clamp(1, 200)
    }
}

impl From<konto_db::entities::credit_note::Model> for CreditNoteResponse {
    fn from(m: konto_db::entities::credit_note::Model) -> Self {
        Self {
            id: m.id,
            credit_note_number: m.credit_note_number,
            invoice_id: m.invoice_id,
            contact_id: m.contact_id,
            status: m.status,
            issue_date: m.issue_date.to_string(),
            currency_id: m.currency_id,
            subtotal: m.subtotal,
            vat_amount: m.vat_amount,
            total: m.total,
            notes: m.notes,
            journal_entry_id: m.journal_entry_id,
            created_by: m.created_by,
        }
    }
}

impl From<konto_db::entities::credit_note_line::Model> for CreditNoteLineResponse {
    fn from(m: konto_db::entities::credit_note_line::Model) -> Self {
        Self {
            id: m.id,
            credit_note_id: m.credit_note_id,
            sort_order: m.sort_order,
            description: m.description,
            quantity: m.quantity,
            unit_price: m.unit_price,
            vat_rate_id: m.vat_rate_id,
            vat_amount: m.vat_amount,
            line_total: m.line_total,
            account_id: m.account_id,
        }
    }
}
