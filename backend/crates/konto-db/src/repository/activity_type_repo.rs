use sea_orm::*;

use crate::entities::activity_type::{self, Entity as ActivityTypeEntity};

pub struct ActivityTypeRepo;

impl ActivityTypeRepo {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<activity_type::Model>, DbErr> {
        ActivityTypeEntity::find()
            .order_by_asc(activity_type::Column::Name)
            .all(db)
            .await
    }

    pub async fn find_active(
        db: &DatabaseConnection,
    ) -> Result<Vec<activity_type::Model>, DbErr> {
        ActivityTypeEntity::find()
            .filter(activity_type::Column::IsActive.eq(true))
            .order_by_asc(activity_type::Column::Name)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<activity_type::Model>, DbErr> {
        ActivityTypeEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: activity_type::ActiveModel,
    ) -> Result<activity_type::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: activity_type::ActiveModel,
    ) -> Result<activity_type::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        ActivityTypeEntity::delete_by_id(id).exec(db).await
    }
}
