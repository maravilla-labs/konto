use sea_orm::*;
use sea_orm::prelude::Expr;

use crate::entities::bank_account::{self, Column, Entity as BankAccountEntity};

pub struct BankAccountRepo;

impl BankAccountRepo {
    pub async fn list_all(
        db: &DatabaseConnection,
    ) -> Result<Vec<bank_account::Model>, DbErr> {
        BankAccountEntity::find()
            .order_by_asc(Column::Name)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<bank_account::Model>, DbErr> {
        BankAccountEntity::find_by_id(id).one(db).await
    }

    pub async fn find_default(
        db: &DatabaseConnection,
    ) -> Result<Option<bank_account::Model>, DbErr> {
        BankAccountEntity::find()
            .filter(Column::IsDefault.eq(true))
            .one(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: bank_account::ActiveModel,
    ) -> Result<bank_account::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: bank_account::ActiveModel,
    ) -> Result<bank_account::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        BankAccountEntity::delete_by_id(id).exec(db).await
    }

    /// Clear is_default on all bank accounts (used before setting a new default).
    pub async fn clear_defaults(db: &DatabaseConnection) -> Result<(), DbErr> {
        BankAccountEntity::update_many()
            .col_expr(Column::IsDefault, Expr::value(false))
            .filter(Column::IsDefault.eq(true))
            .exec(db)
            .await?;
        Ok(())
    }
}
