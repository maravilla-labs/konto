use sea_orm::*;
use crate::entities::expense_line;

pub struct ExpenseLineRepo;

impl ExpenseLineRepo {
    pub async fn find_by_expense(
        db: &DatabaseConnection,
        expense_id: &str,
    ) -> Result<Vec<expense_line::Model>, DbErr> {
        expense_line::Entity::find()
            .filter(expense_line::Column::ExpenseId.eq(expense_id))
            .order_by_asc(expense_line::Column::Position)
            .all(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: expense_line::ActiveModel,
    ) -> Result<expense_line::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn delete_by_expense(db: &DatabaseConnection, expense_id: &str) -> Result<(), DbErr> {
        expense_line::Entity::delete_many()
            .filter(expense_line::Column::ExpenseId.eq(expense_id))
            .exec(db)
            .await?;
        Ok(())
    }
}
