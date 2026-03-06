use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct InvoiceResponse {
    pub id: String,
    pub invoice_number: Option<String>,
    pub contact_id: String,
    pub project_id: Option<String>,
    pub status: String,
    pub issue_date: String,
    pub due_date: String,
    pub language: Option<String>,
    pub currency_id: Option<String>,
    #[schema(value_type = String)]
    pub subtotal: Decimal,
    #[schema(value_type = String)]
    pub vat_amount: Decimal,
    #[schema(value_type = String)]
    pub total: Decimal,
    pub notes: Option<String>,
    pub payment_terms: Option<String>,
    pub journal_entry_id: Option<String>,
    pub payment_journal_entry_id: Option<String>,
    pub created_by: Option<String>,
    pub header_text: Option<String>,
    pub footer_text: Option<String>,
    pub contact_person_id: Option<String>,
    pub bank_account_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct InvoiceLineResponse {
    pub id: String,
    pub invoice_id: String,
    pub position: i32,
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
    #[schema(value_type = Option<String>)]
    pub discount_percent: Option<Decimal>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InvoiceDetailResponse {
    #[serde(flatten)]
    pub invoice: InvoiceResponse,
    pub lines: Vec<InvoiceLineResponse>,
    pub contact_name: Option<String>,
    pub project_name: Option<String>,
    pub contact_person_name: Option<String>,
    #[schema(value_type = String)]
    pub amount_paid: Decimal,
    #[schema(value_type = String)]
    pub amount_remaining: Decimal,
    pub payments: Vec<super::invoice_payment::InvoicePaymentResponse>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateInvoiceRequest {
    pub contact_id: String,
    pub project_id: Option<String>,
    pub issue_date: String,
    pub due_date: String,
    pub language: Option<String>,
    pub currency_id: Option<String>,
    pub notes: Option<String>,
    pub payment_terms: Option<String>,
    pub header_text: Option<String>,
    pub footer_text: Option<String>,
    pub contact_person_id: Option<String>,
    pub bank_account_id: Option<String>,
    pub lines: Vec<CreateInvoiceLineRequest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateInvoiceLineRequest {
    pub description: String,
    #[schema(value_type = String)]
    pub quantity: Decimal,
    #[schema(value_type = String)]
    pub unit_price: Decimal,
    pub vat_rate_id: Option<String>,
    pub account_id: Option<String>,
    #[schema(value_type = Option<String>)]
    pub discount_percent: Option<Decimal>,
}

pub type UpdateInvoiceRequest = CreateInvoiceRequest;

#[derive(Debug, Deserialize, IntoParams)]
pub struct InvoiceListParams {
    #[param(default = 1, minimum = 1)]
    pub page: Option<u64>,
    #[param(default = 50, minimum = 1, maximum = 200)]
    pub per_page: Option<u64>,
    pub status: Option<String>,
    pub contact_id: Option<String>,
    pub project_id: Option<String>,
    pub search: Option<String>,
    pub format: Option<String>,
}

impl InvoiceListParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(50).clamp(1, 200)
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PayInvoiceRequest {
    pub payment_date: String,
    pub payment_account_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateInvoiceFromTimeEntriesRequest {
    pub time_entry_ids: Vec<String>,
    pub contact_id: String,
    pub project_id: Option<String>,
    /// Fallback hourly rate. Per-entry rates from activity types take priority.
    #[schema(value_type = Option<String>)]
    pub hourly_rate: Option<Decimal>,
    pub account_id: String,
    pub language: Option<String>,
}

impl From<konto_db::entities::invoice::Model> for InvoiceResponse {
    fn from(m: konto_db::entities::invoice::Model) -> Self {
        Self {
            id: m.id,
            invoice_number: m.invoice_number,
            contact_id: m.contact_id,
            project_id: m.project_id,
            status: m.status,
            issue_date: m.issue_date.to_string(),
            due_date: m.due_date.to_string(),
            language: m.language,
            currency_id: m.currency_id,
            subtotal: m.subtotal,
            vat_amount: m.vat_amount,
            total: m.total,
            notes: m.notes,
            payment_terms: m.payment_terms,
            journal_entry_id: m.journal_entry_id,
            payment_journal_entry_id: m.payment_journal_entry_id,
            created_by: m.created_by,
            header_text: m.header_text,
            footer_text: m.footer_text,
            contact_person_id: m.contact_person_id,
            bank_account_id: m.bank_account_id,
        }
    }
}

impl From<konto_db::entities::invoice_line::Model> for InvoiceLineResponse {
    fn from(m: konto_db::entities::invoice_line::Model) -> Self {
        Self {
            id: m.id,
            invoice_id: m.invoice_id,
            position: m.position,
            description: m.description,
            quantity: m.quantity,
            unit_price: m.unit_price,
            vat_rate_id: m.vat_rate_id,
            vat_amount: m.vat_amount,
            line_total: m.line_total,
            account_id: m.account_id,
            discount_percent: m.discount_percent,
        }
    }
}
