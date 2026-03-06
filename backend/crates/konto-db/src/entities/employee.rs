use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "employees")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub number: Option<String>,
    pub user_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ahv_number: String,
    pub date_of_birth: chrono::NaiveDate,
    pub nationality: String,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub iban: String,
    pub bic: Option<String>,
    pub bank_name: Option<String>,
    pub employment_start: chrono::NaiveDate,
    pub employment_end: Option<chrono::NaiveDate>,
    pub position: Option<String>,
    pub department: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub employment_percentage: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub gross_monthly_salary: rust_decimal::Decimal,
    pub annual_salary_13th: bool,
    pub has_children: bool,
    pub number_of_children: i32,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub child_allowance_amount: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub education_allowance_amount: rust_decimal::Decimal,
    pub bvg_insured: bool,
    pub uvg_insured: bool,
    pub ktg_insured: bool,
    pub is_quellensteuer: bool,
    pub quellensteuer_tariff: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((5, 4)))")]
    pub quellensteuer_rate: Option<rust_decimal::Decimal>,
    pub marital_status: String,
    pub canton: String,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
