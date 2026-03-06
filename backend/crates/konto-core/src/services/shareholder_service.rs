use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::shareholder;
use konto_db::repository::shareholder_repo::ShareholderRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct ShareholderService;

impl ShareholderService {
    pub async fn list(db: &DatabaseConnection) -> Result<Vec<shareholder::Model>, AppError> {
        ShareholderRepo::find_all_sorted(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<shareholder::Model, AppError> {
        ShareholderRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Shareholder not found".into()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        name: &str,
        city: &str,
        role: &str,
        signing_rights: Option<String>,
        sort_order: i32,
    ) -> Result<shareholder::Model, AppError> {
        let now = Utc::now().naive_utc();
        let model = shareholder::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(name.to_string()),
            city: Set(city.to_string()),
            role: Set(role.to_string()),
            signing_rights: Set(signing_rights),
            sort_order: Set(sort_order),
            created_at: Set(now),
            updated_at: Set(now),
        };
        ShareholderRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: &str,
        city: &str,
        role: &str,
        signing_rights: Option<String>,
        sort_order: i32,
    ) -> Result<shareholder::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let now = Utc::now().naive_utc();
        let mut model: shareholder::ActiveModel = existing.into();
        model.name = Set(name.to_string());
        model.city = Set(city.to_string());
        model.role = Set(role.to_string());
        model.signing_rights = Set(signing_rights);
        model.sort_order = Set(sort_order);
        model.updated_at = Set(now);
        ShareholderRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        ShareholderRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
