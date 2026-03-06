use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct JournalEntryResponse {
    pub id: String,
    pub date: String,
    pub reference: Option<String>,
    pub description: String,
    pub status: String,
    pub currency_id: Option<String>,
    #[schema(value_type = Option<String>)]
    pub exchange_rate: Option<Decimal>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JournalLineResponse {
    pub id: String,
    pub account_id: String,
    #[schema(value_type = String)]
    pub debit_amount: Decimal,
    #[schema(value_type = String)]
    pub credit_amount: Decimal,
    pub description: Option<String>,
    pub vat_rate_id: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JournalDetailResponse {
    pub entry: JournalEntryResponse,
    pub lines: Vec<JournalLineResponse>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateJournalEntryRequest {
    pub date: String,
    pub description: String,
    pub reference: Option<String>,
    pub currency_id: Option<String>,
    #[schema(value_type = Option<String>)]
    pub exchange_rate: Option<Decimal>,
    pub lines: Vec<CreateJournalLineRequest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateJournalLineRequest {
    pub account_id: String,
    #[schema(value_type = String)]
    pub debit_amount: Decimal,
    #[schema(value_type = String)]
    pub credit_amount: Decimal,
    pub description: Option<String>,
    pub vat_rate_id: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct BulkPostRequest {
    pub entry_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all_drafts: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BulkPostResponse {
    pub posted: u32,
}

impl From<konto_db::entities::journal_entry::Model> for JournalEntryResponse {
    fn from(m: konto_db::entities::journal_entry::Model) -> Self {
        Self {
            id: m.id,
            date: m.date.to_string(),
            reference: m.reference,
            description: m.description,
            status: m.status,
            currency_id: m.currency_id,
            exchange_rate: m.exchange_rate,
        }
    }
}

impl From<konto_db::entities::journal_line::Model> for JournalLineResponse {
    fn from(m: konto_db::entities::journal_line::Model) -> Self {
        Self {
            id: m.id,
            account_id: m.account_id,
            debit_amount: m.debit_amount,
            credit_amount: m.credit_amount,
            description: m.description,
            vat_rate_id: m.vat_rate_id,
        }
    }
}
