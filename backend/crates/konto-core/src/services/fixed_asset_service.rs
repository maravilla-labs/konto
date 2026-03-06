use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::FixedAssetStatus;
use konto_db::entities::fixed_asset;
use konto_db::repository::fixed_asset_repo::FixedAssetRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct FixedAssetService;

impl FixedAssetService {
    pub async fn list(db: &DatabaseConnection) -> Result<Vec<fixed_asset::Model>, AppError> {
        FixedAssetRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<fixed_asset::Model, AppError> {
        FixedAssetRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Fixed asset not found".into()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        name: &str,
        description: Option<String>,
        account_id: &str,
        depreciation_account_id: &str,
        acquisition_date: chrono::NaiveDate,
        acquisition_cost: Decimal,
        residual_value: Decimal,
        useful_life_years: i32,
        depreciation_method: &str,
        declining_rate: Option<Decimal>,
    ) -> Result<fixed_asset::Model, AppError> {
        if useful_life_years <= 0 {
            return Err(AppError::Validation("Useful life must be positive".into()));
        }
        if acquisition_cost < Decimal::ZERO {
            return Err(AppError::Validation("Acquisition cost cannot be negative".into()));
        }
        if residual_value < Decimal::ZERO {
            return Err(AppError::Validation("Residual value cannot be negative".into()));
        }
        if residual_value > acquisition_cost {
            return Err(AppError::Validation("Residual value cannot exceed acquisition cost".into()));
        }
        if depreciation_method == "declining_balance" && declining_rate.is_none() {
            return Err(AppError::Validation("Declining rate is required for declining balance method".into()));
        }

        let now = Utc::now().naive_utc();
        let model = fixed_asset::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(name.to_string()),
            description: Set(description),
            account_id: Set(account_id.to_string()),
            depreciation_account_id: Set(depreciation_account_id.to_string()),
            acquisition_date: Set(acquisition_date),
            acquisition_cost: Set(acquisition_cost),
            residual_value: Set(residual_value),
            useful_life_years: Set(useful_life_years),
            depreciation_method: Set(depreciation_method.to_string()),
            declining_rate: Set(declining_rate),
            status: Set(FixedAssetStatus::Active.to_string()),
            disposed_date: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        FixedAssetRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: &str,
        description: Option<String>,
        account_id: &str,
        depreciation_account_id: &str,
        acquisition_date: chrono::NaiveDate,
        acquisition_cost: Decimal,
        residual_value: Decimal,
        useful_life_years: i32,
        depreciation_method: &str,
        declining_rate: Option<Decimal>,
        status: &str,
        disposed_date: Option<chrono::NaiveDate>,
    ) -> Result<fixed_asset::Model, AppError> {
        let existing = FixedAssetRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Fixed asset not found".into()))?;

        let mut model: fixed_asset::ActiveModel = existing.into();
        model.name = Set(name.to_string());
        model.description = Set(description);
        model.account_id = Set(account_id.to_string());
        model.depreciation_account_id = Set(depreciation_account_id.to_string());
        model.acquisition_date = Set(acquisition_date);
        model.acquisition_cost = Set(acquisition_cost);
        model.residual_value = Set(residual_value);
        model.useful_life_years = Set(useful_life_years);
        model.depreciation_method = Set(depreciation_method.to_string());
        model.declining_rate = Set(declining_rate);
        model.status = Set(status.to_string());
        model.disposed_date = Set(disposed_date);
        model.updated_at = Set(Utc::now().naive_utc());

        FixedAssetRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        FixedAssetRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Fixed asset not found".into()))?;

        FixedAssetRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}
