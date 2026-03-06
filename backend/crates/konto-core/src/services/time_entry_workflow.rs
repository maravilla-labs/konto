use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::TimeEntryStatus;
use konto_db::entities::time_entry;
use konto_db::repository::project_repo::TimeEntryRepo;
use sea_orm::{DatabaseConnection, Set};

/// Valid status transitions for time entries:
/// pending -> in_progress -> done -> invoiced -> closed
/// Any status can go back to pending (reopen).
pub struct TimeEntryWorkflow;

impl TimeEntryWorkflow {
    const VALID_STATUSES: &'static [&'static str] =
        &["pending", "in_progress", "done", "invoiced", "closed"];

    pub fn validate_transition(current: &str, next: &str) -> Result<(), AppError> {
        if !Self::VALID_STATUSES.contains(&next) {
            return Err(AppError::Validation(format!(
                "Invalid status: '{}'. Valid: pending, in_progress, done, invoiced, closed",
                next
            )));
        }

        let allowed = match current {
            "pending" => vec!["in_progress", "done"],
            "in_progress" => vec!["pending", "done"],
            "done" => vec!["in_progress", "invoiced", "pending"],
            "invoiced" => vec!["closed", "done"],
            "closed" => vec!["done"],
            _ => vec!["pending"],
        };

        if !allowed.contains(&next) {
            return Err(AppError::Validation(format!(
                "Cannot transition from '{}' to '{}'. Allowed: {:?}",
                current, next, allowed
            )));
        }

        Ok(())
    }

    pub async fn transition(
        db: &DatabaseConnection,
        id: &str,
        next_status: &str,
    ) -> Result<time_entry::Model, AppError> {
        let entry = TimeEntryRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Time entry not found".to_string()))?;

        Self::validate_transition(&entry.status, next_status)?;

        let mut model: time_entry::ActiveModel = entry.into();
        model.status = Set(next_status.to_string());

        // Auto-set billed flag when transitioning to invoiced
        if next_status == TimeEntryStatus::Invoiced.as_str() {
            model.billed = Set(true);
        }

        model.updated_at = Set(Utc::now().naive_utc());

        TimeEntryRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Bulk transition time entries to invoiced status (used when creating invoice from time entries)
    pub async fn mark_invoiced(
        db: &DatabaseConnection,
        ids: &[String],
    ) -> Result<(), AppError> {
        for id in ids {
            let entry = TimeEntryRepo::find_by_id(db, id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .ok_or_else(|| AppError::NotFound(format!("Time entry {} not found", id)))?;

            // Only done + billable entries can be invoiced
            if entry.status != TimeEntryStatus::Done.as_str() || !entry.billable {
                return Err(AppError::Validation(format!(
                    "Time entry {} must be 'done' and billable to be invoiced (status: {}, billable: {})",
                    id, entry.status, entry.billable
                )));
            }

            let mut model: time_entry::ActiveModel = entry.into();
            model.status = Set(TimeEntryStatus::Invoiced.to_string());
            model.billed = Set(true);
            model.updated_at = Set(Utc::now().naive_utc());

            TimeEntryRepo::update(db, model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }
        Ok(())
    }
}
