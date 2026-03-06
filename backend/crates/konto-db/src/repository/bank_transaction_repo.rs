use sea_orm::*;
use konto_common::enums::BankTransactionStatus;

use crate::entities::bank_transaction::{self, Entity as BankTransactionEntity};

pub struct BankTransactionRepo;

impl BankTransactionRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        bank_account_id: Option<&str>,
        status_filter: Option<&str>,
    ) -> Result<(Vec<bank_transaction::Model>, u64), DbErr> {
        let mut query = BankTransactionEntity::find()
            .order_by_desc(bank_transaction::Column::TransactionDate);

        if let Some(acct) = bank_account_id {
            query = query.filter(bank_transaction::Column::BankAccountId.eq(acct));
        }
        if let Some(status) = status_filter {
            query = query.filter(bank_transaction::Column::Status.eq(status));
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<bank_transaction::Model>, DbErr> {
        BankTransactionEntity::find_by_id(id).one(db).await
    }

    pub async fn find_unmatched_by_account(
        db: &DatabaseConnection,
        bank_account_id: &str,
    ) -> Result<Vec<bank_transaction::Model>, DbErr> {
        BankTransactionEntity::find()
            .filter(bank_transaction::Column::BankAccountId.eq(bank_account_id))
            .filter(bank_transaction::Column::Status.eq(BankTransactionStatus::Unmatched.as_str()))
            .order_by_asc(bank_transaction::Column::TransactionDate)
            .all(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: bank_transaction::ActiveModel,
    ) -> Result<bank_transaction::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: bank_transaction::ActiveModel,
    ) -> Result<bank_transaction::Model, DbErr> {
        model.update(db).await
    }
}
