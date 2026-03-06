use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "payroll_run_lines")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub payroll_run_id: String,
    pub employee_id: String,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub gross_salary: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub ahv_employee: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub ahv_employer: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub alv_employee: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub alv_employer: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub bvg_employee: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub bvg_employer: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub nbu_employee: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub bu_employer: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub ktg_employee: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub ktg_employer: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub fak_employer: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub quellensteuer: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub child_allowance: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub net_salary: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub payout_amount: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub total_employer_cost: rust_decimal::Decimal,
    pub created_at: chrono::NaiveDateTime,
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
