use sea_orm::*;

use crate::entities::currency_exchange::{self, Entity as CurrencyExchangeEntity};
use crate::entities::journal_line::{self, Entity as JournalLineEntity};

pub struct CurrencyExchangeRepo;

impl CurrencyExchangeRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<currency_exchange::Model>, u64), DbErr> {
        let query = CurrencyExchangeEntity::find().order_by_desc(currency_exchange::Column::Date);
        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: currency_exchange::ActiveModel,
    ) -> Result<currency_exchange::Model, DbErr> {
        model.insert(db).await
    }

    /// Weighted-average base-currency cost basis (base units per 1 foreign unit) of a
    /// bank account's GL account, derived from its own posted journal lines that carry
    /// a `currency_amount` (i.e. every foreign-currency inflow/outflow booked so far).
    /// Returns `None` when the account has no FX-annotated history yet.
    pub async fn weighted_average_rate(
        db: &DatabaseConnection,
        gl_account_id: &str,
        as_of_date: chrono::NaiveDate,
    ) -> Result<Option<rust_decimal::Decimal>, DbErr> {
        use crate::entities::journal_entry;

        let lines = JournalLineEntity::find()
            .filter(journal_line::Column::AccountId.eq(gl_account_id))
            .filter(journal_line::Column::CurrencyAmount.is_not_null())
            .inner_join(journal_entry::Entity)
            .filter(journal_entry::Column::Date.lte(as_of_date))
            .filter(journal_entry::Column::Status.eq(konto_common::enums::JournalStatus::Posted.as_str()))
            .all(db)
            .await?;

        let mut total_currency = rust_decimal::Decimal::ZERO;
        let mut total_base = rust_decimal::Decimal::ZERO;
        for line in lines {
            if let (Some(currency_amount), Some(base_amount)) =
                (line.currency_amount, line.base_currency_amount)
            {
                total_currency += currency_amount;
                total_base += base_amount;
            }
        }

        if total_currency == rust_decimal::Decimal::ZERO {
            return Ok(None);
        }
        Ok(Some(total_base / total_currency))
    }
}
