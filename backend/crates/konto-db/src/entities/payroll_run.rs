use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "payroll_runs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub period_month: i32,
    pub period_year: i32,
    pub status: String,
    pub run_date: chrono::NaiveDate,
    pub approved_by: Option<String>,
    pub approved_at: Option<chrono::NaiveDateTime>,
    pub paid_at: Option<chrono::NaiveDateTime>,
    pub journal_entry_id: Option<String>,
    pub payment_file_generated: bool,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub total_gross: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub total_net: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub total_employer_cost: rust_decimal::Decimal,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::payroll_run_line::Entity")]
    Lines,
    #[sea_orm(
        belongs_to = "super::journal_entry::Entity",
        from = "Column::JournalEntryId",
        to = "super::journal_entry::Column::Id"
    )]
    JournalEntry,
}

impl Related<super::payroll_run_line::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lines.def()
    }
}

impl Related<super::journal_entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JournalEntry.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
