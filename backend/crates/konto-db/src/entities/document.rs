use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub doc_type: String,
    #[sea_orm(unique)]
    pub doc_number: Option<String>,
    pub title: String,
    pub status: String,
    pub contact_id: String,
    pub project_id: Option<String>,
    pub template_id: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub content_json: String,
    pub language: Option<String>,
    pub currency_id: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((19, 4)))")]
    pub subtotal: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub vat_rate: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((19, 4)))")]
    pub vat_amount: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((19, 4)))")]
    pub total: rust_decimal::Decimal,
    pub valid_until: Option<chrono::NaiveDate>,
    pub issued_at: Option<chrono::NaiveDate>,
    pub signed_at: Option<chrono::NaiveDate>,
    pub converted_from: Option<String>,
    pub created_by: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::document_line_item::Entity")]
    Lines,
    #[sea_orm(
        belongs_to = "super::contact::Entity",
        from = "Column::ContactId",
        to = "super::contact::Column::Id"
    )]
    Contact,
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id"
    )]
    Project,
    #[sea_orm(
        belongs_to = "super::document_template::Entity",
        from = "Column::TemplateId",
        to = "super::document_template::Column::Id"
    )]
    Template,
    #[sea_orm(
        belongs_to = "super::currency::Entity",
        from = "Column::CurrencyId",
        to = "super::currency::Column::Id"
    )]
    Currency,
}

impl Related<super::document_line_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lines.def()
    }
}

impl Related<super::contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::currency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Currency.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
