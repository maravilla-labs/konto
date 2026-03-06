use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dunning_levels")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub level: i32,
    pub days_after_due: i32,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub fee_amount: rust_decimal::Decimal,
    #[sea_orm(column_type = "Text")]
    pub subject_template: String,
    #[sea_orm(column_type = "Text")]
    pub body_template: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::dunning_entry::Entity")]
    Entries,
}

impl Related<super::dunning_entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Entries.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
