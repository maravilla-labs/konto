use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "payroll_settings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub ahv_iv_eo_rate_employee: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub ahv_iv_eo_rate_employer: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub alv_rate_employee: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub alv_rate_employer: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub alv_salary_cap: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub bvg_coordination_deduction: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub bvg_entry_threshold: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub bvg_min_insured_salary: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub bvg_max_insured_salary: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))", column_name = "bvg_rate2534")]
    pub bvg_rate_25_34: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))", column_name = "bvg_rate3544")]
    pub bvg_rate_35_44: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))", column_name = "bvg_rate4554")]
    pub bvg_rate_45_54: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))", column_name = "bvg_rate5565")]
    pub bvg_rate_55_65: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub bvg_risk_rate: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub bvg_employer_share_pct: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub nbu_rate_employee: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub bu_rate_employer: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub ktg_rate_employee: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub ktg_rate_employer: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub fak_rate_employer: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub uvg_max_salary: rust_decimal::Decimal,
    pub payment_bank_account_id: Option<String>,
    pub company_clearing_number: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
