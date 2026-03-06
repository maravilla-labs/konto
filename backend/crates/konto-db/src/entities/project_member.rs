use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "project_members")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_id: String,
    pub user_id: String,
    pub rate_function_id: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub role_label: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub budget_hours: Option<rust_decimal::Decimal>,
    pub joined_at: chrono::NaiveDateTime,
    pub left_at: Option<chrono::NaiveDateTime>,
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
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::rate_function::Entity",
        from = "Column::RateFunctionId",
        to = "super::rate_function::Column::Id"
    )]
    RateFunction,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::rate_function::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RateFunction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
