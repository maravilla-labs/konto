use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "project_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_id: String,
    pub parent_id: Option<String>,
    pub item_type: String,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub status: String,
    pub assignee_id: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
    pub due_date: Option<chrono::NaiveDate>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub estimated_hours: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub budget_hours: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))", nullable)]
    pub budget_amount: Option<rust_decimal::Decimal>,
    pub sort_order: i32,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
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
        belongs_to = "super::user::Entity",
        from = "Column::AssigneeId",
        to = "super::user::Column::Id"
    )]
    Assignee,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id"
    )]
    Parent,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
