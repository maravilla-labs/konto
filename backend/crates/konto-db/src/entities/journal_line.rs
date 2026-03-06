use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "journal_lines")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub journal_entry_id: String,
    pub account_id: String,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub debit_amount: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub credit_amount: rust_decimal::Decimal,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub vat_rate_id: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))", nullable)]
    pub currency_amount: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))", nullable)]
    pub base_currency_amount: Option<rust_decimal::Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::journal_entry::Entity",
        from = "Column::JournalEntryId",
        to = "super::journal_entry::Column::Id"
    )]
    JournalEntry,
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::Id"
    )]
    Account,
}

impl Related<super::journal_entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JournalEntry.def()
    }
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
