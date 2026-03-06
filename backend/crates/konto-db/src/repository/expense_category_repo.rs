use sea_orm::*;

use crate::entities::expense_category::{self, Entity as ExpenseCategoryEntity};

pub struct ExpenseCategoryRepo;

impl ExpenseCategoryRepo {
    pub async fn find_all(
        db: &DatabaseConnection,
    ) -> Result<Vec<expense_category::Model>, DbErr> {
        ExpenseCategoryEntity::find()
            .order_by_asc(expense_category::Column::Name)
            .all(db)
            .await
    }

    pub async fn find_active(
        db: &DatabaseConnection,
    ) -> Result<Vec<expense_category::Model>, DbErr> {
        ExpenseCategoryEntity::find()
            .filter(expense_category::Column::IsActive.eq(true))
            .order_by_asc(expense_category::Column::Name)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<expense_category::Model>, DbErr> {
        ExpenseCategoryEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: expense_category::ActiveModel,
    ) -> Result<expense_category::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: expense_category::ActiveModel,
    ) -> Result<expense_category::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        ExpenseCategoryEntity::delete_by_id(id).exec(db).await
    }
}
