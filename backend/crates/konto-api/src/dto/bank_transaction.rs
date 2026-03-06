use konto_db::entities::bank_transaction;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BankTransactionResponse {
    pub id: String,
    pub bank_account_id: String,
    pub transaction_date: String,
    pub value_date: String,
    #[schema(value_type = String)]
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub counterparty_name: Option<String>,
    pub counterparty_iban: Option<String>,
    pub reference: Option<String>,
    pub bank_reference: Option<String>,
    pub status: String,
    pub matched_invoice_id: Option<String>,
    pub matched_expense_id: Option<String>,
    pub matched_journal_entry_id: Option<String>,
    pub import_batch_id: Option<String>,
}

impl From<bank_transaction::Model> for BankTransactionResponse {
    fn from(m: bank_transaction::Model) -> Self {
        Self {
            id: m.id,
            bank_account_id: m.bank_account_id,
            transaction_date: m.transaction_date.to_string(),
            value_date: m.value_date.to_string(),
            amount: m.amount,
            currency: m.currency_id.unwrap_or_else(|| "CHF".into()),
            description: m.description,
            counterparty_name: m.counterparty_name,
            counterparty_iban: m.counterparty_iban,
            reference: m.reference,
            bank_reference: m.bank_reference,
            status: m.status,
            matched_invoice_id: m.matched_invoice_id,
            matched_expense_id: m.matched_expense_id,
            matched_journal_entry_id: m.matched_journal_entry_id,
            import_batch_id: m.import_batch_id,
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct BankTransactionListParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub bank_account_id: Option<String>,
    pub status: Option<String>,
    pub format: Option<String>,
}

impl BankTransactionListParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1)
    }
    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(25)
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ManualMatchRequest {
    pub target_type: String,
    pub target_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateJournalFromTxRequest {
    pub debit_account_id: String,
    pub credit_account_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AutoMatchResponse {
    pub matched_count: u64,
    pub unmatched_count: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ImportResponse {
    pub imported_count: u64,
    pub batch_id: String,
}
