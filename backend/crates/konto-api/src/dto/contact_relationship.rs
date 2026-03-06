use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ContactRelationshipResponse {
    pub id: String,
    pub person_contact_id: String,
    pub person_name: String,
    pub org_contact_id: String,
    pub org_name: String,
    pub role: Option<String>,
    pub position: Option<String>,
    pub department: Option<String>,
    pub is_primary: bool,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl ContactRelationshipResponse {
    pub fn from_model(
        m: konto_db::entities::contact_relationship::Model,
        person_name: String,
        org_name: String,
    ) -> Self {
        Self {
            id: m.id,
            person_contact_id: m.person_contact_id,
            person_name,
            org_contact_id: m.org_contact_id,
            org_name,
            role: m.role,
            position: m.position,
            department: m.department,
            is_primary: m.is_primary,
            notes: m.notes,
            created_at: m.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            updated_at: m.updated_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateContactRelationshipRequest {
    pub person_contact_id: String,
    pub org_contact_id: String,
    pub role: Option<String>,
    pub position: Option<String>,
    pub department: Option<String>,
    #[serde(default)]
    pub is_primary: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateContactRelationshipRequest {
    pub role: Option<Option<String>>,
    pub position: Option<Option<String>>,
    pub department: Option<Option<String>>,
    pub is_primary: Option<bool>,
    pub notes: Option<Option<String>>,
}
