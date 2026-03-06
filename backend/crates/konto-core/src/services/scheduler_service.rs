use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::InvoiceStatus;
use konto_db::entities::invoice;
use konto_db::repository::invoice_repo::InvoiceRepo;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set};

use super::audit_service::AuditService;

pub struct SchedulerService;

impl SchedulerService {
    /// Find all sent invoices past their due date and mark them overdue.
    pub async fn check_overdue(db: &DatabaseConnection) -> Result<u64, AppError> {
        let today = Utc::now().naive_utc().date();
        let overdue_invoices = invoice::Entity::find()
            .filter(
                Condition::all()
                    .add(invoice::Column::Status.eq(InvoiceStatus::Sent.as_str()))
                    .add(invoice::Column::DueDate.lt(today)),
            )
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let count = overdue_invoices.len() as u64;

        for inv in overdue_invoices {
            let inv_id = inv.id.clone();
            let mut model: invoice::ActiveModel = inv.into();
            model.status = Set(InvoiceStatus::Overdue.to_string());
            model.updated_at = Set(Utc::now().naive_utc());

            InvoiceRepo::update(db, model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

            AuditService::log(
                db,
                None,
                InvoiceStatus::Overdue.as_str(),
                "invoice",
                Some(&inv_id),
                Some("{\"status\":\"sent\"}"),
                Some("{\"status\":\"overdue\"}"),
            )
            .await?;
        }

        Ok(count)
    }
}
