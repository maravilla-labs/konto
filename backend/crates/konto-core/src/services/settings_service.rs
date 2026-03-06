use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::company_setting;
use konto_db::repository::settings_repo::SettingsRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::language::normalize_or_default;

pub struct SettingsService;

impl SettingsService {
    /// Returns the singleton company settings, or creates a default row if none exists.
    pub async fn get_or_create(
        db: &DatabaseConnection,
    ) -> Result<company_setting::Model, AppError> {
        if let Some(s) = SettingsRepo::find(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
        {
            return Ok(s);
        }

        let now = Utc::now().naive_utc();
        let model = company_setting::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            legal_name: Set(String::new()),
            trade_name: Set(None),
            street: Set(String::new()),
            postal_code: Set(String::new()),
            city: Set(String::new()),
            country: Set("CH".to_string()),
            email: Set(None),
            phone: Set(None),
            website: Set(None),
            vat_number: Set(None),
            vat_method: Set("flat_rate".to_string()),
            flat_rate_percentage: Set(Some(Decimal::new(62, 1))),
            register_number: Set(None),
            logo_url: Set(None),
            default_currency_id: Set(None),
            date_format: Set("dd.MM.yyyy".to_string()),
            number_format: Set("ch".to_string()),
            ui_language: Set("en".to_string()),
            fiscal_year_start_month: Set(1),
            tax_id_label: Set("UID/MWST".to_string()),
            jurisdiction: Set("CH".to_string()),
            legal_entity_type: Set(Some("GmbH".to_string())),
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
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        legal_name: &str,
        trade_name: Option<String>,
        street: &str,
        postal_code: &str,
        city: &str,
        country: &str,
        email: Option<String>,
        phone: Option<String>,
        website: Option<String>,
        vat_number: Option<String>,
        vat_method: &str,
        flat_rate_percentage: Option<Decimal>,
        register_number: Option<String>,
        default_currency_id: Option<String>,
        date_format: Option<String>,
        number_format: Option<String>,
        ui_language: Option<String>,
        fiscal_year_start_month: Option<i32>,
        tax_id_label: Option<String>,
        audit_optout: Option<bool>,
        project_number_auto: Option<bool>,
        project_number_prefix: Option<String>,
        project_number_restart_yearly: Option<bool>,
        project_number_start: Option<i32>,
        project_number_min_length: Option<i32>,
        customer_number_auto: Option<bool>,
        customer_number_prefix: Option<String>,
        customer_number_restart_yearly: Option<bool>,
        customer_number_start: Option<i32>,
        customer_number_min_length: Option<i32>,
        employee_number_auto: Option<bool>,
        employee_number_prefix: Option<String>,
        employee_number_restart_yearly: Option<bool>,
        employee_number_start: Option<i32>,
        employee_number_min_length: Option<i32>,
    ) -> Result<company_setting::Model, AppError> {
        let existing = Self::get_or_create(db).await?;
        let now = Utc::now().naive_utc();

        let mut model: company_setting::ActiveModel = existing.into();
        model.legal_name = Set(legal_name.to_string());
        model.trade_name = Set(trade_name);
        model.street = Set(street.to_string());
        model.postal_code = Set(postal_code.to_string());
        model.city = Set(city.to_string());
        model.country = Set(country.to_string());
        model.email = Set(email);
        model.phone = Set(phone);
        model.website = Set(website);
        model.vat_number = Set(vat_number);
        model.vat_method = Set(vat_method.to_string());
        model.flat_rate_percentage = Set(flat_rate_percentage);
        model.register_number = Set(register_number);
        if let Some(cid) = default_currency_id {
            model.default_currency_id = Set(if cid.is_empty() { None } else { Some(cid) });
        }
        if let Some(df) = date_format {
            model.date_format = Set(df);
        }
        if let Some(nf) = number_format {
            model.number_format = Set(nf);
        }
        if let Some(lang) = ui_language {
            model.ui_language = Set(normalize_or_default(Some(&lang), "en"));
        }
        if let Some(fy) = fiscal_year_start_month {
            model.fiscal_year_start_month = Set(fy);
        }
        if let Some(tl) = tax_id_label {
            model.tax_id_label = Set(tl);
        }
        if let Some(ao) = audit_optout {
            model.audit_optout = Set(ao);
        }
        if let Some(v) = project_number_auto { model.project_number_auto = Set(v); }
        if let Some(v) = project_number_prefix { model.project_number_prefix = Set(v); }
        if let Some(v) = project_number_restart_yearly { model.project_number_restart_yearly = Set(v); }
        if let Some(v) = project_number_start { model.project_number_start = Set(v); }
        if let Some(v) = project_number_min_length { model.project_number_min_length = Set(v); }
        if let Some(v) = customer_number_auto { model.customer_number_auto = Set(v); }
        if let Some(v) = customer_number_prefix { model.customer_number_prefix = Set(v); }
        if let Some(v) = customer_number_restart_yearly { model.customer_number_restart_yearly = Set(v); }
        if let Some(v) = customer_number_start { model.customer_number_start = Set(v); }
        if let Some(v) = customer_number_min_length { model.customer_number_min_length = Set(v); }
        if let Some(v) = employee_number_auto { model.employee_number_auto = Set(v); }
        if let Some(v) = employee_number_prefix { model.employee_number_prefix = Set(v); }
        if let Some(v) = employee_number_restart_yearly { model.employee_number_restart_yearly = Set(v); }
        if let Some(v) = employee_number_start { model.employee_number_start = Set(v); }
        if let Some(v) = employee_number_min_length { model.employee_number_min_length = Set(v); }
        model.updated_at = Set(now);

        SettingsRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update_logo_url(
        db: &DatabaseConnection,
        logo_url: Option<String>,
    ) -> Result<company_setting::Model, AppError> {
        let existing = Self::get_or_create(db).await?;
        let now = Utc::now().naive_utc();

        let mut model: company_setting::ActiveModel = existing.into();
        model.logo_url = Set(logo_url);
        model.updated_at = Set(now);

        SettingsRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}
