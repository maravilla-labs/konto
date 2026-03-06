use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::bank_account;
use konto_db::repository::bank_account_repo::BankAccountRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct BankAccountService;

impl BankAccountService {
    pub async fn list(
        db: &DatabaseConnection,
    ) -> Result<Vec<bank_account::Model>, AppError> {
        BankAccountRepo::list_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<bank_account::Model, AppError> {
        BankAccountRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Bank account not found".to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        name: &str,
        bank_name: &str,
        iban: &str,
        bic: Option<String>,
        currency_id: Option<String>,
        account_id: Option<String>,
        qr_iban: Option<String>,
        is_default: bool,
    ) -> Result<bank_account::Model, AppError> {
        // If marking as default, clear existing defaults first
        if is_default {
            BankAccountRepo::clear_defaults(db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        let now = Utc::now().naive_utc();
        let model = bank_account::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(name.to_string()),
            bank_name: Set(bank_name.to_string()),
            iban: Set(iban.to_string()),
            bic: Set(bic),
            currency_id: Set(currency_id),
            account_id: Set(account_id),
            qr_iban: Set(qr_iban),
            is_default: Set(is_default),
            created_at: Set(now),
            updated_at: Set(now),
        };

        BankAccountRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: &str,
        bank_name: &str,
        iban: &str,
        bic: Option<String>,
        currency_id: Option<String>,
        account_id: Option<String>,
        qr_iban: Option<String>,
        is_default: bool,
    ) -> Result<bank_account::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;

        if is_default {
            BankAccountRepo::clear_defaults(db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        let now = Utc::now().naive_utc();
        let mut model: bank_account::ActiveModel = existing.into();
        model.name = Set(name.to_string());
        model.bank_name = Set(bank_name.to_string());
        model.iban = Set(iban.to_string());
        model.bic = Set(bic);
        model.currency_id = Set(currency_id);
        model.account_id = Set(account_id);
        model.qr_iban = Set(qr_iban);
        model.is_default = Set(is_default);
        model.updated_at = Set(now);

        BankAccountRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        // Verify existence
        Self::get_by_id(db, id).await?;

        BankAccountRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
