use sea_orm::*;
use crate::entities::expense_receipt;

pub struct ExpenseReceiptRepo;

impl ExpenseReceiptRepo {
    pub async fn find_by_expense(
        db: &DatabaseConnection,
        expense_id: &str,
    ) -> Result<Vec<expense_receipt::Model>, DbErr> {
        expense_receipt::Entity::find()
            .filter(expense_receipt::Column::ExpenseId.eq(expense_id))
            .order_by_asc(expense_receipt::Column::UploadedAt)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<expense_receipt::Model>, DbErr> {
        expense_receipt::Entity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: expense_receipt::ActiveModel,
    ) -> Result<expense_receipt::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        expense_receipt::Entity::delete_by_id(id).exec(db).await
    }
}
