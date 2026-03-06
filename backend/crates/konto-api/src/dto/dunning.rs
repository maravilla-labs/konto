use konto_db::entities::{dunning_entry, dunning_level};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DunningLevelResponse {
    pub id: String,
    pub level: i32,
    pub days_after_due: i32,
    pub fee_amount: String,
    pub subject_template: String,
    pub body_template: String,
    pub is_active: bool,
}

impl From<dunning_level::Model> for DunningLevelResponse {
    fn from(m: dunning_level::Model) -> Self {
        Self {
            id: m.id,
            level: m.level,
            days_after_due: m.days_after_due,
            fee_amount: m.fee_amount.to_string(),
            subject_template: m.subject_template,
            body_template: m.body_template,
            is_active: m.is_active,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDunningLevelRequest {
    pub days_after_due: i32,
    pub fee_amount: f64,
    pub subject_template: String,
    pub body_template: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DunningEntryResponse {
    pub id: String,
    pub invoice_id: String,
    pub dunning_level_id: String,
    pub level_name: String,
    pub level_number: i32,
    pub sent_at: String,
    pub fee_amount: String,
    pub email_sent: bool,
    pub journal_entry_id: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SendReminderRequest {
    pub dunning_level_id: String,
    #[serde(default = "default_true")]
    pub send_email: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DunningRunResponse {
    pub reminders_sent: u32,
    pub errors: Vec<String>,
}

impl From<dunning_entry::Model> for DunningEntryResponse {
    fn from(m: dunning_entry::Model) -> Self {
        Self {
            id: m.id,
            invoice_id: m.invoice_id,
            dunning_level_id: m.dunning_level_id,
            level_name: String::new(),
            level_number: 0,
            sent_at: m.sent_at.to_string(),
            fee_amount: m.fee_amount.to_string(),
            email_sent: m.email_sent,
            journal_entry_id: m.journal_entry_id,
            notes: m.notes,
        }
    }
}
