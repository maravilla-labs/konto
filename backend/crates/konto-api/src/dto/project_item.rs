use konto_core::services::project_item_service::ProjectItemTree;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectItemResponse {
    pub id: String,
    pub project_id: String,
    pub parent_id: Option<String>,
    pub item_type: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub assignee_id: Option<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    #[schema(value_type = Option<String>)]
    pub estimated_hours: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub budget_hours: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub budget_amount: Option<rust_decimal::Decimal>,
    pub sort_order: i32,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectItemTreeResponse {
    #[serde(flatten)]
    pub item: ProjectItemResponse,
    pub children: Vec<ProjectItemTreeResponse>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectItemRequest {
    pub parent_id: Option<String>,
    pub item_type: String,
    pub name: String,
    pub description: Option<String>,
    pub assignee_id: Option<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    #[schema(value_type = Option<String>)]
    pub estimated_hours: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub budget_hours: Option<rust_decimal::Decimal>,
    #[schema(value_type = Option<String>)]
    pub budget_amount: Option<rust_decimal::Decimal>,
    #[serde(default)]
    pub sort_order: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectItemRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub status: Option<String>,
    pub assignee_id: Option<Option<String>>,
    pub start_date: Option<Option<String>>,
    pub due_date: Option<Option<String>>,
    #[schema(value_type = Option<Option<String>>)]
    pub estimated_hours: Option<Option<rust_decimal::Decimal>>,
    #[schema(value_type = Option<Option<String>>)]
    pub budget_hours: Option<Option<rust_decimal::Decimal>>,
    #[schema(value_type = Option<Option<String>>)]
    pub budget_amount: Option<Option<rust_decimal::Decimal>>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReorderProjectItemRequest {
    pub new_parent_id: Option<String>,
    pub new_sort_order: i32,
}

impl From<konto_db::entities::project_item::Model> for ProjectItemResponse {
    fn from(m: konto_db::entities::project_item::Model) -> Self {
        Self {
            id: m.id,
            project_id: m.project_id,
            parent_id: m.parent_id,
            item_type: m.item_type,
            name: m.name,
            description: m.description,
            status: m.status,
            assignee_id: m.assignee_id,
            start_date: m.start_date.map(|d| d.to_string()),
            due_date: m.due_date.map(|d| d.to_string()),
            estimated_hours: m.estimated_hours,
            budget_hours: m.budget_hours,
            budget_amount: m.budget_amount,
            sort_order: m.sort_order,
            created_by: m.created_by,
            updated_by: m.updated_by,
            created_at: m.created_at.to_string(),
            updated_at: m.updated_at.to_string(),
        }
    }
}

impl From<ProjectItemTree> for ProjectItemTreeResponse {
    fn from(tree: ProjectItemTree) -> Self {
        Self {
            item: ProjectItemResponse::from(tree.item),
            children: tree.children.into_iter().map(Self::from).collect(),
        }
    }
}
