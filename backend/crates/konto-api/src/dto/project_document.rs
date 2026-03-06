use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectDocumentResponse {
    pub id: String,
    pub project_id: String,
    pub project_item_id: Option<String>,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i32,
    pub content_type: Option<String>,
    pub uploaded_by: Option<String>,
    pub created_at: String,
}

impl From<konto_db::entities::project_document::Model> for ProjectDocumentResponse {
    fn from(m: konto_db::entities::project_document::Model) -> Self {
        Self {
            id: m.id,
            project_id: m.project_id,
            project_item_id: m.project_item_id,
            file_name: m.file_name,
            file_path: m.file_path,
            file_size: m.file_size,
            content_type: m.content_type,
            uploaded_by: m.uploaded_by,
            created_at: m.created_at.to_string(),
        }
    }
}
