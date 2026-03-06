use konto_common::error::AppError;
use konto_db::entities::vat_rate;
use konto_db::repository::vat_rate_repo::VatRateRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct VatRateService;

impl VatRateService {
    pub async fn list(db: &DatabaseConnection) -> Result<Vec<vat_rate::Model>, AppError> {
        VatRateRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        code: &str,
        name: &str,
        rate: Decimal,
        vat_type: &str,
        vat_category: Option<&str>,
        valid_from: Option<chrono::NaiveDate>,
        valid_to: Option<chrono::NaiveDate>,
    ) -> Result<vat_rate::Model, AppError> {
        let model = vat_rate::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            code: Set(code.to_string()),
            name: Set(name.to_string()),
            rate: Set(rate),
            vat_type: Set(vat_type.to_string()),
            vat_category: Set(vat_category.unwrap_or("standard").to_string()),
            is_active: Set(true),
            valid_from: Set(valid_from),
            valid_to: Set(valid_to),
        };
        VatRateRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        code: &str,
        name: &str,
        rate: Decimal,
        vat_type: &str,
        vat_category: Option<&str>,
        is_active: bool,
        valid_from: Option<chrono::NaiveDate>,
        valid_to: Option<chrono::NaiveDate>,
    ) -> Result<vat_rate::Model, AppError> {
        let existing = VatRateRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("VAT rate not found".into()))?;

        let mut model: vat_rate::ActiveModel = existing.into();
        model.code = Set(code.to_string());
        model.name = Set(name.to_string());
        model.rate = Set(rate);
        model.vat_type = Set(vat_type.to_string());
        if let Some(cat) = vat_category {
            model.vat_category = Set(cat.to_string());
        }
        model.is_active = Set(is_active);
        model.valid_from = Set(valid_from);
        model.valid_to = Set(valid_to);

        VatRateRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn deactivate(db: &DatabaseConnection, id: &str) -> Result<vat_rate::Model, AppError> {
        let existing = VatRateRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("VAT rate not found".into()))?;

        let mut model: vat_rate::ActiveModel = existing.into();
        model.is_active = Set(false);

        VatRateRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}
