use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, ToSchema)]
pub struct TimeEntryResponse {
    pub id: String,
    pub project_id: Option<String>,
    pub contact_id: Option<String>,
    pub user_id: Option<String>,
    pub activity_type_id: Option<String>,
    pub task_id: Option<String>,
    pub date: String,
    pub actual_minutes: i32,
    pub estimated_minutes: Option<i32>,
    pub description: Option<String>,
    #[schema(value_type = Option<String>)]
    pub flat_amount: Option<Decimal>,
    pub travel_minutes: Option<i32>,
    #[schema(value_type = Option<String>)]
    pub quantity: Option<Decimal>,
    pub timesheet_id: Option<String>,
    pub status: String,
    pub billed: bool,
    pub billable: bool,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TransitionTimeEntryRequest {
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTimeEntryRequest {
    pub project_id: Option<String>,
    pub contact_id: Option<String>,
    /// If provided and caller is admin, use this user_id instead of the JWT subject.
    pub user_id: Option<String>,
    pub activity_type_id: Option<String>,
    pub task_id: Option<String>,
    pub date: String,
    pub actual_minutes: i32,
    pub estimated_minutes: Option<i32>,
    pub description: Option<String>,
    #[schema(value_type = Option<String>)]
    pub flat_amount: Option<Decimal>,
    pub travel_minutes: Option<i32>,
    #[schema(value_type = Option<String>)]
    pub travel_flat_rate: Option<Decimal>,
    #[schema(value_type = Option<String>)]
    pub travel_distance: Option<Decimal>,
    #[schema(value_type = Option<String>)]
    pub quantity: Option<Decimal>,
    #[serde(default = "default_true")]
    pub billable: bool,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

fn default_true() -> bool { true }

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTimeEntryRequest {
    pub project_id: Option<Option<String>>,
    pub contact_id: Option<Option<String>>,
    pub activity_type_id: Option<Option<String>>,
    pub task_id: Option<Option<String>>,
    pub date: Option<String>,
    pub actual_minutes: Option<i32>,
    pub estimated_minutes: Option<Option<i32>>,
    pub description: Option<Option<String>>,
    #[schema(value_type = Option<Option<String>>)]
    pub flat_amount: Option<Option<Decimal>>,
    pub travel_minutes: Option<Option<i32>>,
    #[schema(value_type = Option<Option<String>>)]
    pub quantity: Option<Option<Decimal>>,
    pub billable: Option<bool>,
    pub start_time: Option<Option<String>>,
    pub end_time: Option<Option<String>>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct TimeEntryListParams {
    #[param(default = 1, minimum = 1)]
    pub page: Option<u64>,
    #[param(default = 50, minimum = 1, maximum = 200)]
    pub per_page: Option<u64>,
    pub project_id: Option<String>,
    pub search: Option<String>,
    pub billed: Option<bool>,
    pub status: Option<String>,
    pub billable: Option<bool>,
}

impl TimeEntryListParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }
    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(50).clamp(1, 200)
    }
}

impl From<konto_db::entities::time_entry::Model> for TimeEntryResponse {
    fn from(m: konto_db::entities::time_entry::Model) -> Self {
        Self {
            id: m.id,
            project_id: m.project_id,
            contact_id: m.contact_id,
            user_id: m.user_id,
            activity_type_id: m.activity_type_id,
            task_id: m.task_id,
            date: m.date.to_string(),
            actual_minutes: m.actual_minutes,
            estimated_minutes: m.estimated_minutes,
            description: m.description,
            flat_amount: m.flat_amount,
            travel_minutes: m.travel_minutes,
            quantity: m.quantity,
            timesheet_id: m.timesheet_id,
            status: m.status,
            billed: m.billed,
            billable: m.billable,
            start_time: m.start_time,
            end_time: m.end_time,
        }
    }
}
