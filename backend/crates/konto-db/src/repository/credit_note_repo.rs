use sea_orm::*;

use crate::entities::credit_note::{self, Entity as CreditNoteEntity};
use crate::entities::credit_note_line::{self, Entity as CreditNoteLineEntity};

pub struct CreditNoteRepo;

impl CreditNoteRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        status_filter: Option<&str>,
        search: Option<&str>,
    ) -> Result<(Vec<credit_note::Model>, u64), DbErr> {
        let mut query = CreditNoteEntity::find()
            .order_by_desc(credit_note::Column::IssueDate);

        if let Some(status) = status_filter {
            query = query.filter(credit_note::Column::Status.eq(status));
        }

        if let Some(search) = search {
            query = query.filter(
                Condition::any()
                    .add(credit_note::Column::CreditNoteNumber.contains(search))
                    .add(credit_note::Column::Notes.contains(search)),
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
    ) -> Result<Option<credit_note::Model>, DbErr> {
        CreditNoteEntity::find_by_id(id).one(db).await
    }

    pub async fn find_lines_by_credit_note(
        db: &DatabaseConnection,
        credit_note_id: &str,
    ) -> Result<Vec<credit_note_line::Model>, DbErr> {
        CreditNoteLineEntity::find()
            .filter(credit_note_line::Column::CreditNoteId.eq(credit_note_id))
            .order_by_asc(credit_note_line::Column::SortOrder)
            .all(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: credit_note::ActiveModel,
    ) -> Result<credit_note::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: credit_note::ActiveModel,
    ) -> Result<credit_note::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<DeleteResult, DbErr> {
        CreditNoteEntity::delete_by_id(id).exec(db).await
    }

    pub async fn create_line(
        db: &DatabaseConnection,
        model: credit_note_line::ActiveModel,
    ) -> Result<credit_note_line::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn delete_lines_by_credit_note(
        db: &DatabaseConnection,
        credit_note_id: &str,
    ) -> Result<DeleteResult, DbErr> {
        CreditNoteLineEntity::delete_many()
            .filter(credit_note_line::Column::CreditNoteId.eq(credit_note_id))
            .exec(db)
            .await
    }

    pub async fn next_credit_note_number(
        db: &DatabaseConnection,
        year: i32,
    ) -> Result<String, DbErr> {
        let prefix = format!("GS-{year}-");
        let count = CreditNoteEntity::find()
            .filter(credit_note::Column::CreditNoteNumber.starts_with(&prefix))
            .count(db)
            .await?;
        Ok(format!("GS-{year}-{:03}", count + 1))
    }
}
