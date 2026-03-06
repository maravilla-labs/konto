use sea_orm::*;

use crate::entities::audit_log::{self, Entity as AuditLogEntity};

pub struct AuditRepo;

impl AuditRepo {
    pub async fn create(db: &DatabaseConnection, model: audit_log::ActiveModel) -> Result<audit_log::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<audit_log::Model>, u64), DbErr> {
        let query = AuditLogEntity::find().order_by_desc(audit_log::Column::CreatedAt);
        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn find_filtered(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        entity_type: Option<&str>,
        action: Option<&str>,
        user_id: Option<&str>,
        from_date: Option<chrono::NaiveDateTime>,
        to_date: Option<chrono::NaiveDateTime>,
    ) -> Result<(Vec<audit_log::Model>, u64), DbErr> {
        let mut query = AuditLogEntity::find().order_by_desc(audit_log::Column::CreatedAt);

        if let Some(et) = entity_type {
            query = query.filter(audit_log::Column::EntityType.eq(et));
        }
        if let Some(a) = action {
            query = query.filter(audit_log::Column::Action.eq(a));
        }
        if let Some(uid) = user_id {
            query = query.filter(audit_log::Column::UserId.eq(uid));
        }
        if let Some(from) = from_date {
            query = query.filter(audit_log::Column::CreatedAt.gte(from));
        }
        if let Some(to) = to_date {
            query = query.filter(audit_log::Column::CreatedAt.lte(to));
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }
}
