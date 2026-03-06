use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "document_templates")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub template_type: String,
    #[sea_orm(column_type = "Text")]
    pub content_json: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub header_json: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub footer_json: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub page_setup_json: Option<String>,
    pub is_default: bool,
    pub created_by: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::CreatedBy",
        to = "super::user::Column::Id"
    )]
    Creator,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Creator.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
