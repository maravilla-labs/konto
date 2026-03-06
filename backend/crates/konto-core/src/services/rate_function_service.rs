use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::rate_function;
use konto_db::repository::rate_function_repo::RateFunctionRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct RateFunctionService;

impl RateFunctionService {
    pub async fn list(db: &DatabaseConnection) -> Result<Vec<rate_function::Model>, AppError> {
        RateFunctionRepo::find_all_sorted(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<rate_function::Model, AppError> {
        RateFunctionRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Rate function not found".into()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        name: &str,
        description: Option<String>,
        hourly_rate: Decimal,
        sort_order: i32,
    ) -> Result<rate_function::Model, AppError> {
        let now = Utc::now().naive_utc();
        let model = rate_function::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(name.to_string()),
            description: Set(description),
            hourly_rate: Set(hourly_rate),
            is_active: Set(true),
            sort_order: Set(sort_order),
            created_at: Set(now),
            updated_at: Set(now),
        };
        RateFunctionRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: &str,
        description: Option<String>,
        hourly_rate: Decimal,
        is_active: bool,
        sort_order: i32,
    ) -> Result<rate_function::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let now = Utc::now().naive_utc();
        let mut model: rate_function::ActiveModel = existing.into();
        model.name = Set(name.to_string());
        model.description = Set(description);
        model.hourly_rate = Set(hourly_rate);
        model.is_active = Set(is_active);
        model.sort_order = Set(sort_order);
        model.updated_at = Set(now);
        RateFunctionRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn deactivate(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<rate_function::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let now = Utc::now().naive_utc();
        let mut model: rate_function::ActiveModel = existing.into();
        model.is_active = Set(false);
        model.updated_at = Set(now);
        RateFunctionRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        RateFunctionRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
