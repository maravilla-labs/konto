use chrono::{NaiveDate, Utc};
use konto_common::error::AppError;
use konto_common::enums::TimesheetStatus;
use konto_db::entities::timesheet;
use konto_db::repository::timesheet_repo::TimesheetRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct TimesheetService;

impl TimesheetService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        user_id: Option<&str>,
    ) -> Result<(Vec<timesheet::Model>, u64), AppError> {
        TimesheetRepo::find_paginated(db, page, per_page, user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<timesheet::Model, AppError> {
        TimesheetRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Timesheet not found".to_string()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        user_id: &str,
        period_start: NaiveDate,
        period_end: NaiveDate,
        notes: Option<String>,
    ) -> Result<timesheet::Model, AppError> {
        if period_end < period_start {
            return Err(AppError::Validation(
                "Period end must be on or after period start".to_string(),
            ));
        }

        let now = Utc::now().naive_utc();

        let model = timesheet::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            user_id: Set(user_id.to_string()),
            period_start: Set(period_start),
            period_end: Set(period_end),
            status: Set(TimesheetStatus::Draft.to_string()),
            submitted_at: Set(None),
            approved_by: Set(None),
            approved_at: Set(None),
            notes: Set(notes),
            created_at: Set(now),
            updated_at: Set(now),
        };

        TimesheetRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        notes: Option<Option<String>>,
        period_start: Option<NaiveDate>,
        period_end: Option<NaiveDate>,
    ) -> Result<timesheet::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        if existing.status != TimesheetStatus::Draft.as_str() {
            return Err(AppError::Validation(
                "Only draft timesheets can be updated".to_string(),
            ));
        }

        let mut model: timesheet::ActiveModel = existing.into();

        if let Some(n) = notes {
            model.notes = Set(n);
        }
        if let Some(ps) = period_start {
            model.period_start = Set(ps);
        }
        if let Some(pe) = period_end {
            model.period_end = Set(pe);
        }
        model.updated_at = Set(Utc::now().naive_utc());

        TimesheetRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        let existing = Self::get_by_id(db, id).await?;
        if existing.status != TimesheetStatus::Draft.as_str() {
            return Err(AppError::Validation(
                "Only draft timesheets can be deleted".to_string(),
            ));
        }

        TimesheetRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
