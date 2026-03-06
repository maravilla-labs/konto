use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "import_jobs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub import_type: String,
    pub file_name: String,
    #[sea_orm(column_type = "VarBinary(StringLen::None)")]
    #[serde(skip)]
    pub file_data: Vec<u8>,
    pub status: String,
    pub total_rows: Option<i32>,
    pub imported_rows: Option<i32>,
    pub error_rows: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub preview_data: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub error_log: Option<String>,
    pub created_by: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
