use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "credit_note_lines")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub credit_note_id: String,
    pub sort_order: i32,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Decimal(Some((15, 4)))")]
    pub quantity: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub unit_price: rust_decimal::Decimal,
    pub vat_rate_id: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub vat_amount: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub line_total: rust_decimal::Decimal,
    pub account_id: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::credit_note::Entity",
        from = "Column::CreditNoteId",
        to = "super::credit_note::Column::Id"
    )]
    CreditNote,
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::Id"
    )]
    Account,
    #[sea_orm(
        belongs_to = "super::vat_rate::Entity",
        from = "Column::VatRateId",
        to = "super::vat_rate::Column::Id"
    )]
    VatRate,
}

impl Related<super::credit_note::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CreditNote.def()
    }
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::vat_rate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::VatRate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
