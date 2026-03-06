use chrono::{Datelike, NaiveDate, Utc};
use konto_common::error::AppError;
use konto_db::entities::{company_setting, fiscal_period, fiscal_year, user};
use konto_db::repository::fiscal_year_repo::{FiscalPeriodRepo, FiscalYearRepo};
use konto_db::repository::settings_repo::SettingsRepo;
use konto_db::repository::user_repo::UserRepo;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use crate::auth::jwt::JwtService;
use crate::auth::password;
use crate::services::language::normalize_or_default;
use konto_common::enums::{FiscalYearStatus, FiscalPeriodStatus, UserRole};

pub struct SetupService;

pub struct SetupResult {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct SetupInput {
    pub admin_email: String,
    pub admin_password: String,
    pub admin_full_name: String,
    pub admin_language: String,
    pub legal_name: String,
    pub trade_name: Option<String>,
    pub street: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub legal_entity_type: Option<String>,
    pub default_currency: Option<String>,
    pub vat_method: Option<String>,
    pub flat_rate_percentage: Option<f64>,
    pub date_format: Option<String>,
    pub fiscal_year_start_month: Option<i32>,
}

impl SetupService {
    /// Returns true if no users exist (first-run state).
    pub async fn is_setup_needed(db: &DatabaseConnection) -> Result<bool, AppError> {
        let users = UserRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(users.is_empty())
    }

