use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProjectSubStatusResponse {
    pub id: String,
    pub name: String,
    pub sort_order: i32,
    pub color: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectSubStatusRequest {
    pub name: String,
    pub sort_order: Option<i32>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectSubStatusRequest {
    pub name: Option<String>,
    pub sort_order: Option<i32>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
}

impl From<konto_db::entities::project_sub_status::Model> for ProjectSubStatusResponse {
    fn from(m: konto_db::entities::project_sub_status::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            sort_order: m.sort_order,
            color: m.color,
            is_active: m.is_active,
        }
    }
}
