use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::project_activity_type;
use konto_db::repository::project_activity_type_repo::ProjectActivityTypeRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct ProjectActivityTypeService;

impl ProjectActivityTypeService {
    pub async fn list_for_project(
        db: &DatabaseConnection,
        project_id: &str,
    ) -> Result<Vec<project_activity_type::Model>, AppError> {
        ProjectActivityTypeRepo::find_by_project(db, project_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<project_activity_type::Model, AppError> {
        ProjectActivityTypeRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Project activity type not found".into()))
    }

    pub async fn add(
        db: &DatabaseConnection,
        project_id: &str,
        activity_type_id: &str,
        rate: Option<Decimal>,
        budget_hours: Option<Decimal>,
        chargeable: bool,
    ) -> Result<project_activity_type::Model, AppError> {
        // Check for duplicate
        let existing = ProjectActivityTypeRepo::find_by_project_and_activity_type(
            db, project_id, activity_type_id,
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        if existing.is_some() {
            return Err(AppError::Conflict(
                "Activity type already assigned to this project".into(),
            ));
        }

        let now = Utc::now().naive_utc();
        let model = project_activity_type::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            project_id: Set(project_id.to_string()),
            activity_type_id: Set(activity_type_id.to_string()),
            rate: Set(rate),
            budget_hours: Set(budget_hours),
            chargeable: Set(chargeable),
            created_at: Set(now),
            updated_at: Set(now),
        };
        ProjectActivityTypeRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        rate: Option<Option<Decimal>>,
        budget_hours: Option<Option<Decimal>>,
        chargeable: Option<bool>,
    ) -> Result<project_activity_type::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let mut model: project_activity_type::ActiveModel = existing.into();

        if let Some(r) = rate {
            model.rate = Set(r);
        }
        if let Some(bh) = budget_hours {
            model.budget_hours = Set(bh);
        }
        if let Some(c) = chargeable {
            model.chargeable = Set(c);
        }
        model.updated_at = Set(Utc::now().naive_utc());

        ProjectActivityTypeRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn remove(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        ProjectActivityTypeRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
