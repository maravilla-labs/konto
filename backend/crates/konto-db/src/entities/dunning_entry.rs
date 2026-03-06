use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dunning_entries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub invoice_id: String,
    pub dunning_level_id: String,
    pub sent_at: chrono::NaiveDateTime,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub fee_amount: rust_decimal::Decimal,
    pub email_sent: bool,
    pub journal_entry_id: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    pub created_at: chrono::NaiveDateTime,
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
        belongs_to = "super::dunning_level::Entity",
        from = "Column::DunningLevelId",
        to = "super::dunning_level::Column::Id"
    )]
    DunningLevel,
}

impl Related<super::invoice::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invoice.def()
    }
}

impl Related<super::dunning_level::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DunningLevel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
