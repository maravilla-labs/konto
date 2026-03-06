use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::project_document;
use konto_db::repository::project_document_repo::ProjectDocumentRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::storage::StorageService;

pub struct ProjectDocumentService;

impl ProjectDocumentService {
    pub async fn list_for_project(
        db: &DatabaseConnection,
        project_id: &str,
    ) -> Result<Vec<project_document::Model>, AppError> {
        ProjectDocumentRepo::find_by_project(db, project_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn upload(
        db: &DatabaseConnection,
        storage: &dyn StorageService,
        project_id: &str,
        project_item_id: Option<&str>,
        file_name: &str,
        content_type: &str,
        data: &[u8],
        user_id: Option<&str>,
    ) -> Result<project_document::Model, AppError> {
        let id = Uuid::new_v4().to_string();
        let ext = file_name.rsplit('.').next().unwrap_or("bin");
        let storage_key = format!("project-documents/{project_id}/{id}.{ext}");

        storage.upload(&storage_key, data, content_type).await?;

        let model = project_document::ActiveModel {
            id: Set(id),
            project_id: Set(project_id.to_string()),
            project_item_id: Set(project_item_id.map(|s| s.to_string())),
            file_name: Set(file_name.to_string()),
            file_path: Set(storage_key),
            file_size: Set(data.len() as i32),
            content_type: Set(Some(content_type.to_string())),
            uploaded_by: Set(user_id.map(|s| s.to_string())),
            created_at: Set(Utc::now().naive_utc()),
        };

        ProjectDocumentRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn download(
        db: &DatabaseConnection,
        storage: &dyn StorageService,
        id: &str,
    ) -> Result<(project_document::Model, Vec<u8>), AppError> {
        let doc = ProjectDocumentRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Project document not found".to_string()))?;

        let data = storage.download(&doc.file_path).await?;
        Ok((doc, data))
    }

    pub async fn delete(
        db: &DatabaseConnection,
        storage: &dyn StorageService,
        id: &str,
    ) -> Result<(), AppError> {
        let doc = ProjectDocumentRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Project document not found".to_string()))?;

        storage.delete(&doc.file_path).await?;

        ProjectDocumentRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
