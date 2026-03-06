use sea_orm::*;

use crate::entities::email_setting::{self, Entity as EmailSettingEntity};

pub struct EmailSettingsRepo;

impl EmailSettingsRepo {
    pub async fn find_first(
        db: &DatabaseConnection,
    ) -> Result<Option<email_setting::Model>, DbErr> {
        EmailSettingEntity::find().one(db).await
    }

    pub async fn upsert(
        db: &DatabaseConnection,
        model: email_setting::ActiveModel,
    ) -> Result<email_setting::Model, DbErr> {
        let existing = EmailSettingEntity::find().one(db).await?;
        if existing.is_some() {
            model.update(db).await
        } else {
            model.insert(db).await
        }
    }
}
