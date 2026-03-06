use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "exchange_rates")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub from_currency_id: String,
    pub to_currency_id: String,
    #[sea_orm(column_type = "Decimal(Some((10, 6)))")]
    pub rate: rust_decimal::Decimal,
    pub valid_date: chrono::NaiveDate,
    pub source: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::currency::Entity",
        from = "Column::FromCurrencyId",
        to = "super::currency::Column::Id"
    )]
    FromCurrency,
    #[sea_orm(
        belongs_to = "super::currency::Entity",
        from = "Column::ToCurrencyId",
        to = "super::currency::Column::Id"
    )]
    ToCurrency,
}

impl ActiveModelBehavior for ActiveModel {}
