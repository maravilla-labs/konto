use std::collections::HashMap;

use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::email_template;
use konto_db::repository::email_template_repo::EmailTemplateRepo;
use sea_orm::{DatabaseConnection, Set};

pub struct EmailTemplateService;

impl EmailTemplateService {
    pub async fn list(
        db: &DatabaseConnection,
    ) -> Result<Vec<email_template::Model>, AppError> {
        EmailTemplateRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<email_template::Model, AppError> {
        EmailTemplateRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Email template not found".into()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        subject: &str,
        body_html: &str,
    ) -> Result<email_template::Model, AppError> {
        let existing = Self::get(db, id).await?;
        let now = Utc::now().naive_utc();

        let mut model: email_template::ActiveModel = existing.into();
        model.subject = Set(subject.to_string());
        model.body_html = Set(body_html.to_string());
        model.is_default = Set(false);
        model.updated_at = Set(now);

        EmailTemplateRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn render(
        db: &DatabaseConnection,
        template_type: &str,
        language: &str,
        variables: &HashMap<String, String>,
    ) -> Result<(String, String), AppError> {
        let tmpl = EmailTemplateRepo::find_by_type_and_language(db, template_type, language)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "No template for type={template_type}, lang={language}"
                ))
            })?;

        let subject = render_template(&tmpl.subject, variables);
        let body = render_template(&tmpl.body_html, variables);
        Ok((subject, body))
    }

    pub async fn preview(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<(String, String), AppError> {
        let tmpl = Self::get(db, id).await?;
        let vars = sample_variables();
        let subject = render_template(&tmpl.subject, &vars);
        let body = render_template(&tmpl.body_html, &vars);
        Ok((subject, body))
    }
}

fn render_template(template: &str, variables: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in variables {
        let placeholder = format!("{{{{{key}}}}}");
        result = result.replace(&placeholder, value);
    }
    result
}

fn sample_variables() -> HashMap<String, String> {
    let mut vars = HashMap::new();
    vars.insert("company_name".into(), "Acme GmbH".into());
    vars.insert("contact_name".into(), "Max Mustermann".into());
    vars.insert("contact_email".into(), "max@example.com".into());
    vars.insert("invoice_number".into(), "RE-2026-001".into());
    vars.insert("credit_note_number".into(), "CN-2026-001".into());
    vars.insert("document_number".into(), "AN-00001".into());
    vars.insert("amount".into(), "1'250.00".into());
    vars.insert("currency".into(), "CHF".into());
    vars.insert("due_date".into(), "2026-04-01".into());
    vars.insert("today".into(), Utc::now().format("%Y-%m-%d").to_string());
    vars.insert("invoice_date".into(), "2026-03-01".into());
    vars
}
