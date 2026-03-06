use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectMilestoneResponse {
    pub id: String,
    pub project_id: String,
    pub project_item_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub target_date: String,
    pub status: String,
    pub reached_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectMilestoneRequest {
    pub project_item_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub target_date: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectMilestoneRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub target_date: Option<String>,
    pub project_item_id: Option<Option<String>>,
}

impl From<konto_db::entities::project_milestone::Model> for ProjectMilestoneResponse {
    fn from(m: konto_db::entities::project_milestone::Model) -> Self {
        Self {
            id: m.id,
            project_id: m.project_id,
            project_item_id: m.project_item_id,
            name: m.name,
            description: m.description,
            target_date: m.target_date.to_string(),
            status: m.status,
            reached_at: m.reached_at.map(|d| d.to_string()),
            created_at: m.created_at.to_string(),
            updated_at: m.updated_at.to_string(),
        }
    }
}
