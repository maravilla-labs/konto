use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "document_line_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub document_id: String,
    pub position: i32,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub quantity: rust_decimal::Decimal,
    pub unit: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((19, 4)))")]
    pub unit_price: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub discount_pct: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((19, 4)))")]
    pub total: rust_decimal::Decimal,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::document::Entity",
        from = "Column::DocumentId",
        to = "super::document::Column::Id"
    )]
    Document,
}

impl Related<super::document::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Document.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
