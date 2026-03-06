use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AuditLogResponse {
    pub id: String,
    pub user_id: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub old_values: Option<String>,
    pub new_values: Option<String>,
    pub created_at: String,
}

impl From<konto_db::entities::audit_log::Model> for AuditLogResponse {
    fn from(m: konto_db::entities::audit_log::Model) -> Self {
        Self {
            id: m.id,
            user_id: m.user_id,
            action: m.action,
            entity_type: m.entity_type,
            entity_id: m.entity_id,
            old_values: m.old_values,
            new_values: m.new_values,
            created_at: m.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct AuditLogParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub entity_type: Option<String>,
    pub action: Option<String>,
    pub user_id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

impl AuditLogParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(50).clamp(1, 200)
    }
}
