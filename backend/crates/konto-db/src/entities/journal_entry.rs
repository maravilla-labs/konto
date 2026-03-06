use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "journal_entries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub date: chrono::NaiveDate,
    pub reference: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub status: String,
    pub currency_id: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))", nullable)]
    pub exchange_rate: Option<rust_decimal::Decimal>,
    pub created_by: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::journal_line::Entity")]
    Lines,
}

impl Related<super::journal_line::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lines.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
