use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::audit_log;
use konto_db::repository::audit_repo::AuditRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct AuditService;

impl AuditService {
    pub async fn log(
        db: &DatabaseConnection,
        user_id: Option<&str>,
        action: &str,
        entity_type: &str,
        entity_id: Option<&str>,
        old_values: Option<&str>,
        new_values: Option<&str>,
    ) -> Result<(), AppError> {
        let model = audit_log::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            user_id: Set(user_id.map(|s| s.to_string())),
            action: Set(action.to_string()),
            entity_type: Set(entity_type.to_string()),
            entity_id: Set(entity_id.map(|s| s.to_string())),
            old_values: Set(old_values.map(|s| s.to_string())),
            new_values: Set(new_values.map(|s| s.to_string())),
            created_at: Set(Utc::now().naive_utc()),
        };

        AuditRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
