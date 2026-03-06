use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "payout_entries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub payroll_run_id: String,
    pub employee_id: String,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub amount: rust_decimal::Decimal,
    pub iban: String,
    pub bic: Option<String>,
    pub recipient_name: String,
    pub recipient_street: String,
    pub recipient_postal_code: String,
    pub recipient_city: String,
    pub recipient_country: String,
    pub status: String,
    pub paid_at: Option<chrono::NaiveDateTime>,
    pub payment_reference: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::payroll_run::Entity",
        from = "Column::PayrollRunId",
        to = "super::payroll_run::Column::Id"
    )]
    PayrollRun,
    #[sea_orm(
        belongs_to = "super::employee::Entity",
        from = "Column::EmployeeId",
        to = "super::employee::Column::Id"
    )]
    Employee,
}

impl Related<super::payroll_run::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PayrollRun.def()
    }
}

impl Related<super::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
