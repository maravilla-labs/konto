use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct AccountResponse {
    pub id: String,
    pub number: i32,
    pub name: String,
    pub account_type: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub currency_id: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAccountRequest {
    pub number: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub currency_id: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAccountRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub is_active: Option<bool>,
    pub parent_id: Option<Option<String>>,
    pub currency_id: Option<Option<String>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AccountTreeNode {
    pub id: String,
    pub number: i32,
    pub name: String,
    pub account_type: String,
    pub description: Option<String>,
    pub is_active: bool,
    #[schema(no_recursion)]
    pub children: Vec<AccountTreeNode>,
}

impl From<konto_db::entities::account::Model> for AccountResponse {
    fn from(m: konto_db::entities::account::Model) -> Self {
        Self {
            id: m.id,
            number: m.number,
            name: m.name,
            account_type: m.account_type,
            description: m.description,
            parent_id: m.parent_id,
            currency_id: m.currency_id,
            is_active: m.is_active,
        }
    }
}
