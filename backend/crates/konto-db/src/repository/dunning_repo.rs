use sea_orm::*;
use crate::entities::{dunning_entry, dunning_level};

pub struct DunningRepo;

impl DunningRepo {
    // ── Dunning Levels ──

    pub async fn find_all_levels(db: &DatabaseConnection) -> Result<Vec<dunning_level::Model>, DbErr> {
        dunning_level::Entity::find()
            .order_by_asc(dunning_level::Column::Level)
            .all(db)
            .await
    }

    pub async fn find_active_levels(db: &DatabaseConnection) -> Result<Vec<dunning_level::Model>, DbErr> {
        dunning_level::Entity::find()
            .filter(dunning_level::Column::IsActive.eq(true))
            .order_by_asc(dunning_level::Column::Level)
            .all(db)
            .await
    }

    pub async fn find_level_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<dunning_level::Model>, DbErr> {
        dunning_level::Entity::find_by_id(id).one(db).await
    }

    pub async fn update_level(
        db: &DatabaseConnection,
        model: dunning_level::ActiveModel,
    ) -> Result<dunning_level::Model, DbErr> {
        model.update(db).await
    }

    // ── Dunning Entries ──

    pub async fn find_entries_by_invoice(
        db: &DatabaseConnection,
        invoice_id: &str,
    ) -> Result<Vec<dunning_entry::Model>, DbErr> {
        dunning_entry::Entity::find()
            .filter(dunning_entry::Column::InvoiceId.eq(invoice_id))
            .order_by_desc(dunning_entry::Column::SentAt)
            .all(db)
            .await
    }

    pub async fn find_latest_entry_for_invoice(
        db: &DatabaseConnection,
        invoice_id: &str,
    ) -> Result<Option<dunning_entry::Model>, DbErr> {
        dunning_entry::Entity::find()
            .filter(dunning_entry::Column::InvoiceId.eq(invoice_id))
            .order_by_desc(dunning_entry::Column::SentAt)
            .one(db)
            .await
    }

    pub async fn create_entry(
        db: &DatabaseConnection,
        model: dunning_entry::ActiveModel,
    ) -> Result<dunning_entry::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn find_all_entries_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        invoice_id: Option<&str>,
    ) -> Result<(Vec<dunning_entry::Model>, u64), DbErr> {
        let mut query = dunning_entry::Entity::find()
            .order_by_desc(dunning_entry::Column::SentAt);

        if let Some(inv_id) = invoice_id {
            query = query.filter(dunning_entry::Column::InvoiceId.eq(inv_id));
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_pages().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }
}
