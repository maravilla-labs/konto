use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::project_member;
use konto_db::repository::project_member_repo::ProjectMemberRepo;
use konto_db::repository::project_repo::ProjectRepo;
use konto_db::repository::rate_function_repo::RateFunctionRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct ProjectMemberService;

impl ProjectMemberService {
    pub async fn list_for_project(
        db: &DatabaseConnection,
        project_id: &str,
    ) -> Result<Vec<project_member::Model>, AppError> {
        ProjectMemberRepo::find_by_project(db, project_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<project_member::Model, AppError> {
        ProjectMemberRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Project member not found".into()))
    }

    pub async fn add_member(
        db: &DatabaseConnection,
        project_id: &str,
        user_id: &str,
        rate_function_id: Option<String>,
        hourly_rate: Option<Decimal>,
        role_label: Option<String>,
        budget_hours: Option<Decimal>,
    ) -> Result<project_member::Model, AppError> {
        // Check for duplicate
        let existing = ProjectMemberRepo::find_by_project_and_user(db, project_id, user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        if existing.is_some() {
            return Err(AppError::Conflict(
                "User is already a member of this project".into(),
            ));
        }

        let now = Utc::now().naive_utc();
        let model = project_member::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            project_id: Set(project_id.to_string()),
            user_id: Set(user_id.to_string()),
            rate_function_id: Set(rate_function_id),
            hourly_rate: Set(hourly_rate),
            role_label: Set(role_label),
            budget_hours: Set(budget_hours),
            joined_at: Set(now),
            left_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };
        ProjectMemberRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update_member(
        db: &DatabaseConnection,
        id: &str,
        rate_function_id: Option<Option<String>>,
        hourly_rate: Option<Option<Decimal>>,
        role_label: Option<Option<String>>,
        budget_hours: Option<Option<Decimal>>,
    ) -> Result<project_member::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let now = Utc::now().naive_utc();
        let mut model: project_member::ActiveModel = existing.into();

        if let Some(rf_id) = rate_function_id {
            model.rate_function_id = Set(rf_id);
        }
        if let Some(hr) = hourly_rate {
            model.hourly_rate = Set(hr);
        }
        if let Some(rl) = role_label {
            model.role_label = Set(rl);
        }
        if let Some(bh) = budget_hours {
            model.budget_hours = Set(bh);
        }
        model.updated_at = Set(now);

        ProjectMemberRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn remove_member(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        ProjectMemberRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    /// Resolve the effective hourly rate for a user on a project.
    /// Priority: member.hourly_rate > rate_function.hourly_rate > project.hourly_rate > None
    #[allow(clippy::collapsible_if)]
    pub async fn resolve_rate(
        db: &DatabaseConnection,
        project_id: &str,
        user_id: &str,
    ) -> Result<Option<Decimal>, AppError> {
        // 1. Check project member record
        let member = ProjectMemberRepo::find_by_project_and_user(db, project_id, user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if let Some(ref m) = member {
            // 1. Member-level override
            if let Some(rate) = m.hourly_rate {
                return Ok(Some(rate));
            }

            // 2. Rate function
            if let Some(rf) = m.rate_function_id.as_ref() {
                if let Some(rf) = RateFunctionRepo::find_by_id(db, rf)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?
                {
                    return Ok(Some(rf.hourly_rate));
                }
            }
        }

        // 3. Project-level hourly rate
        if let Some(rate) = ProjectRepo::find_by_id(db, project_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .and_then(|p| p.hourly_rate)
        {
            return Ok(Some(rate));
        }

        // 4. None
        Ok(None)
    }
}
