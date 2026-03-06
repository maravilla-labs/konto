use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TemplateResponse {
    pub id: String,
    pub name: String,
    pub template_type: String,
    pub content_json: String,
    pub header_json: Option<String>,
    pub footer_json: Option<String>,
    pub page_setup_json: Option<String>,
    pub is_default: bool,
    pub created_by: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub template_type: String,
    pub content_json: String,
    pub header_json: Option<String>,
    pub footer_json: Option<String>,
    pub page_setup_json: Option<String>,
    #[serde(default)]
    pub is_default: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTemplateRequest {
    pub name: String,
    pub template_type: String,
    pub content_json: String,
    pub header_json: Option<String>,
    pub footer_json: Option<String>,
    pub page_setup_json: Option<String>,
    #[serde(default)]
    pub is_default: bool,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct TemplateListParams {
    #[param(default = 1, minimum = 1)]
    pub page: Option<u64>,
    #[param(default = 50, minimum = 1, maximum = 200)]
    pub per_page: Option<u64>,
    pub template_type: Option<String>,
    pub search: Option<String>,
}

impl TemplateListParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(50).clamp(1, 200)
    }
}

impl From<konto_db::entities::document_template::Model> for TemplateResponse {
    fn from(m: konto_db::entities::document_template::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            template_type: m.template_type,
            content_json: m.content_json,
            header_json: m.header_json,
            footer_json: m.footer_json,
            page_setup_json: m.page_setup_json,
            is_default: m.is_default,
            created_by: m.created_by,
        }
    }
}
