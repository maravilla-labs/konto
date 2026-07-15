use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "currency_exchanges")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub from_bank_account_id: String,
    pub to_bank_account_id: String,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub from_amount: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub to_amount: rust_decimal::Decimal,
    pub date: chrono::NaiveDate,
    pub journal_entry_id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bank_account::Entity",
        from = "Column::FromBankAccountId",
        to = "super::bank_account::Column::Id"
    )]
    FromBankAccount,
    #[sea_orm(
        belongs_to = "super::bank_account::Entity",
        from = "Column::ToBankAccountId",
        to = "super::bank_account::Column::Id"
    )]
    ToBankAccount,
    #[sea_orm(
        belongs_to = "super::journal_entry::Entity",
        from = "Column::JournalEntryId",
        to = "super::journal_entry::Column::Id"
    )]
    JournalEntry,
}

impl Related<super::journal_entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JournalEntry.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
