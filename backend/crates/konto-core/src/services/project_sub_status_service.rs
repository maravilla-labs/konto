use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::project_sub_status;
use konto_db::repository::project_sub_status_repo::ProjectSubStatusRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct ProjectSubStatusService;

impl ProjectSubStatusService {
    pub async fn list(
        db: &DatabaseConnection,
    ) -> Result<Vec<project_sub_status::Model>, AppError> {
        ProjectSubStatusRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn list_active(
        db: &DatabaseConnection,
    ) -> Result<Vec<project_sub_status::Model>, AppError> {
        ProjectSubStatusRepo::find_active(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<project_sub_status::Model, AppError> {
        ProjectSubStatusRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Sub-status not found".into()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        name: &str,
        sort_order: Option<i32>,
        color: Option<String>,
    ) -> Result<project_sub_status::Model, AppError> {
        let now = Utc::now().naive_utc();
        let model = project_sub_status::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(name.to_string()),
            sort_order: Set(sort_order.unwrap_or(0)),
            color: Set(color.unwrap_or_else(|| "#6b7280".to_string())),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
        };

        ProjectSubStatusRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: Option<String>,
        sort_order: Option<i32>,
        color: Option<String>,
        is_active: Option<bool>,
    ) -> Result<project_sub_status::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let mut model: project_sub_status::ActiveModel = existing.into();

        if let Some(n) = name { model.name = Set(n); }
        if let Some(so) = sort_order { model.sort_order = Set(so); }
        if let Some(c) = color { model.color = Set(c); }
        if let Some(a) = is_active { model.is_active = Set(a); }
        model.updated_at = Set(Utc::now().naive_utc());

        ProjectSubStatusRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        Self::get_by_id(db, id).await?;
        ProjectSubStatusRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
