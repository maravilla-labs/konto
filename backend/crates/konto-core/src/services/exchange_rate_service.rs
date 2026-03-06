use chrono::{NaiveDate, Utc};
use konto_common::error::AppError;
use konto_db::entities::exchange_rate;
use konto_db::repository::exchange_rate_repo::ExchangeRateRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct ExchangeRateService;

impl ExchangeRateService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<exchange_rate::Model>, u64), AppError> {
        ExchangeRateRepo::find_paginated(db, page, per_page)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<exchange_rate::Model, AppError> {
        ExchangeRateRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Exchange rate not found".to_string()))
    }

    pub async fn get_latest(
        db: &DatabaseConnection,
        from_currency_id: &str,
        to_currency_id: &str,
    ) -> Result<exchange_rate::Model, AppError> {
        ExchangeRateRepo::find_latest(db, from_currency_id, to_currency_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("No exchange rate found".to_string()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        from_currency_id: &str,
        to_currency_id: &str,
        rate: Decimal,
        valid_date: NaiveDate,
        source: Option<String>,
    ) -> Result<exchange_rate::Model, AppError> {
        let now = Utc::now().naive_utc();

        let model = exchange_rate::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            from_currency_id: Set(from_currency_id.to_string()),
            to_currency_id: Set(to_currency_id.to_string()),
            rate: Set(rate),
            valid_date: Set(valid_date),
            source: Set(source),
            created_at: Set(now),
            updated_at: Set(now),
        };

        ExchangeRateRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        rate: Option<Decimal>,
        valid_date: Option<NaiveDate>,
        source: Option<Option<String>>,
    ) -> Result<exchange_rate::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let mut model: exchange_rate::ActiveModel = existing.into();

        if let Some(rate) = rate {
            model.rate = Set(rate);
        }
        if let Some(date) = valid_date {
            model.valid_date = Set(date);
        }
        if let Some(src) = source {
            model.source = Set(src);
        }
        model.updated_at = Set(Utc::now().naive_utc());

        ExchangeRateRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        Self::get_by_id(db, id).await?;
        ExchangeRateRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
