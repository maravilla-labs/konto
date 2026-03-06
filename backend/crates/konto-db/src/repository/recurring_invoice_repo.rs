use sea_orm::*;

use crate::entities::recurring_invoice::{self, Entity as RecurringInvoiceEntity};

pub struct RecurringInvoiceRepo;

impl RecurringInvoiceRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        is_active_filter: Option<bool>,
        search: Option<&str>,
    ) -> Result<(Vec<recurring_invoice::Model>, u64), DbErr> {
        let mut query =
            RecurringInvoiceEntity::find().order_by_desc(recurring_invoice::Column::NextRunDate);

        if let Some(active) = is_active_filter {
            query = query.filter(recurring_invoice::Column::IsActive.eq(active));
        }

        if let Some(search) = search {
            query = query.filter(
                Condition::any()
                    .add(recurring_invoice::Column::Frequency.contains(search))
                    .add(recurring_invoice::Column::TemplateData.contains(search)),
            );
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<recurring_invoice::Model>, DbErr> {
        RecurringInvoiceEntity::find_by_id(id).one(db).await
    }

    pub async fn find_due(
        db: &DatabaseConnection,
    ) -> Result<Vec<recurring_invoice::Model>, DbErr> {
        let today = chrono::Utc::now().naive_utc().date();
        RecurringInvoiceEntity::find()
            .filter(recurring_invoice::Column::IsActive.eq(true))
            .filter(recurring_invoice::Column::NextRunDate.lte(today))
            .filter(
                Condition::any()
                    .add(recurring_invoice::Column::EndDate.is_null())
                    .add(recurring_invoice::Column::EndDate.gte(today)),
            )
            .all(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: recurring_invoice::ActiveModel,
    ) -> Result<recurring_invoice::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: recurring_invoice::ActiveModel,
    ) -> Result<recurring_invoice::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<DeleteResult, DbErr> {
        RecurringInvoiceEntity::delete_by_id(id).exec(db).await
    }
}
