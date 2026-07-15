use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrencyExchangeResponse {
    pub id: String,
    pub from_bank_account_id: String,
    pub to_bank_account_id: String,
    #[schema(value_type = String)]
    pub from_amount: Decimal,
    #[schema(value_type = String)]
    pub to_amount: Decimal,
    pub date: String,
    pub journal_entry_id: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RecordTransferRequest {
    pub from_bank_account_id: String,
    pub to_bank_account_id: String,
    #[schema(value_type = String)]
    pub from_amount: Decimal,
    /// The actual base-currency value received into the destination account.
    #[schema(value_type = String)]
    pub to_amount: Decimal,
    pub date: String,
    pub notes: Option<String>,
}

impl From<konto_db::entities::currency_exchange::Model> for CurrencyExchangeResponse {
    fn from(m: konto_db::entities::currency_exchange::Model) -> Self {
        Self {
            id: m.id,
            from_bank_account_id: m.from_bank_account_id,
            to_bank_account_id: m.to_bank_account_id,
            from_amount: m.from_amount,
            to_amount: m.to_amount,
            date: m.date.to_string(),
            journal_entry_id: m.journal_entry_id,
            notes: m.notes,
        }
    }
}
