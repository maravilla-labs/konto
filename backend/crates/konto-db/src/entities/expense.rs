use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "expenses")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub expense_number: Option<String>,
    pub contact_id: Option<String>,
    pub category_id: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub amount: rust_decimal::Decimal,
    pub currency_id: String,
    pub vat_rate_id: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub vat_amount: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub total: rust_decimal::Decimal,
    pub expense_date: chrono::NaiveDate,
    pub due_date: Option<chrono::NaiveDate>,
    pub status: String,
    pub payment_account_id: Option<String>,
    pub receipt_url: Option<String>,
    pub project_id: Option<String>,
    pub journal_entry_id: Option<String>,
    pub payment_journal_entry_id: Option<String>,
    pub created_by: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    // Report fields (added in migration 000030)
    pub expense_type: String,
    pub purpose: Option<String>,
    pub employee_id: Option<String>,
    pub period_from: Option<chrono::NaiveDate>,
    pub period_to: Option<chrono::NaiveDate>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub advances: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub total_reimbursement: rust_decimal::Decimal,
    pub approved_by: Option<String>,
    pub approved_at: Option<chrono::NaiveDateTime>,
    pub rejected_reason: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::contact::Entity",
        from = "Column::ContactId",
        to = "super::contact::Column::Id"
    )]
    Contact,
    #[sea_orm(
        belongs_to = "super::expense_category::Entity",
        from = "Column::CategoryId",
        to = "super::expense_category::Column::Id"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::currency::Entity",
        from = "Column::CurrencyId",
        to = "super::currency::Column::Id"
    )]
    Currency,
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id"
    )]
    Project,
}

impl Related<super::contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl Related<super::expense_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::currency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Currency.def()
    }
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
