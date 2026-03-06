use sea_orm::*;
use konto_common::enums::FixedAssetStatus;

use crate::entities::fixed_asset::{self, Entity as FixedAssetEntity};

pub struct FixedAssetRepo;

impl FixedAssetRepo {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<fixed_asset::Model>, DbErr> {
        FixedAssetEntity::find()
            .order_by_asc(fixed_asset::Column::Name)
            .all(db)
            .await
    }

    pub async fn find_active(db: &DatabaseConnection) -> Result<Vec<fixed_asset::Model>, DbErr> {
        FixedAssetEntity::find()
            .filter(fixed_asset::Column::Status.eq(FixedAssetStatus::Active.as_str()))
            .order_by_asc(fixed_asset::Column::Name)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<fixed_asset::Model>, DbErr> {
        FixedAssetEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: fixed_asset::ActiveModel,
    ) -> Result<fixed_asset::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: fixed_asset::ActiveModel,
    ) -> Result<fixed_asset::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), DbErr> {
        FixedAssetEntity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
