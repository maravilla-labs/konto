use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "credit_notes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub credit_note_number: Option<String>,
    pub invoice_id: Option<String>,
    pub contact_id: String,
    pub status: String,
    pub issue_date: chrono::NaiveDate,
    pub currency_id: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub subtotal: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub vat_amount: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub total: rust_decimal::Decimal,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    pub journal_entry_id: Option<String>,
    pub created_by: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::credit_note_line::Entity")]
    Lines,
    #[sea_orm(
        belongs_to = "super::invoice::Entity",
        from = "Column::InvoiceId",
        to = "super::invoice::Column::Id"
    )]
    Invoice,
    #[sea_orm(
        belongs_to = "super::contact::Entity",
        from = "Column::ContactId",
        to = "super::contact::Column::Id"
    )]
    Contact,
    #[sea_orm(
        belongs_to = "super::currency::Entity",
        from = "Column::CurrencyId",
        to = "super::currency::Column::Id"
    )]
    Currency,
}

impl Related<super::credit_note_line::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lines.def()
    }
}

impl Related<super::invoice::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invoice.def()
    }
}

impl Related<super::contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl Related<super::currency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Currency.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
