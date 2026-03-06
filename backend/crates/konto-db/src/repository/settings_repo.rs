use sea_orm::*;

use crate::entities::company_setting::{self, Entity as CompanySettingEntity};

pub struct SettingsRepo;

impl SettingsRepo {
    /// Get the singleton company settings row (there should be at most one).
    pub async fn find(db: &DatabaseConnection) -> Result<Option<company_setting::Model>, DbErr> {
        CompanySettingEntity::find().one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: company_setting::ActiveModel,
    ) -> Result<company_setting::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: company_setting::ActiveModel,
    ) -> Result<company_setting::Model, DbErr> {
        model.update(db).await
    }
}
