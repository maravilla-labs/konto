use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct EmailTemplateResponse {
    pub id: String,
    pub template_type: String,
    pub subject: String,
    pub body_html: String,
    pub language: String,
    pub is_default: bool,
}

impl From<konto_db::entities::email_template::Model> for EmailTemplateResponse {
    fn from(m: konto_db::entities::email_template::Model) -> Self {
        Self {
            id: m.id,
            template_type: m.template_type,
            subject: m.subject,
            body_html: m.body_html,
            language: m.language,
            is_default: m.is_default,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEmailTemplateRequest {
    pub subject: String,
    pub body_html: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmailTemplatePreviewResponse {
    pub rendered_subject: String,
    pub rendered_body_html: String,
}
