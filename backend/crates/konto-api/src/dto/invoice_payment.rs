use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct InvoicePaymentResponse {
    pub id: String,
    pub invoice_id: String,
    #[schema(value_type = String)]
    pub amount: Decimal,
    pub payment_date: String,
    pub payment_method: Option<String>,
    pub reference: Option<String>,
    pub bank_transaction_id: Option<String>,
    pub journal_entry_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RecordPaymentRequest {
    #[schema(value_type = String)]
    pub amount: Decimal,
    pub payment_date: String,
    pub payment_account_id: String,
    pub payment_method: Option<String>,
    pub reference: Option<String>,
}

impl From<konto_db::entities::invoice_payment::Model> for InvoicePaymentResponse {
    fn from(m: konto_db::entities::invoice_payment::Model) -> Self {
        Self {
            id: m.id,
            invoice_id: m.invoice_id,
            amount: m.amount,
            payment_date: m.payment_date.to_string(),
            payment_method: m.payment_method,
            reference: m.reference,
            bank_transaction_id: m.bank_transaction_id,
            journal_entry_id: m.journal_entry_id,
            created_at: m.created_at.to_string(),
        }
    }
}
