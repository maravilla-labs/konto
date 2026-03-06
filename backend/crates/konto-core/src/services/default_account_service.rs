use konto_common::error::AppError;
use konto_db::entities::default_account;
use konto_db::repository::default_account_repo::DefaultAccountRepo;
use sea_orm::{DatabaseConnection, Set};

pub struct DefaultAccountService;

impl DefaultAccountService {
    pub async fn list(
        db: &DatabaseConnection,
    ) -> Result<Vec<default_account::Model>, AppError> {
        DefaultAccountRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_key(
        db: &DatabaseConnection,
        key: &str,
    ) -> Result<Option<default_account::Model>, AppError> {
        DefaultAccountRepo::find_by_key(db, key)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        key: &str,
        account_id: Option<String>,
    ) -> Result<default_account::Model, AppError> {
        let existing = DefaultAccountRepo::find_by_key(db, key)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| {
                AppError::NotFound(format!("Default account setting '{key}' not found"))
            })?;

        let mut model: default_account::ActiveModel = existing.into();
        model.account_id = Set(account_id);

        DefaultAccountRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn bulk_update(
        db: &DatabaseConnection,
        updates: Vec<(String, Option<String>)>,
    ) -> Result<Vec<default_account::Model>, AppError> {
        let mut results = Vec::new();
        for (key, account_id) in updates {
            let updated = Self::update(db, &key, account_id).await?;
            results.push(updated);
        }
        Ok(results)
    }
}
