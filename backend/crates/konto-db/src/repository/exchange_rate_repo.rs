use sea_orm::*;

use crate::entities::exchange_rate::{self, ActiveModel, Entity as ExchangeRateEntity};

pub struct ExchangeRateRepo;

impl ExchangeRateRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<exchange_rate::Model>, u64), DbErr> {
        let query =
            ExchangeRateEntity::find().order_by_desc(exchange_rate::Column::ValidDate);

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<exchange_rate::Model>, DbErr> {
        ExchangeRateEntity::find_by_id(id).one(db).await
    }

    pub async fn find_latest(
        db: &DatabaseConnection,
        from_currency_id: &str,
        to_currency_id: &str,
    ) -> Result<Option<exchange_rate::Model>, DbErr> {
        ExchangeRateEntity::find()
            .filter(exchange_rate::Column::FromCurrencyId.eq(from_currency_id))
            .filter(exchange_rate::Column::ToCurrencyId.eq(to_currency_id))
            .order_by_desc(exchange_rate::Column::ValidDate)
            .one(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<exchange_rate::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<exchange_rate::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        ExchangeRateEntity::delete_by_id(id).exec(db).await
    }
}
