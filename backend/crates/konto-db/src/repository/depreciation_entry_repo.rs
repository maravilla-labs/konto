use sea_orm::*;

use crate::entities::depreciation_entry::{self, Entity as DepreciationEntryEntity};

pub struct DepreciationEntryRepo;

impl DepreciationEntryRepo {
    pub async fn find_by_asset_id(
        db: &DatabaseConnection,
        asset_id: &str,
    ) -> Result<Vec<depreciation_entry::Model>, DbErr> {
        DepreciationEntryEntity::find()
            .filter(depreciation_entry::Column::FixedAssetId.eq(asset_id))
            .order_by_asc(depreciation_entry::Column::PeriodDate)
            .all(db)
            .await
    }

    pub async fn find_by_fiscal_year(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
    ) -> Result<Vec<depreciation_entry::Model>, DbErr> {
        DepreciationEntryEntity::find()
            .filter(depreciation_entry::Column::FiscalYearId.eq(fiscal_year_id))
            .order_by_asc(depreciation_entry::Column::PeriodDate)
            .all(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: depreciation_entry::ActiveModel,
    ) -> Result<depreciation_entry::Model, DbErr> {
        model.insert(db).await
    }
}
