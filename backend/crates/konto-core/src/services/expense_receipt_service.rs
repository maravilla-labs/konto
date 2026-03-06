use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::expense_receipt;
use konto_db::repository::expense_receipt_repo::ExpenseReceiptRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::storage::StorageService;

pub struct ExpenseReceiptService;

impl ExpenseReceiptService {
    pub async fn list_by_expense(
        db: &DatabaseConnection,
        expense_id: &str,
    ) -> Result<Vec<expense_receipt::Model>, AppError> {
        ExpenseReceiptRepo::find_by_expense(db, expense_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn upload(
        db: &DatabaseConnection,
        storage: &dyn StorageService,
        expense_id: &str,
        file_name: &str,
        data: &[u8],
        mime_type: &str,
    ) -> Result<expense_receipt::Model, AppError> {
        let id = Uuid::new_v4().to_string();
        let ext = file_name.rsplit('.').next().unwrap_or("bin");
        let storage_key = format!("expense-receipts/{expense_id}/{id}.{ext}");

        storage.upload(&storage_key, data, mime_type).await?;

        let model = expense_receipt::ActiveModel {
            id: Set(id),
            expense_id: Set(expense_id.to_string()),
            line_id: Set(None),
            file_name: Set(file_name.to_string()),
            storage_key: Set(storage_key),
            file_size: Set(data.len() as i64),
            mime_type: Set(mime_type.to_string()),
            uploaded_at: Set(Utc::now()),
        };

        ExpenseReceiptRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn download(
        db: &DatabaseConnection,
        storage: &dyn StorageService,
        id: &str,
    ) -> Result<(expense_receipt::Model, Vec<u8>), AppError> {
        let receipt = ExpenseReceiptRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Receipt not found".to_string()))?;

        let data = storage.download(&receipt.storage_key).await?;
        Ok((receipt, data))
    }

    pub async fn delete(
        db: &DatabaseConnection,
        storage: &dyn StorageService,
        id: &str,
    ) -> Result<(), AppError> {
        let receipt = ExpenseReceiptRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Receipt not found".to_string()))?;

        storage.delete(&receipt.storage_key).await?;

        ExpenseReceiptRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
