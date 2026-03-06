use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contacts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub contact_type: String,
    pub category: Option<String>,
    pub industry: Option<String>,
    pub name1: String,
    pub name2: Option<String>,
    pub salutation: Option<String>,
    pub title: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub email2: Option<String>,
    pub phone: Option<String>,
    pub phone2: Option<String>,
    pub mobile: Option<String>,
    pub fax: Option<String>,
    pub website: Option<String>,
    pub vat_number: Option<String>,
    pub language: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    pub birthday: Option<chrono::NaiveDate>,
    pub employee_count: Option<i32>,
    pub trade_register_number: Option<String>,
    pub salutation_form: Option<String>,
    pub customer_number: Option<String>,
    pub bexio_id: Option<i32>,
    pub vat_mode: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::contact_person::Entity")]
    ContactPersons,
    #[sea_orm(has_many = "super::project::Entity")]
    Projects,
}

impl Related<super::contact_person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContactPersons.def()
    }
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
