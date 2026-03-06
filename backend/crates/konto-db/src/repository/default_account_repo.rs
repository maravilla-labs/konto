use sea_orm::*;

use crate::entities::default_account::{self, Entity as DefaultAccountEntity};

pub struct DefaultAccountRepo;

impl DefaultAccountRepo {
    pub async fn find_all(
        db: &DatabaseConnection,
    ) -> Result<Vec<default_account::Model>, DbErr> {
        DefaultAccountEntity::find()
            .order_by_asc(default_account::Column::SettingKey)
            .all(db)
            .await
    }

    pub async fn find_by_key(
        db: &DatabaseConnection,
        key: &str,
    ) -> Result<Option<default_account::Model>, DbErr> {
        DefaultAccountEntity::find()
            .filter(default_account::Column::SettingKey.eq(key))
            .one(db)
            .await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: default_account::ActiveModel,
    ) -> Result<default_account::Model, DbErr> {
        model.update(db).await
    }
}
