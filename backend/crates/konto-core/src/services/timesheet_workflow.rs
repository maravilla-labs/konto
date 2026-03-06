use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::TimesheetStatus;
use konto_db::entities::timesheet;
use konto_db::repository::timesheet_repo::TimesheetRepo;
use sea_orm::{DatabaseConnection, Set};

use super::timesheet_service::TimesheetService;

/// Submit a draft timesheet for approval.
pub async fn submit(
    db: &DatabaseConnection,
    id: &str,
) -> Result<timesheet::Model, AppError> {
    let ts = TimesheetService::get_by_id(db, id).await?;
    if ts.status != TimesheetStatus::Draft.as_str() {
        return Err(AppError::Validation(
            "Only draft timesheets can be submitted".to_string(),
        ));
    }

    let now = Utc::now().naive_utc();
    let mut model: timesheet::ActiveModel = ts.into();
    model.status = Set(TimesheetStatus::Submitted.to_string());
    model.submitted_at = Set(Some(now));
    model.updated_at = Set(now);

    TimesheetRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

/// Approve a submitted timesheet.
pub async fn approve(
    db: &DatabaseConnection,
    id: &str,
    approver_user_id: &str,
) -> Result<timesheet::Model, AppError> {
    let ts = TimesheetService::get_by_id(db, id).await?;
    if ts.status != TimesheetStatus::Submitted.as_str() {
        return Err(AppError::Validation(
            "Only submitted timesheets can be approved".to_string(),
        ));
    }

    let now = Utc::now().naive_utc();
    let mut model: timesheet::ActiveModel = ts.into();
    model.status = Set(TimesheetStatus::Approved.to_string());
    model.approved_by = Set(Some(approver_user_id.to_string()));
    model.approved_at = Set(Some(now));
    model.updated_at = Set(now);

    TimesheetRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

/// Reject a submitted timesheet back to draft.
pub async fn reject(
    db: &DatabaseConnection,
    id: &str,
) -> Result<timesheet::Model, AppError> {
    let ts = TimesheetService::get_by_id(db, id).await?;
    if ts.status != TimesheetStatus::Submitted.as_str() {
        return Err(AppError::Validation(
            "Only submitted timesheets can be rejected".to_string(),
        ));
    }

    let now = Utc::now().naive_utc();
    let mut model: timesheet::ActiveModel = ts.into();
    model.status = Set(TimesheetStatus::Draft.to_string());
    model.submitted_at = Set(None);
    model.updated_at = Set(now);

    TimesheetRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

/// Lock an approved timesheet.
pub async fn lock(
    db: &DatabaseConnection,
    id: &str,
) -> Result<timesheet::Model, AppError> {
    let ts = TimesheetService::get_by_id(db, id).await?;
    if ts.status != TimesheetStatus::Approved.as_str() {
        return Err(AppError::Validation(
            "Only approved timesheets can be locked".to_string(),
        ));
    }

    let now = Utc::now().naive_utc();
    let mut model: timesheet::ActiveModel = ts.into();
    model.status = Set(TimesheetStatus::Locked.to_string());
    model.updated_at = Set(now);

    TimesheetRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}
