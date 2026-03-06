use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::journal_attachment;
use konto_db::repository::journal_attachment_repo::JournalAttachmentRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::storage::StorageService;

pub struct JournalAttachmentService;

impl JournalAttachmentService {
    pub async fn list_by_entry(
        db: &DatabaseConnection,
        entry_id: &str,
    ) -> Result<Vec<journal_attachment::Model>, AppError> {
        JournalAttachmentRepo::find_by_entry(db, entry_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn upload(
        db: &DatabaseConnection,
        storage: &dyn StorageService,
        entry_id: &str,
        file_name: &str,
        data: &[u8],
        mime_type: &str,
        uploaded_by: Option<&str>,
    ) -> Result<journal_attachment::Model, AppError> {
        let id = Uuid::new_v4().to_string();
        let ext = file_name.rsplit('.').next().unwrap_or("bin");
        let storage_key = format!("journal-attachments/{entry_id}/{id}.{ext}");

        storage.upload(&storage_key, data, mime_type).await?;

        let model = journal_attachment::ActiveModel {
            id: Set(id),
            journal_entry_id: Set(entry_id.to_string()),
            file_name: Set(file_name.to_string()),
            storage_key: Set(storage_key),
            file_size: Set(data.len() as i64),
            mime_type: Set(mime_type.to_string()),
            uploaded_by: Set(uploaded_by.map(|s| s.to_string())),
            created_at: Set(Utc::now()),
        };

        JournalAttachmentRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn download(
        db: &DatabaseConnection,
        storage: &dyn StorageService,
        id: &str,
    ) -> Result<(journal_attachment::Model, Vec<u8>), AppError> {
        let att = JournalAttachmentRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Attachment not found".to_string()))?;

        let data = storage.download(&att.storage_key).await?;
        Ok((att, data))
    }

    pub async fn delete(
        db: &DatabaseConnection,
        storage: &dyn StorageService,
        id: &str,
    ) -> Result<(), AppError> {
        let att = JournalAttachmentRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Attachment not found".to_string()))?;

        storage.delete(&att.storage_key).await?;

        JournalAttachmentRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
