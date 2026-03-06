use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "bank_transactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub bank_account_id: String,
    pub transaction_date: chrono::NaiveDate,
    pub value_date: chrono::NaiveDate,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub amount: rust_decimal::Decimal,
    pub currency_id: Option<String>,
    #[sea_orm(column_type = "Text")]
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
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bank_account::Entity",
        from = "Column::BankAccountId",
        to = "super::bank_account::Column::Id"
    )]
    BankAccount,
}

impl Related<super::bank_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BankAccount.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
