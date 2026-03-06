use konto_common::error::AppError;
use konto_db::entities::currency;
use konto_db::repository::currency_repo::CurrencyRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct CurrencyService;

impl CurrencyService {
    pub async fn list(db: &DatabaseConnection) -> Result<Vec<currency::Model>, AppError> {
        CurrencyRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        code: &str,
        name: &str,
        symbol: &str,
    ) -> Result<currency::Model, AppError> {
        let model = currency::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            code: Set(code.to_string()),
            name: Set(name.to_string()),
            symbol: Set(symbol.to_string()),
            is_primary: Set(false),
        };
        CurrencyRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        code: &str,
        name: &str,
        symbol: &str,
    ) -> Result<currency::Model, AppError> {
        let existing = CurrencyRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Currency not found".into()))?;

        let mut model: currency::ActiveModel = existing.into();
        model.code = Set(code.to_string());
        model.name = Set(name.to_string());
        model.symbol = Set(symbol.to_string());

        CurrencyRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}
