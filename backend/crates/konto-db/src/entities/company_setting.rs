use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "company_settings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub legal_name: String,
    pub trade_name: Option<String>,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub vat_number: Option<String>,
    pub vat_method: String,
    pub flat_rate_percentage: Option<rust_decimal::Decimal>,
    pub register_number: Option<String>,
    pub logo_url: Option<String>,
    pub default_currency_id: Option<String>,
    pub date_format: String,
    pub number_format: String,
    pub ui_language: String,
    pub fiscal_year_start_month: i32,
    pub tax_id_label: String,
    pub jurisdiction: String,
    pub legal_entity_type: Option<String>,
    pub audit_optout: bool,
    pub project_number_auto: bool,
    pub project_number_prefix: String,
    pub project_number_restart_yearly: bool,
    pub project_number_start: i32,
    pub project_number_min_length: i32,
    pub customer_number_auto: bool,
    pub customer_number_prefix: String,
    pub customer_number_restart_yearly: bool,
    pub customer_number_start: i32,
    pub customer_number_min_length: i32,
    pub employee_number_auto: bool,
    pub employee_number_prefix: String,
    pub employee_number_restart_yearly: bool,
    pub employee_number_start: i32,
    pub employee_number_min_length: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
