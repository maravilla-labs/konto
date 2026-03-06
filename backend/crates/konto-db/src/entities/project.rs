use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub number: Option<String>,
    pub name: String,
    pub contact_id: Option<String>,
    pub contact_person_name: Option<String>,
    pub language: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub project_type: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub budget_hours: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))", nullable)]
    pub budget_amount: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub hourly_rate: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub soft_budget_hours: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", nullable)]
    pub hard_budget_hours: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))", nullable)]
    pub soft_budget_amount: Option<rust_decimal::Decimal>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))", nullable)]
    pub hard_budget_amount: Option<rust_decimal::Decimal>,
    pub contact_person_id: Option<String>,
    pub invoicing_method: String,
    pub currency: String,
    pub rounding_method: Option<String>,
    pub rounding_factor_minutes: Option<i32>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))", nullable)]
    pub flat_rate_total: Option<rust_decimal::Decimal>,
    pub owner_id: Option<String>,
    pub sub_status_id: Option<String>,
    pub bexio_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::contact::Entity",
        from = "Column::ContactId",
        to = "super::contact::Column::Id"
    )]
    Contact,
    #[sea_orm(has_many = "super::time_entry::Entity")]
    TimeEntries,
    #[sea_orm(has_many = "super::project_member::Entity")]
    ProjectMembers,
}

impl Related<super::contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl Related<super::time_entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimeEntries.def()
    }
}

impl Related<super::project_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProjectMembers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
