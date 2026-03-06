use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub language: String,
    pub avatar_url: Option<String>,
    pub employee_id: Option<String>,
    pub role_id: String,
    pub role_name: String,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub role_id: String,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub email: String,
    pub full_name: String,
    pub role_id: String,
    pub is_active: bool,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
}

impl UserResponse {
    pub fn from_model(user: konto_db::entities::user::Model, role_name: String) -> Self {
        Self {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            language: user.language,
            avatar_url: user.avatar_url,
            employee_id: user.employee_id,
            role_id: user.role_id,
            role_name,
            is_active: user.is_active,
            created_at: user.created_at.to_string(),
        }
    }
}

impl From<konto_db::entities::role::Model> for RoleResponse {
    fn from(m: konto_db::entities::role::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
        }
    }
}
