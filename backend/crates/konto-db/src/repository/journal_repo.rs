use sea_orm::*;
use sea_orm::sea_query::Expr;

use crate::entities::journal_entry::{self, Entity as JournalEntryEntity};
use crate::entities::journal_line::{self, Entity as JournalLineEntity};

pub struct JournalRepo;

impl JournalRepo {
    #[allow(clippy::too_many_arguments)]
    pub async fn find_entries_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        search: Option<&str>,
        date_from: Option<&str>,
        date_to: Option<&str>,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
    ) -> Result<(Vec<journal_entry::Model>, u64), DbErr> {
        let is_asc = sort_order.map(|s| s.eq_ignore_ascii_case("asc")).unwrap_or(false);

        let mut query = JournalEntryEntity::find();

        // Apply sort
        match sort_by.unwrap_or("date") {
            "reference" => {
                query = if is_asc {
                    query.order_by_asc(journal_entry::Column::Reference)
                } else {
                    query.order_by_desc(journal_entry::Column::Reference)
                };
            }
            "description" => {
                query = if is_asc {
                    query.order_by_asc(journal_entry::Column::Description)
                } else {
                    query.order_by_desc(journal_entry::Column::Description)
                };
            }
            "status" => {
                query = if is_asc {
                    query.order_by_asc(journal_entry::Column::Status)
                } else {
                    query.order_by_desc(journal_entry::Column::Status)
                };
            }
            _ => {
                query = if is_asc {
                    query.order_by_asc(journal_entry::Column::Date)
                } else {
                    query.order_by_desc(journal_entry::Column::Date)
                };
            }
        }

        if let Some(search) = search {
            query = query.filter(
                Condition::any()
                    .add(journal_entry::Column::Description.contains(search))
                    .add(journal_entry::Column::Reference.contains(search)),
            );
        }

        // Use string comparison for SQLite text-stored dates
        if let Some(from) = date_from {
            query = query.filter(
                Expr::col(journal_entry::Column::Date).gte(from),
            );
        }
        if let Some(to) = date_to {
            query = query.filter(
                Expr::col(journal_entry::Column::Date).lte(to),
            );
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_entry_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<journal_entry::Model>, DbErr> {
        JournalEntryEntity::find_by_id(id).one(db).await
    }

    pub async fn find_lines_by_entry(
        db: &DatabaseConnection,
        entry_id: &str,
    ) -> Result<Vec<journal_line::Model>, DbErr> {
        JournalLineEntity::find()
            .filter(journal_line::Column::JournalEntryId.eq(entry_id))
            .all(db)
            .await
    }

    pub async fn create_entry(
        db: &DatabaseConnection,
        entry: journal_entry::ActiveModel,
    ) -> Result<journal_entry::Model, DbErr> {
        entry.insert(db).await
    }

    pub async fn create_line(
        db: &DatabaseConnection,
        line: journal_line::ActiveModel,
    ) -> Result<journal_line::Model, DbErr> {
        line.insert(db).await
    }

    pub async fn find_all_by_status(
        db: &DatabaseConnection,
        status: &str,
    ) -> Result<Vec<journal_entry::Model>, DbErr> {
        JournalEntryEntity::find()
            .filter(journal_entry::Column::Status.eq(status))
            .all(db)
            .await
    }

    pub async fn update_entry(
        db: &DatabaseConnection,
        entry: journal_entry::ActiveModel,
    ) -> Result<journal_entry::Model, DbErr> {
        entry.update(db).await
    }

    /// Find journal entries by exact reference string.
    pub async fn find_entries_by_reference(
        db: &DatabaseConnection,
        reference: &str,
    ) -> Result<Vec<journal_entry::Model>, DbErr> {
        JournalEntryEntity::find()
            .filter(journal_entry::Column::Reference.eq(reference))
            .all(db)
            .await
    }

    pub async fn update_line(
        db: &DatabaseConnection,
        line: journal_line::ActiveModel,
    ) -> Result<journal_line::Model, DbErr> {
        line.update(db).await
    }
}
