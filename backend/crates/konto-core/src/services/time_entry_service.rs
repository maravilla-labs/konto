use chrono::{NaiveDate, Utc};
use konto_common::error::AppError;
use konto_common::enums::TimeEntryStatus;
use konto_db::entities::time_entry;
use konto_db::repository::project_repo::TimeEntryRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct TimeEntryService;

impl TimeEntryService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        project_id: Option<&str>,
        billed: Option<bool>,
        status: Option<&str>,
        billable: Option<bool>,
    ) -> Result<(Vec<time_entry::Model>, u64), AppError> {
        TimeEntryRepo::find_paginated(db, page, per_page, project_id, billed, status, billable)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<time_entry::Model, AppError> {
        TimeEntryRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Time entry not found".to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        project_id: Option<String>,
        contact_id: Option<String>,
        user_id: Option<String>,
        activity_type_id: Option<String>,
        date: NaiveDate,
        actual_minutes: i32,
        estimated_minutes: Option<i32>,
        description: Option<String>,
        flat_amount: Option<Decimal>,
        travel_minutes: Option<i32>,
        travel_flat_rate: Option<Decimal>,
        travel_distance: Option<Decimal>,
        task_id: Option<String>,
        quantity: Option<Decimal>,
        billable: bool,
        start_time: Option<String>,
        end_time: Option<String>,
    ) -> Result<time_entry::Model, AppError> {
        let now = Utc::now().naive_utc();

        let model = time_entry::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            project_id: Set(project_id),
            contact_id: Set(contact_id),
            user_id: Set(user_id),
            activity_type_id: Set(activity_type_id),
            date: Set(date),
            actual_minutes: Set(actual_minutes),
            estimated_minutes: Set(estimated_minutes),
            description: Set(description),
            flat_amount: Set(flat_amount),
            travel_minutes: Set(travel_minutes),
            travel_flat_rate: Set(travel_flat_rate),
            travel_distance: Set(travel_distance),
            quantity: Set(quantity),
            task_id: Set(task_id),
            timesheet_id: Set(None),
            status: Set(TimeEntryStatus::Pending.to_string()),
            billed: Set(false),
            billable: Set(billable),
            start_time: Set(start_time),
            end_time: Set(end_time),
            bexio_id: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        TimeEntryRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        project_id: Option<Option<String>>,
        contact_id: Option<Option<String>>,
        activity_type_id: Option<Option<String>>,
        date: Option<NaiveDate>,
        actual_minutes: Option<i32>,
        estimated_minutes: Option<Option<i32>>,
        description: Option<Option<String>>,
        flat_amount: Option<Option<Decimal>>,
        travel_minutes: Option<Option<i32>>,
        task_id: Option<Option<String>>,
        quantity: Option<Option<Decimal>>,
        billable: Option<bool>,
        start_time: Option<Option<String>>,
        end_time: Option<Option<String>>,
    ) -> Result<time_entry::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let mut model: time_entry::ActiveModel = existing.into();

        if let Some(pid) = project_id { model.project_id = Set(pid); }
        if let Some(cid) = contact_id { model.contact_id = Set(cid); }
        if let Some(aid) = activity_type_id { model.activity_type_id = Set(aid); }
        if let Some(d) = date { model.date = Set(d); }
        if let Some(m) = actual_minutes { model.actual_minutes = Set(m); }
        if let Some(em) = estimated_minutes { model.estimated_minutes = Set(em); }
        if let Some(desc) = description { model.description = Set(desc); }
        if let Some(fa) = flat_amount { model.flat_amount = Set(fa); }
        if let Some(tm) = travel_minutes { model.travel_minutes = Set(tm); }
        if let Some(tid) = task_id { model.task_id = Set(tid); }
        if let Some(q) = quantity { model.quantity = Set(q); }
        if let Some(b) = billable { model.billable = Set(b); }
        if let Some(st) = start_time { model.start_time = Set(st); }
        if let Some(et) = end_time { model.end_time = Set(et); }
        model.updated_at = Set(Utc::now().naive_utc());

        TimeEntryRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn mark_billed(
        db: &DatabaseConnection,
        ids: &[String],
    ) -> Result<(), AppError> {
        for id in ids {
            let entry = Self::get_by_id(db, id).await?;
            let mut model: time_entry::ActiveModel = entry.into();
            model.billed = Set(true);
            model.updated_at = Set(Utc::now().naive_utc());
            TimeEntryRepo::update(db, model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        Self::get_by_id(db, id).await?;
        TimeEntryRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
