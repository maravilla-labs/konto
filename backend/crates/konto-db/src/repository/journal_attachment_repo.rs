use sea_orm::*;
use crate::entities::journal_attachment;

pub struct JournalAttachmentRepo;

impl JournalAttachmentRepo {
    pub async fn find_by_entry(
        db: &DatabaseConnection,
        entry_id: &str,
    ) -> Result<Vec<journal_attachment::Model>, DbErr> {
        journal_attachment::Entity::find()
            .filter(journal_attachment::Column::JournalEntryId.eq(entry_id))
            .order_by_asc(journal_attachment::Column::CreatedAt)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<journal_attachment::Model>, DbErr> {
        journal_attachment::Entity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: journal_attachment::ActiveModel,
    ) -> Result<journal_attachment::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        journal_attachment::Entity::delete_by_id(id).exec(db).await
    }
}
