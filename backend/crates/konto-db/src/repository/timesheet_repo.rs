use sea_orm::*;

use crate::entities::timesheet::{self, Entity as TimesheetEntity};

pub struct TimesheetRepo;

impl TimesheetRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        user_id_filter: Option<&str>,
    ) -> Result<(Vec<timesheet::Model>, u64), DbErr> {
        let mut query = TimesheetEntity::find()
            .order_by_desc(timesheet::Column::PeriodStart);

        if let Some(uid) = user_id_filter {
            query = query.filter(timesheet::Column::UserId.eq(uid));
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<timesheet::Model>, DbErr> {
        TimesheetEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: timesheet::ActiveModel,
    ) -> Result<timesheet::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: timesheet::ActiveModel,
    ) -> Result<timesheet::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        TimesheetEntity::delete_by_id(id).exec(db).await
    }
}
