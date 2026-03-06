use konto_common::enums::{TokenType, UserRole};
use konto_common::error::AppError;
use konto_db::entities::role;
use konto_db::repository::user_repo::UserRepo;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

use crate::auth::jwt::JwtService;
use crate::auth::password;
use crate::services::language::normalize_or_default;

fn parse_role(name: &str) -> UserRole {
    match name {
        "admin" => UserRole::Admin,
        "accountant" => UserRole::Accountant,
        "auditor" => UserRole::Auditor,
        _ => UserRole::User,
    }
}

pub struct AuthService;

#[derive(Debug, serde::Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, serde::Serialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub language: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub permissions: String,
}

impl AuthService {
    pub async fn login(
        db: &DatabaseConnection,
        jwt: &JwtService,
        email: &str,
        password_input: &str,
    ) -> Result<TokenPair, AppError> {
        let user = UserRepo::find_by_email(db, email)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        if !user.is_active {
            return Err(AppError::Unauthorized("Account is disabled".to_string()));
        }

        let valid = password::verify_password(password_input, &user.password_hash)?;
        if !valid {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // Get role name
        let role_model = role::Entity::find_by_id(&user.role_id)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::Internal("Role not found".to_string()))?;

        let role = parse_role(&role_model.name);
        let access_token = jwt.create_access_token(&user.id, &user.email, role)?;
        let refresh_token = jwt.create_refresh_token(&user.id, &user.email, role)?;

        tracing::info!(user_id = %user.id, action = "login", "User logged in");

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    pub async fn refresh(
        db: &DatabaseConnection,
        jwt: &JwtService,
        refresh_token: &str,
    ) -> Result<TokenPair, AppError> {
        let claims = jwt.verify_token(refresh_token)?;

        if claims.token_type != TokenType::Refresh {
            return Err(AppError::Unauthorized("Invalid token type".to_string()));
        }

        let user = UserRepo::find_by_id(db, &claims.sub)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        if !user.is_active {
            return Err(AppError::Unauthorized("Account is disabled".to_string()));
        }

        let access_token = jwt.create_access_token(&user.id, &user.email, claims.role)?;
        let new_refresh = jwt.create_refresh_token(&user.id, &user.email, claims.role)?;

        Ok(TokenPair {
            access_token,
            refresh_token: new_refresh,
        })
    }

    pub async fn get_current_user(
        db: &DatabaseConnection,
        user_id: &str,
    ) -> Result<AuthUser, AppError> {
        let user = UserRepo::find_by_id(db, user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let role_model = role::Entity::find_by_id(&user.role_id)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::Internal("Role not found".to_string()))?;

        Ok(AuthUser {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            language: user.language,
            avatar_url: user.avatar_url,
            role: role_model.name,
            permissions: role_model.permissions,
        })
    }

    pub async fn set_language(
        db: &DatabaseConnection,
        user_id: &str,
        language: &str,
    ) -> Result<AuthUser, AppError> {
        let user = UserRepo::find_by_id(db, user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let mut model: konto_db::entities::user::ActiveModel = user.into();
        model.language = Set(normalize_or_default(Some(language), "en"));
        UserRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Self::get_current_user(db, user_id).await
    }

    pub async fn update_profile(
        db: &DatabaseConnection,
        user_id: &str,
        full_name: Option<String>,
        language: Option<String>,
    ) -> Result<AuthUser, AppError> {
        let user = UserRepo::find_by_id(db, user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let mut model: konto_db::entities::user::ActiveModel = user.into();
        if let Some(name) = full_name.filter(|n| !n.trim().is_empty()) {
            model.full_name = Set(name);
        }
        if let Some(lang) = language {
            model.language = Set(normalize_or_default(Some(&lang), "en"));
        }
        UserRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Self::get_current_user(db, user_id).await
    }

    pub async fn update_avatar_url(
        db: &DatabaseConnection,
        user_id: &str,
        avatar_url: Option<String>,
    ) -> Result<AuthUser, AppError> {
        let user = UserRepo::find_by_id(db, user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let mut model: konto_db::entities::user::ActiveModel = user.into();
        model.avatar_url = Set(avatar_url);
        UserRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Self::get_current_user(db, user_id).await
    }
}
