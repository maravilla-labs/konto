use sea_orm::*;

use crate::entities::rate_function::{self, ActiveModel, Entity as RateFunctionEntity};

pub struct RateFunctionRepo;

impl RateFunctionRepo {
    pub async fn find_all_sorted(
        db: &DatabaseConnection,
    ) -> Result<Vec<rate_function::Model>, DbErr> {
        RateFunctionEntity::find()
            .order_by_asc(rate_function::Column::SortOrder)
            .all(db)
            .await
    }

    pub async fn find_active(
        db: &DatabaseConnection,
    ) -> Result<Vec<rate_function::Model>, DbErr> {
        RateFunctionEntity::find()
            .filter(rate_function::Column::IsActive.eq(true))
            .order_by_asc(rate_function::Column::SortOrder)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<rate_function::Model>, DbErr> {
        RateFunctionEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<rate_function::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<rate_function::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        RateFunctionEntity::delete_by_id(id).exec(db).await
    }
}
