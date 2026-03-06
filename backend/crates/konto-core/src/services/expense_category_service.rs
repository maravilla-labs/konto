use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::expense_category;
use konto_db::repository::expense_category_repo::ExpenseCategoryRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct ExpenseCategoryService;

impl ExpenseCategoryService {
    pub async fn list(
        db: &DatabaseConnection,
    ) -> Result<Vec<expense_category::Model>, AppError> {
        ExpenseCategoryRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn list_active(
        db: &DatabaseConnection,
    ) -> Result<Vec<expense_category::Model>, AppError> {
        ExpenseCategoryRepo::find_active(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        name: &str,
        account_id: Option<String>,
    ) -> Result<expense_category::Model, AppError> {
        let now = Utc::now().naive_utc();
        let model = expense_category::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(name.to_string()),
            account_id: Set(account_id),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
        };
        ExpenseCategoryRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: &str,
        account_id: Option<String>,
        is_active: bool,
    ) -> Result<expense_category::Model, AppError> {
        let existing = ExpenseCategoryRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Expense category not found".into()))?;

        let now = Utc::now().naive_utc();
        let mut model: expense_category::ActiveModel = existing.into();
        model.name = Set(name.to_string());
        model.account_id = Set(account_id);
        model.is_active = Set(is_active);
        model.updated_at = Set(now);

        ExpenseCategoryRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        ExpenseCategoryRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
