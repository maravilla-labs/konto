use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct EmailSettingsResponse {
    pub id: String,
    pub smtp_host: String,
    pub smtp_port: i32,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_encryption: String,
    pub from_email: String,
    pub from_name: String,
    pub reply_to_email: Option<String>,
    pub bcc_email: Option<String>,
    pub is_active: bool,
}

impl From<konto_db::entities::email_setting::Model> for EmailSettingsResponse {
    fn from(m: konto_db::entities::email_setting::Model) -> Self {
        Self {
            id: m.id,
            smtp_host: m.smtp_host,
            smtp_port: m.smtp_port,
            smtp_username: m.smtp_username,
            smtp_password: if m.smtp_password.is_empty() {
                String::new()
            } else {
                "********".to_string()
            },
            smtp_encryption: m.smtp_encryption,
            from_email: m.from_email,
            from_name: m.from_name,
            reply_to_email: m.reply_to_email,
            bcc_email: m.bcc_email,
            is_active: m.is_active,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEmailSettingsRequest {
    pub smtp_host: String,
    pub smtp_port: i32,
    pub smtp_username: String,
    pub smtp_password: Option<String>,
    pub smtp_encryption: String,
    pub from_email: String,
    pub from_name: String,
    pub reply_to_email: Option<String>,
    pub bcc_email: Option<String>,
    #[serde(default)]
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TestEmailRequest {
    pub to_email: Option<String>,
}
