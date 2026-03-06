use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "fiscal_periods")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub fiscal_year_id: String,
    pub name: String,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub period_number: Option<i32>,
    pub status: String,
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
