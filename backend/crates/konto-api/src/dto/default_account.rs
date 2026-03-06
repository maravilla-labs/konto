use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DefaultAccountResponse {
    pub id: String,
    pub setting_key: String,
    pub account_id: Option<String>,
    pub account_name: Option<String>,
    pub account_number: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDefaultAccountsRequest {
    pub settings: Vec<DefaultAccountUpdate>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DefaultAccountUpdate {
    pub setting_key: String,
    pub account_id: Option<String>,
}
