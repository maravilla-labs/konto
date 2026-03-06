use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ContactPersonResponse {
    pub id: String,
    pub contact_id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department: Option<String>,
    pub position: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateContactPersonRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department: Option<String>,
    pub position: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateContactPersonRequest {
    pub first_name: Option<Option<String>>,
    pub last_name: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub phone: Option<Option<String>>,
    pub department: Option<Option<String>>,
    pub position: Option<Option<String>>,
}

impl From<konto_db::entities::contact_person::Model> for ContactPersonResponse {
    fn from(m: konto_db::entities::contact_person::Model) -> Self {
        Self {
            id: m.id,
            contact_id: m.contact_id,
            first_name: m.first_name,
            last_name: m.last_name,
            email: m.email,
            phone: m.phone,
            department: m.department,
            position: m.position,
        }
    }
}
