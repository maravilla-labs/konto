use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::contact;
use konto_db::repository::contact_repo::ContactRepo;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use super::language::normalize_language;

pub struct ContactService;

impl ContactService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        search: Option<&str>,
        category: Option<&str>,
    ) -> Result<(Vec<contact::Model>, u64), AppError> {
        ContactRepo::find_paginated(db, page, per_page, search, category)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: &str) -> Result<contact::Model, AppError> {
        ContactRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Contact not found".to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        contact_type: &str,
        name1: &str,
        name2: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        address: Option<String>,
        postal_code: Option<String>,
        city: Option<String>,
        country: Option<String>,
        language: Option<String>,
        notes: Option<String>,
        salutation: Option<String>,
        title: Option<String>,
        email2: Option<String>,
        phone2: Option<String>,
        mobile: Option<String>,
        fax: Option<String>,
        industry: Option<String>,
        birthday: Option<String>,
        employee_count: Option<i32>,
        trade_register_number: Option<String>,
        salutation_form: Option<String>,
        website: Option<String>,
        vat_number: Option<String>,
        category: Option<String>,
        customer_number: Option<String>,
        vat_mode: Option<String>,
    ) -> Result<contact::Model, AppError> {
        let now = Utc::now().naive_utc();

        let birthday_date = birthday
            .as_deref()
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

        let resolved_customer_number = if customer_number.is_some() {
            customer_number
        } else {
            Self::auto_assign_customer_number(db).await?
        };

        let model = contact::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            contact_type: Set(contact_type.to_string()),
            category: Set(category),
            industry: Set(industry),
            name1: Set(name1.to_string()),
            name2: Set(name2),
            salutation: Set(salutation),
            title: Set(title),
            address: Set(address),
            postal_code: Set(postal_code),
            city: Set(city),
            country: Set(country),
            email: Set(email),
            email2: Set(email2),
            phone: Set(phone),
            phone2: Set(phone2),
            mobile: Set(mobile),
            fax: Set(fax),
            website: Set(website),
            vat_number: Set(vat_number),
            language: Set(normalize_language(language.as_deref())),
            notes: Set(notes),
            birthday: Set(birthday_date),
            employee_count: Set(employee_count),
            trade_register_number: Set(trade_register_number),
            salutation_form: Set(salutation_form),
            customer_number: Set(resolved_customer_number),
            vat_mode: Set(vat_mode.unwrap_or_else(|| "auto".to_string())),
            bexio_id: Set(None),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
        };

        ContactRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name1: Option<String>,
        contact_type: Option<String>,
        name2: Option<Option<String>>,
        email: Option<Option<String>>,
        phone: Option<Option<String>>,
        city: Option<Option<String>>,
        country: Option<Option<String>>,
        language: Option<Option<String>>,
        is_active: Option<bool>,
        notes: Option<Option<String>>,
        address: Option<Option<String>>,
        postal_code: Option<Option<String>>,
        website: Option<Option<String>>,
        vat_number: Option<Option<String>>,
        salutation: Option<Option<String>>,
        title: Option<Option<String>>,
        email2: Option<Option<String>>,
        phone2: Option<Option<String>>,
        mobile: Option<Option<String>>,
        fax: Option<Option<String>>,
        industry: Option<Option<String>>,
        birthday: Option<Option<String>>,
        employee_count: Option<Option<i32>>,
        trade_register_number: Option<Option<String>>,
        salutation_form: Option<Option<String>>,
        customer_number: Option<Option<String>>,
        category: Option<Option<String>>,
        vat_mode: Option<String>,
    ) -> Result<contact::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let mut model: contact::ActiveModel = existing.into();

        if let Some(n) = name1 { model.name1 = Set(n); }
        if let Some(ct) = contact_type { model.contact_type = Set(ct); }
        if let Some(n) = name2 { model.name2 = Set(n); }
        if let Some(e) = email { model.email = Set(e); }
        if let Some(p) = phone { model.phone = Set(p); }
        if let Some(c) = city { model.city = Set(c); }
        if let Some(c) = country { model.country = Set(c); }
        if let Some(lang) = language {
            model.language = Set(normalize_language(lang.as_deref()));
        }
        if let Some(a) = is_active { model.is_active = Set(a); }
        if let Some(n) = notes { model.notes = Set(n); }
        if let Some(a) = address { model.address = Set(a); }
        if let Some(p) = postal_code { model.postal_code = Set(p); }
        if let Some(w) = website { model.website = Set(w); }
        if let Some(v) = vat_number { model.vat_number = Set(v); }
        if let Some(s) = salutation { model.salutation = Set(s); }
        if let Some(t) = title { model.title = Set(t); }
        if let Some(e) = email2 { model.email2 = Set(e); }
        if let Some(p) = phone2 { model.phone2 = Set(p); }
        if let Some(m_val) = mobile { model.mobile = Set(m_val); }
        if let Some(f) = fax { model.fax = Set(f); }
        if let Some(i) = industry { model.industry = Set(i); }
        if let Some(b) = birthday {
            let date = b.as_deref()
                .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
            model.birthday = Set(date);
        }
        if let Some(e) = employee_count { model.employee_count = Set(e); }
        if let Some(t) = trade_register_number { model.trade_register_number = Set(t); }
        if let Some(s) = salutation_form { model.salutation_form = Set(s); }
        if let Some(cn) = customer_number { model.customer_number = Set(cn); }
        if let Some(c) = category { model.category = Set(c); }
        if let Some(v) = vat_mode { model.vat_mode = Set(v); }
        model.updated_at = Set(Utc::now().naive_utc());

        ContactRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        Self::get_by_id(db, id).await?;
        ContactRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn auto_assign_customer_number(
        db: &DatabaseConnection,
    ) -> Result<Option<String>, AppError> {
        use konto_db::entities::company_setting;

        let settings = company_setting::Entity::find()
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let settings = match settings {
            Some(s) if s.customer_number_auto => s,
            _ => return Ok(None),
        };

        let all_contacts = contact::Entity::find()
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let prefix = &settings.customer_number_prefix;
        let year = chrono::Utc::now().format("%Y").to_string();
        let year_prefix = if settings.customer_number_restart_yearly {
            format!("{}{}-", prefix, year)
        } else {
            prefix.clone()
        };

        let max_num = all_contacts
            .iter()
            .filter_map(|c| {
                c.customer_number.as_ref().and_then(|n| {
                    n.strip_prefix(&year_prefix)
                        .and_then(|s| s.parse::<i32>().ok())
                })
            })
            .max()
            .unwrap_or(settings.customer_number_start - 1);

        let next = max_num + 1;
        let min_len = settings.customer_number_min_length as usize;
        let number_str = format!("{:0>width$}", next, width = min_len);

        Ok(Some(format!("{}{}", year_prefix, number_str)))
    }
}
