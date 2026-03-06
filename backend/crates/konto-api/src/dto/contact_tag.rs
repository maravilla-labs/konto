use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ContactTagResponse {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateContactTagRequest {
    pub name: String,
    pub color: String,
}

impl From<konto_db::entities::contact_tag::Model> for ContactTagResponse {
    fn from(m: konto_db::entities::contact_tag::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            color: m.color,
        }
    }
}
