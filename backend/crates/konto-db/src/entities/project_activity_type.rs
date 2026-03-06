use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "project_activity_types")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_id: String,
    pub activity_type_id: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub rate: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub budget_hours: Option<rust_decimal::Decimal>,
    pub chargeable: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id"
    )]
    Project,
    #[sea_orm(
        belongs_to = "super::activity_type::Entity",
        from = "Column::ActivityTypeId",
        to = "super::activity_type::Column::Id"
    )]
    ActivityType,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::activity_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ActivityType.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
