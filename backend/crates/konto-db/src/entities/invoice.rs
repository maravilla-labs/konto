use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "invoices")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub invoice_number: Option<String>,
    pub contact_id: String,
    pub project_id: Option<String>,
    pub status: String,
    pub issue_date: chrono::NaiveDate,
    pub due_date: chrono::NaiveDate,
    pub language: Option<String>,
    pub currency_id: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub subtotal: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub vat_amount: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub total: rust_decimal::Decimal,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub payment_terms: Option<String>,
    pub journal_entry_id: Option<String>,
    pub payment_journal_entry_id: Option<String>,
    pub created_by: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub template_id: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub content_json: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub header_text: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub footer_text: Option<String>,
    pub contact_person_id: Option<String>,
    pub bank_account_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::invoice_line::Entity")]
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
        belongs_to = "super::currency::Entity",
        from = "Column::CurrencyId",
        to = "super::currency::Column::Id"
    )]
    Currency,
}

impl Related<super::invoice_line::Entity> for Entity {
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
