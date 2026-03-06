use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "invoice_lines")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub invoice_id: String,
    pub position: i32,
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
    #[sea_orm(column_type = "Decimal(Some((5, 2)))", nullable)]
    pub discount_percent: Option<rust_decimal::Decimal>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::invoice::Entity",
        from = "Column::InvoiceId",
        to = "super::invoice::Column::Id"
    )]
    Invoice,
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

impl Related<super::invoice::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invoice.def()
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
