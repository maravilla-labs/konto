use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "annual_report_notes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub fiscal_year_id: String,
    pub section_key: String,
    #[sea_orm(column_type = "Text")]
    pub content_json: String,
    pub sort_order: i32,
    pub label: String,
    pub section_type: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::fiscal_year::Entity",
        from = "Column::FiscalYearId",
        to = "super::fiscal_year::Column::Id"
    )]
    FiscalYear,
}

impl Related<super::fiscal_year::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FiscalYear.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
