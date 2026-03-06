use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct TimesheetResponse {
    pub id: String,
    pub user_id: String,
    pub period_start: String,
    pub period_end: String,
    pub status: String,
    pub submitted_at: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTimesheetRequest {
    pub period_start: String,
    pub period_end: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTimesheetRequest {
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub notes: Option<Option<String>>,
}

impl From<konto_db::entities::timesheet::Model> for TimesheetResponse {
    fn from(m: konto_db::entities::timesheet::Model) -> Self {
        Self {
            id: m.id,
            user_id: m.user_id,
            period_start: m.period_start.to_string(),
            period_end: m.period_end.to_string(),
            status: m.status,
            submitted_at: m.submitted_at.map(|t| t.to_string()),
            approved_by: m.approved_by,
            approved_at: m.approved_at.map(|t| t.to_string()),
            notes: m.notes,
            created_at: m.created_at.to_string(),
            updated_at: m.updated_at.to_string(),
        }
    }
}
