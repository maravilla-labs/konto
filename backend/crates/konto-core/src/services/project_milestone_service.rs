use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::ProjectMilestoneStatus;
use konto_db::entities::project_milestone;
use konto_db::repository::project_milestone_repo::ProjectMilestoneRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct ProjectMilestoneService;

impl ProjectMilestoneService {
    pub async fn list_for_project(
        db: &DatabaseConnection,
        project_id: &str,
    ) -> Result<Vec<project_milestone::Model>, AppError> {
        ProjectMilestoneRepo::find_by_project(db, project_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<project_milestone::Model, AppError> {
        ProjectMilestoneRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Project milestone not found".into()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        project_id: &str,
        project_item_id: Option<String>,
        name: &str,
        description: Option<String>,
        target_date: chrono::NaiveDate,
    ) -> Result<project_milestone::Model, AppError> {
        let now = Utc::now().naive_utc();
        let model = project_milestone::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            project_id: Set(project_id.to_string()),
            project_item_id: Set(project_item_id),
            name: Set(name.to_string()),
            description: Set(description),
            target_date: Set(target_date),
            status: Set(ProjectMilestoneStatus::Pending.to_string()),
            reached_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };
        ProjectMilestoneRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: Option<String>,
        description: Option<Option<String>>,
        target_date: Option<chrono::NaiveDate>,
        project_item_id: Option<Option<String>>,
    ) -> Result<project_milestone::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let now = Utc::now().naive_utc();
        let mut model: project_milestone::ActiveModel = existing.into();

        if let Some(v) = name {
            model.name = Set(v);
        }
        if let Some(v) = description {
            model.description = Set(v);
        }
        if let Some(v) = target_date {
            model.target_date = Set(v);
        }
        if let Some(v) = project_item_id {
            model.project_item_id = Set(v);
        }
        model.updated_at = Set(now);

        ProjectMilestoneRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Mark milestone as reached: set status="reached" and reached_at=now.
    pub async fn reach(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<project_milestone::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let now = Utc::now().naive_utc();
        let mut model: project_milestone::ActiveModel = existing.into();
        model.status = Set(ProjectMilestoneStatus::Reached.to_string());
        model.reached_at = Set(Some(now));
        model.updated_at = Set(now);

        ProjectMilestoneRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        ProjectMilestoneRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
