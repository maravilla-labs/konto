use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::{InvoiceStatus, ProjectStatus as ProjectStatusEnum};
use konto_db::entities::{project, time_entry, invoice, document};
use konto_db::repository::contact_repo::ContactRepo;
use konto_db::repository::project_repo::ProjectRepo;
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use super::language::normalize_language;

pub struct ProjectSummary {
    pub project: project::Model,
    pub contact_name: Option<String>,
    pub total_hours: Decimal,
    pub billable_hours: Decimal,
    pub budget_hours_remaining: Option<Decimal>,
    pub total_invoiced: Decimal,
}

pub struct ProjectService;

impl ProjectService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        search: Option<&str>,
    ) -> Result<(Vec<project::Model>, u64), AppError> {
        ProjectRepo::find_paginated(db, page, per_page, search)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<project::Model, AppError> {
        ProjectRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))
    }

    pub async fn get_summary(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<ProjectSummary, AppError> {
        let proj = Self::get_by_id(db, id).await?;

        let contact_name = if let Some(ref cid) = proj.contact_id {
            ContactRepo::find_by_id(db, cid)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .map(|c| c.name1)
        } else {
            None
        };

        // Sum time entries
        let entries = time_entry::Entity::find()
            .filter(time_entry::Column::ProjectId.eq(id))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let total_minutes: i32 = entries.iter().map(|e| e.actual_minutes).sum();
        let total_hours = Decimal::from(total_minutes) / Decimal::from(60);

        let billable_minutes: i32 = entries
            .iter()
            .filter(|e| e.billable)
            .map(|e| e.actual_minutes)
            .sum();
        let billable_hours = Decimal::from(billable_minutes) / Decimal::from(60);

        let budget_hours_remaining = proj.budget_hours.map(|bh| bh - total_hours);

        // Sum invoices linked to this project
        let invoices = invoice::Entity::find()
            .filter(invoice::Column::ProjectId.eq(id))
            .filter(
                Condition::any()
                    .add(invoice::Column::Status.eq(InvoiceStatus::Sent.as_str()))
                    .add(invoice::Column::Status.eq(InvoiceStatus::Paid.as_str()))
                    .add(invoice::Column::Status.eq(InvoiceStatus::Overdue.as_str())),
            )
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let total_invoiced: Decimal = invoices.iter().map(|i| i.total).sum();

        Ok(ProjectSummary {
            project: proj,
            contact_name,
            total_hours,
            billable_hours,
            budget_hours_remaining,
            total_invoiced,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        name: &str,
        number: Option<String>,
        contact_id: Option<String>,
        language: Option<String>,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
        description: Option<String>,
        budget_hours: Option<Decimal>,
        budget_amount: Option<Decimal>,
        hourly_rate: Option<Decimal>,
        soft_budget_hours: Option<Decimal>,
        hard_budget_hours: Option<Decimal>,
        soft_budget_amount: Option<Decimal>,
        hard_budget_amount: Option<Decimal>,
        contact_person_id: Option<String>,
        invoicing_method: Option<String>,
        currency: Option<String>,
        rounding_method: Option<String>,
        rounding_factor_minutes: Option<i32>,
        flat_rate_total: Option<Decimal>,
        owner_id: Option<String>,
    ) -> Result<project::Model, AppError> {
        let now = Utc::now().naive_utc();
        let resolved_language = if let Some(lang) = normalize_language(language.as_deref()) {
            Some(lang)
        } else if let Some(ref cid) = contact_id {
            ContactRepo::find_by_id(db, cid)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .and_then(|c| normalize_language(c.language.as_deref()))
        } else {
            None
        };

        // Auto-assign number if settings enabled and no number provided
        let resolved_number = if number.is_some() {
            number
        } else {
            Self::auto_assign_number(db).await?
        };

        let model = project::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            number: Set(resolved_number),
            name: Set(name.to_string()),
            contact_id: Set(contact_id),
            contact_person_name: Set(None),
            language: Set(resolved_language),
            start_date: Set(start_date),
            end_date: Set(end_date),
            status: Set(ProjectStatusEnum::Active.to_string()),
            description: Set(description),
            project_type: Set(None),
            budget_hours: Set(budget_hours),
            budget_amount: Set(budget_amount),
            hourly_rate: Set(hourly_rate),
            soft_budget_hours: Set(soft_budget_hours),
            hard_budget_hours: Set(hard_budget_hours),
            soft_budget_amount: Set(soft_budget_amount),
            hard_budget_amount: Set(hard_budget_amount),
            contact_person_id: Set(contact_person_id),
            invoicing_method: Set(invoicing_method.unwrap_or_else(|| "hourly".to_string())),
            currency: Set(currency.unwrap_or_else(|| "CHF".to_string())),
            rounding_method: Set(rounding_method),
            rounding_factor_minutes: Set(rounding_factor_minutes),
            flat_rate_total: Set(flat_rate_total),
            sub_status_id: Set(None),
            owner_id: Set(owner_id),
            bexio_id: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        ProjectRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn auto_assign_number(
        db: &DatabaseConnection,
    ) -> Result<Option<String>, AppError> {
        use konto_db::entities::company_setting;

        let settings = company_setting::Entity::find()
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let settings = match settings {
            Some(s) if s.project_number_auto => s,
            _ => return Ok(None),
        };

        // Find highest existing number with this prefix
        let all_projects = project::Entity::find()
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let prefix = &settings.project_number_prefix;
        let year = chrono::Utc::now().format("%Y").to_string();
        let year_prefix = if settings.project_number_restart_yearly {
            format!("{}{}-", prefix, year)
        } else {
            prefix.clone()
        };

        let max_num = all_projects
            .iter()
            .filter_map(|p| {
                p.number.as_ref().and_then(|n| {
                    n.strip_prefix(&year_prefix)
                        .and_then(|s| s.parse::<i32>().ok())
                })
            })
            .max()
            .unwrap_or(settings.project_number_start - 1);

        let next = max_num + 1;
        let min_len = settings.project_number_min_length as usize;
        let number_str = format!("{:0>width$}", next, width = min_len);

        Ok(Some(format!("{}{}", year_prefix, number_str)))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: Option<String>,
        status: Option<String>,
        contact_id: Option<Option<String>>,
        number: Option<Option<String>>,
        start_date: Option<Option<chrono::NaiveDate>>,
        end_date: Option<Option<chrono::NaiveDate>>,
        language: Option<Option<String>>,
        description: Option<Option<String>>,
        budget_hours: Option<Option<Decimal>>,
        budget_amount: Option<Option<Decimal>>,
        hourly_rate: Option<Option<Decimal>>,
        soft_budget_hours: Option<Option<Decimal>>,
        hard_budget_hours: Option<Option<Decimal>>,
        soft_budget_amount: Option<Option<Decimal>>,
        hard_budget_amount: Option<Option<Decimal>>,
        contact_person_id: Option<Option<String>>,
        invoicing_method: Option<String>,
        currency: Option<String>,
        rounding_method: Option<Option<String>>,
        rounding_factor_minutes: Option<Option<i32>>,
        flat_rate_total: Option<Option<Decimal>>,
        sub_status_id: Option<Option<String>>,
        owner_id: Option<Option<String>>,
    ) -> Result<project::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let mut model: project::ActiveModel = existing.into();

        if let Some(n) = name { model.name = Set(n); }
        if let Some(s) = status { model.status = Set(s); }
        if let Some(cid) = contact_id { model.contact_id = Set(cid); }
        if let Some(num) = number { model.number = Set(num); }
        if let Some(sd) = start_date { model.start_date = Set(sd); }
        if let Some(ed) = end_date { model.end_date = Set(ed); }
        if let Some(lang) = language {
            model.language = Set(normalize_language(lang.as_deref()));
        }
        if let Some(d) = description { model.description = Set(d); }
        if let Some(bh) = budget_hours { model.budget_hours = Set(bh); }
        if let Some(ba) = budget_amount { model.budget_amount = Set(ba); }
        if let Some(hr) = hourly_rate { model.hourly_rate = Set(hr); }
        if let Some(sbh) = soft_budget_hours { model.soft_budget_hours = Set(sbh); }
        if let Some(hbh) = hard_budget_hours { model.hard_budget_hours = Set(hbh); }
        if let Some(sba) = soft_budget_amount { model.soft_budget_amount = Set(sba); }
        if let Some(hba) = hard_budget_amount { model.hard_budget_amount = Set(hba); }
        if let Some(cpid) = contact_person_id { model.contact_person_id = Set(cpid); }
        if let Some(im) = invoicing_method { model.invoicing_method = Set(im); }
        if let Some(c) = currency { model.currency = Set(c); }
        if let Some(rm) = rounding_method { model.rounding_method = Set(rm); }
        if let Some(rf) = rounding_factor_minutes { model.rounding_factor_minutes = Set(rf); }
        if let Some(frt) = flat_rate_total { model.flat_rate_total = Set(frt); }
        if let Some(ssid) = sub_status_id { model.sub_status_id = Set(ssid); }
        if let Some(oid) = owner_id { model.owner_id = Set(oid); }
        model.updated_at = Set(Utc::now().naive_utc());

        ProjectRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        Self::get_by_id(db, id).await?;

        // Check for linked time entries
        let entries = time_entry::Entity::find()
            .filter(time_entry::Column::ProjectId.eq(id))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        if !entries.is_empty() {
            return Err(AppError::Conflict(
                "Cannot delete project with linked time entries".to_string(),
            ));
        }

        // Check for linked invoices
        let invoices = invoice::Entity::find()
            .filter(invoice::Column::ProjectId.eq(id))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        if !invoices.is_empty() {
            return Err(AppError::Conflict(
                "Cannot delete project with linked invoices".to_string(),
            ));
        }

        // Check for linked documents
        let docs = document::Entity::find()
            .filter(document::Column::ProjectId.eq(id))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        if !docs.is_empty() {
            return Err(AppError::Conflict(
                "Cannot delete project with linked documents".to_string(),
            ));
        }

        ProjectRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
