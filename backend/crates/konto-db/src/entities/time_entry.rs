use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "time_entries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_id: Option<String>,
    pub contact_id: Option<String>,
    pub user_id: Option<String>,
    pub activity_type_id: Option<String>,
    pub date: chrono::NaiveDate,
    pub estimated_minutes: Option<i32>,
    pub actual_minutes: i32,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))", nullable)]
    pub flat_amount: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub travel_minutes: Option<i32>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub travel_flat_rate: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub travel_distance: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Decimal(Some((15, 4)))", nullable)]
    pub quantity: Option<rust_decimal::Decimal>,
    pub task_id: Option<String>,
    pub timesheet_id: Option<String>,
    pub status: String,
    pub billed: bool,
    pub billable: bool,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub bexio_id: Option<i32>,
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
        belongs_to = "super::contact::Entity",
        from = "Column::ContactId",
        to = "super::contact::Column::Id"
    )]
    Contact,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
