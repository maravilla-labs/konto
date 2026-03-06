use sea_orm::*;

use crate::entities::shareholder::{self, ActiveModel, Entity as ShareholderEntity};

pub struct ShareholderRepo;

impl ShareholderRepo {
    pub async fn find_all_sorted(
        db: &DatabaseConnection,
    ) -> Result<Vec<shareholder::Model>, DbErr> {
        ShareholderEntity::find()
            .order_by_asc(shareholder::Column::SortOrder)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<shareholder::Model>, DbErr> {
        ShareholderEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<shareholder::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<shareholder::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        ShareholderEntity::delete_by_id(id).exec(db).await
    }
}
