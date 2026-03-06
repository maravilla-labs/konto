use chrono::Utc;
use konto_common::enums::{TokenType, UserRole};
use konto_common::error::AppError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,       // user id
    pub email: String,
    pub role: UserRole,
    pub token_type: TokenType,
    pub exp: usize,
    pub iat: usize,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_expires_secs: u64,
    refresh_expires_secs: u64,
}

impl JwtService {
    pub fn new(secret: &str, access_expires_secs: u64, refresh_expires_secs: u64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_expires_secs,
            refresh_expires_secs,
        }
    }

    pub fn create_access_token(&self, user_id: &str, email: &str, role: UserRole) -> Result<String, AppError> {
        let now = Utc::now().timestamp() as usize;
        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role,
            token_type: TokenType::Access,
            exp: now + self.access_expires_secs as usize,
            iat: now,
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(format!("Token creation failed: {e}")))
    }

    pub fn create_refresh_token(&self, user_id: &str, email: &str, role: UserRole) -> Result<String, AppError> {
        let now = Utc::now().timestamp() as usize;
        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role,
            token_type: TokenType::Refresh,
            exp: now + self.refresh_expires_secs as usize,
            iat: now,
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(format!("Token creation failed: {e}")))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| AppError::Unauthorized(format!("Invalid token: {e}")))
    }
}
