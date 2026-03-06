use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct JournalAttachmentResponse {
    pub id: String,
    pub journal_entry_id: String,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub uploaded_by: Option<String>,
    pub created_at: String,
}

impl From<konto_db::entities::journal_attachment::Model> for JournalAttachmentResponse {
    fn from(m: konto_db::entities::journal_attachment::Model) -> Self {
        Self {
            id: m.id,
            journal_entry_id: m.journal_entry_id,
            file_name: m.file_name,
            file_size: m.file_size,
            mime_type: m.mime_type,
            uploaded_by: m.uploaded_by,
            created_at: m.created_at.to_rfc3339(),
        }
    }
}
