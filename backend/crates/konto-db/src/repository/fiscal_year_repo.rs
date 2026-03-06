use sea_orm::*;

use crate::entities::fiscal_period::{self, Entity as FiscalPeriodEntity};
use crate::entities::fiscal_year::{self, ActiveModel, Entity as FiscalYearEntity};

pub struct FiscalYearRepo;

impl FiscalYearRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        search: Option<&str>,
    ) -> Result<(Vec<fiscal_year::Model>, u64), DbErr> {
        let mut query = FiscalYearEntity::find().order_by_desc(fiscal_year::Column::StartDate);

        if let Some(search) = search {
            query = query.filter(fiscal_year::Column::Name.contains(search));
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<fiscal_year::Model>, DbErr> {
        FiscalYearEntity::find()
            .order_by_desc(fiscal_year::Column::StartDate)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<fiscal_year::Model>, DbErr> {
        FiscalYearEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<fiscal_year::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<fiscal_year::Model, DbErr> {
        model.update(db).await
    }
}

pub struct FiscalPeriodRepo;

impl FiscalPeriodRepo {
    pub async fn find_by_fiscal_year_id(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
    ) -> Result<Vec<fiscal_period::Model>, DbErr> {
        FiscalPeriodEntity::find()
            .filter(fiscal_period::Column::FiscalYearId.eq(fiscal_year_id))
            .order_by_asc(fiscal_period::Column::PeriodNumber)
            .all(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: fiscal_period::ActiveModel,
    ) -> Result<fiscal_period::Model, DbErr> {
        model.insert(db).await
    }
}
