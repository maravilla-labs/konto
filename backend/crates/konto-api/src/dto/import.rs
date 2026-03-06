use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ImportJobResponse {
    pub id: String,
    pub import_type: String,
    pub file_name: String,
    pub status: String,
    pub total_rows: Option<i32>,
    pub imported_rows: Option<i32>,
    pub error_rows: Option<i32>,
    pub error_log: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ImportUploadQuery {
    pub import_type: String,
}

impl From<konto_db::entities::import_job::Model> for ImportJobResponse {
    fn from(m: konto_db::entities::import_job::Model) -> Self {
        let error_log = m.error_log.as_deref()
            .and_then(|s| serde_json::from_str::<Vec<String>>(s).ok());
        Self {
            id: m.id,
            import_type: m.import_type,
            file_name: m.file_name,
            status: m.status,
            total_rows: m.total_rows,
            imported_rows: m.imported_rows,
            error_rows: m.error_rows,
            error_log,
        }
    }
}
