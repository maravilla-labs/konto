use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "fixed_assets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub account_id: String,
    pub depreciation_account_id: String,
    pub acquisition_date: chrono::NaiveDate,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub acquisition_cost: rust_decimal::Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub residual_value: rust_decimal::Decimal,
    pub useful_life_years: i32,
    pub depreciation_method: String,
    #[sea_orm(column_type = "Decimal(Some((5, 4)))")]
    pub declining_rate: Option<rust_decimal::Decimal>,
    pub status: String,
    pub disposed_date: Option<chrono::NaiveDate>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
