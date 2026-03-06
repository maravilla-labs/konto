use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "annual_reports")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub fiscal_year_id: String,
    pub status: String,
    pub generated_at: Option<chrono::NaiveDateTime>,
    pub generated_by: Option<String>,
    pub pdf_path: Option<String>,
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
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::GeneratedBy",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::fiscal_year::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FiscalYear.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
