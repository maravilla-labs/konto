use sea_orm::*;

use crate::entities::currency::{self, Entity as CurrencyEntity};

pub struct CurrencyRepo;

impl CurrencyRepo {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<currency::Model>, DbErr> {
        CurrencyEntity::find()
            .order_by_asc(currency::Column::Code)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<currency::Model>, DbErr> {
        CurrencyEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: currency::ActiveModel,
    ) -> Result<currency::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: currency::ActiveModel,
    ) -> Result<currency::Model, DbErr> {
        model.update(db).await
    }
}
