use konto_common::error::AppError;
use konto_db::entities::activity_type;
use konto_db::repository::activity_type_repo::ActivityTypeRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct ActivityTypeService;

impl ActivityTypeService {
    pub async fn list(db: &DatabaseConnection) -> Result<Vec<activity_type::Model>, AppError> {
        ActivityTypeRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<activity_type::Model, AppError> {
        ActivityTypeRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Activity type not found".into()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        name: &str,
        unit_type: Option<String>,
        default_rate: Option<Decimal>,
    ) -> Result<activity_type::Model, AppError> {
        let model = activity_type::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(name.to_string()),
            is_active: Set(true),
            unit_type: Set(unit_type.unwrap_or_else(|| "hour".to_string())),
            default_rate: Set(default_rate),
        };
        ActivityTypeRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: &str,
        is_active: bool,
        unit_type: Option<String>,
        default_rate: Option<Option<Decimal>>,
    ) -> Result<activity_type::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;

        let mut model: activity_type::ActiveModel = existing.into();
        model.name = Set(name.to_string());
        model.is_active = Set(is_active);
        if let Some(ut) = unit_type {
            model.unit_type = Set(ut);
        }
        if let Some(dr) = default_rate {
            model.default_rate = Set(dr);
        }

        ActivityTypeRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        ActivityTypeRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