    /// Returns branding info if company settings exist, None otherwise.
    pub async fn get_branding(
        db: &DatabaseConnection,
    ) -> Result<Option<(String, Option<String>, Option<String>)>, AppError> {
        let settings = SettingsRepo::find(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        match settings {
            Some(s) if !s.legal_name.is_empty() => {
                Ok(Some((s.legal_name, s.trade_name, s.logo_url)))
            }
            _ => Ok(None),
        }
    }

    /// Creates admin user, company settings, and initial fiscal year.
    /// Returns JWT tokens for auto-login. Returns 409 if users already exist.
    /// The user creation relies on the email UNIQUE constraint as a final guard
    /// against race conditions.
    pub async fn complete_setup(
        db: &DatabaseConnection,
        jwt: &JwtService,
        input: SetupInput,
    ) -> Result<SetupResult, AppError> {
        // Guard: reject if any user already exists
        if !Self::is_setup_needed(db).await? {
            return Err(AppError::Conflict(
                "Setup already completed".to_string(),
            ));
        }

        // Validate input
        if input.admin_email.trim().is_empty() || !input.admin_email.contains('@') {
            return Err(AppError::Validation("Valid email is required".to_string()));
        }
        if input.admin_password.len() < 8 {
            return Err(AppError::Validation(
                "Password must be at least 8 characters".to_string(),
            ));
        }
        if input.legal_name.trim().is_empty() {
            return Err(AppError::Validation(
                "Company legal name is required".to_string(),
            ));
        }

        let now = Utc::now().naive_utc();
        let language = normalize_or_default(Some(&input.admin_language), "en");

        // 1. Create admin user (UNIQUE email constraint prevents duplicates)
        let pw_hash = password::hash_password(&input.admin_password)?;
        let user_id = Uuid::new_v4().to_string();
        let admin_email = input.admin_email.trim().to_lowercase();
        let admin = user::ActiveModel {
            id: Set(user_id.clone()),
            email: Set(admin_email.clone()),
            password_hash: Set(pw_hash),
            full_name: Set(input.admin_full_name),
            language: Set(language.clone()),
            avatar_url: Set(None),
            role_id: Set("role-admin".to_string()),
            is_active: Set(true),
            employee_id: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };
        UserRepo::create(db, admin).await.map_err(|e| {
            let msg = e.to_string();
            if msg.contains("UNIQUE") || msg.contains("unique") || msg.contains("duplicate") {
                AppError::Conflict("Setup already completed".to_string())
            } else {
                AppError::Database(msg)
            }
        })?;

        // Re-check: if more than 1 user exists, another setup raced us
        let all_users = UserRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        if all_users.len() > 1 {
            return Err(AppError::Conflict("Setup already completed".to_string()));
        }

        // 2. Create or update company settings
        let country = input.country.unwrap_or_else(|| "CH".to_string());
        let vat_method = input.vat_method.unwrap_or_else(|| "effective".to_string());
        let flat_rate = input.flat_rate_percentage.map(|v| {
            Decimal::from_f64(v).unwrap_or(Decimal::new(62, 1))
        });
        let date_format = input
            .date_format
            .unwrap_or_else(|| "dd.MM.yyyy".to_string());
        let fy_start = input.fiscal_year_start_month.unwrap_or(1);

        let existing = SettingsRepo::find(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if let Some(existing) = existing {
            // Update existing settings
            let mut model: company_setting::ActiveModel = existing.into();
            model.legal_name = Set(input.legal_name);
            model.trade_name = Set(input.trade_name);
            model.street = Set(input.street.unwrap_or_default());
            model.postal_code = Set(input.postal_code.unwrap_or_default());
            model.city = Set(input.city.unwrap_or_default());
            model.country = Set(country);
            model.vat_method = Set(vat_method);
            model.flat_rate_percentage = Set(flat_rate);
            model.date_format = Set(date_format);
            model.fiscal_year_start_month = Set(fy_start);
            model.ui_language = Set(language);
            if let Some(le) = input.legal_entity_type {
                model.legal_entity_type = Set(Some(le));
            }
            if let Some(cur) = &input.default_currency {
                model.default_currency_id = Set(if cur.is_empty() {
                    None
                } else {
                    Some(cur.clone())
                });
            }
            model.updated_at = Set(now);
            SettingsRepo::update(db, model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        } else {
            // Create new settings
            let model = company_setting::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                legal_name: Set(input.legal_name),
                trade_name: Set(input.trade_name),
                street: Set(input.street.unwrap_or_default()),
                postal_code: Set(input.postal_code.unwrap_or_default()),
                city: Set(input.city.unwrap_or_default()),
                country: Set(country.clone()),
                email: Set(None),
                phone: Set(None),
                website: Set(None),
                vat_number: Set(None),
                vat_method: Set(vat_method),
                flat_rate_percentage: Set(flat_rate),
                register_number: Set(None),
                logo_url: Set(None),
                default_currency_id: Set(input.default_currency),
                date_format: Set(date_format),
                number_format: Set("ch".to_string()),
                ui_language: Set(language),
                fiscal_year_start_month: Set(fy_start),
                tax_id_label: Set("UID/MWST".to_string()),
                jurisdiction: Set(country),
                legal_entity_type: Set(input.legal_entity_type),
                audit_optout: Set(true),
                project_number_auto: Set(false),
                project_number_prefix: Set("P-".to_string()),
                project_number_restart_yearly: Set(false),
                project_number_start: Set(1),
                project_number_min_length: Set(3),
                customer_number_auto: Set(false),
                customer_number_prefix: Set("K-".to_string()),
                customer_number_restart_yearly: Set(false),
                customer_number_start: Set(1),
                customer_number_min_length: Set(6),
                employee_number_auto: Set(false),
                employee_number_prefix: Set("M-".to_string()),
                employee_number_restart_yearly: Set(false),
                employee_number_start: Set(1),
                employee_number_min_length: Set(3),
                created_at: Set(now),
                updated_at: Set(now),
            };
            SettingsRepo::create(db, model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        // 3. Create initial fiscal year
        let current_year = Utc::now().year();
        let fy_start_date = NaiveDate::from_ymd_opt(current_year, fy_start as u32, 1)
            .or_else(|| NaiveDate::from_ymd_opt(current_year, 1, 1))
            .ok_or_else(|| AppError::Internal("Invalid fiscal year start date".to_string()))?;
        let fy_end_date = if fy_start == 1 {
            NaiveDate::from_ymd_opt(current_year, 12, 31)
                .ok_or_else(|| AppError::Internal("Invalid fiscal year end date".to_string()))?
        } else {
            let end_year = current_year + 1;
            let end_month = if fy_start == 1 { 12 } else { fy_start as u32 - 1 };
            let last_day = last_day_of_month(end_year, end_month)?;
            NaiveDate::from_ymd_opt(end_year, end_month, last_day)
                .ok_or_else(|| AppError::Internal("Invalid fiscal year end date".to_string()))?
        };

        // Only create if no fiscal year exists yet
        let existing_fy = FiscalYearRepo::find_paginated(db, 1, 1, None)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if existing_fy.0.is_empty() {
            let fy_id = Uuid::new_v4().to_string();
            let fy_name = if fy_start == 1 {
                format!("FY {current_year}")
            } else {
                format!("FY {current_year}/{}", current_year + 1)
            };

            let fy = fiscal_year::ActiveModel {
                id: Set(fy_id.clone()),
                name: Set(fy_name),
                start_date: Set(fy_start_date),
                end_date: Set(fy_end_date),
                status: Set(FiscalYearStatus::Open.to_string()),
                created_at: Set(now),
                updated_at: Set(now),
            };
            FiscalYearRepo::create(db, fy)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

            // Create 12 monthly periods
            for i in 0..12u32 {
                let month = ((fy_start as u32 - 1 + i) % 12) + 1;
                let year = if month >= fy_start as u32 {
                    fy_start_date.year()
                } else {
                    fy_start_date.year() + 1
                };
                let p_start = NaiveDate::from_ymd_opt(year, month, 1)
                    .ok_or_else(|| AppError::Internal(format!("Invalid period start: {year}-{month}")))?;
                let p_end_day = last_day_of_month(year, month)?;
                let p_end = NaiveDate::from_ymd_opt(year, month, p_end_day)
                    .ok_or_else(|| AppError::Internal(format!("Invalid period end: {year}-{month}-{p_end_day}")))?;

                let period = fiscal_period::ActiveModel {
                    id: Set(Uuid::new_v4().to_string()),
                    fiscal_year_id: Set(fy_id.clone()),
                    name: Set(format!("Period {}", i + 1)),
                    start_date: Set(Some(p_start)),
                    end_date: Set(Some(p_end)),
                    period_number: Set(Some((i + 1) as i32)),
                    status: Set(FiscalPeriodStatus::Open.to_string()),
                    created_at: Set(now),
                    updated_at: Set(now),
                };
                FiscalPeriodRepo::create(db, period)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?;
            }
        }

        // 4. Generate JWT tokens
        let access_token =
            jwt.create_access_token(&user_id, &admin_email, UserRole::Admin)?;
        let refresh_token =
            jwt.create_refresh_token(&user_id, &admin_email, UserRole::Admin)?;

        tracing::info!(admin_email = %admin_email, action = "setup_completed", "Initial setup completed");

        Ok(SetupResult {
            access_token,
            refresh_token,
        })
    }
}

fn last_day_of_month(year: i32, month: u32) -> Result<u32, AppError> {
    let next = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };
    next.ok_or_else(|| AppError::Internal(format!("Invalid date: {year}-{month}")))?
        .pred_opt()
        .ok_or_else(|| AppError::Internal(format!("No previous day for {year}-{month}")))
        .map(|d| d.day())
}
