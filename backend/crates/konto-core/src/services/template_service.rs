use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::document_template;
use konto_db::repository::template_repo::TemplateRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct TemplateService;

impl TemplateService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        template_type: Option<&str>,
        search: Option<&str>,
    ) -> Result<(Vec<document_template::Model>, u64), AppError> {
        TemplateRepo::find_paginated(db, page, per_page, template_type, search)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<document_template::Model, AppError> {
        TemplateRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Template not found".to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        name: &str,
        template_type: &str,
        content_json: &str,
        header_json: Option<String>,
        footer_json: Option<String>,
        page_setup_json: Option<String>,
        is_default: bool,
        user_id: Option<String>,
    ) -> Result<document_template::Model, AppError> {
        if is_default {
            TemplateRepo::clear_defaults_for_type(db, template_type)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        let now = Utc::now().naive_utc();
        let model = document_template::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(name.to_string()),
            template_type: Set(template_type.to_string()),
            content_json: Set(content_json.to_string()),
            header_json: Set(header_json),
            footer_json: Set(footer_json),
            page_setup_json: Set(page_setup_json),
            is_default: Set(is_default),
            created_by: Set(user_id),
            created_at: Set(now),
            updated_at: Set(now),
        };

        TemplateRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: &str,
        template_type: &str,
        content_json: &str,
        header_json: Option<String>,
        footer_json: Option<String>,
        page_setup_json: Option<String>,
        is_default: bool,
    ) -> Result<document_template::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;

        if is_default {
            TemplateRepo::clear_defaults_for_type(db, template_type)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        let now = Utc::now().naive_utc();
        let mut model: document_template::ActiveModel = existing.into();
        model.name = Set(name.to_string());
        model.template_type = Set(template_type.to_string());
        model.content_json = Set(content_json.to_string());
        model.header_json = Set(header_json);
        model.footer_json = Set(footer_json);
        model.page_setup_json = Set(page_setup_json);
        model.is_default = Set(is_default);
        model.updated_at = Set(now);

        TemplateRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        Self::get_by_id(db, id).await?;
        TemplateRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn duplicate(
        db: &DatabaseConnection,
        id: &str,
        user_id: Option<String>,
    ) -> Result<document_template::Model, AppError> {
        let original = Self::get_by_id(db, id).await?;
        let now = Utc::now().naive_utc();

        let model = document_template::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(format!("{} (Copy)", original.name)),
            template_type: Set(original.template_type),
            content_json: Set(original.content_json),
            header_json: Set(original.header_json),
            footer_json: Set(original.footer_json),
            page_setup_json: Set(original.page_setup_json),
            is_default: Set(false),
            created_by: Set(user_id),
            created_at: Set(now),
            updated_at: Set(now),
        };

        TemplateRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}
