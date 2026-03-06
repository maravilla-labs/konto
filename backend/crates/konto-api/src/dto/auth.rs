use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MeResponse {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub language: String,
    pub avatar_url: Option<String>,
    pub role: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMyLanguageRequest {
    pub language: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMyProfileRequest {
    pub full_name: Option<String>,
    pub language: Option<String>,
}
