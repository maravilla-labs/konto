use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contact_relationships")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub person_contact_id: String,
    pub org_contact_id: String,
    pub role: Option<String>,
    pub position: Option<String>,
    pub department: Option<String>,
    pub is_primary: bool,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::contact::Entity",
        from = "Column::PersonContactId",
        to = "super::contact::Column::Id"
    )]
    PersonContact,
    #[sea_orm(
        belongs_to = "super::contact::Entity",
        from = "Column::OrgContactId",
        to = "super::contact::Column::Id"
    )]
    OrgContact,
}

impl ActiveModelBehavior for ActiveModel {}
