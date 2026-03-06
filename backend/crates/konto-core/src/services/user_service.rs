use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::{role, user};
use konto_db::repository::user_repo::UserRepo;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::auth::password::hash_password;
use crate::services::language::normalize_or_default;

pub struct UserService;

impl UserService {
    /// List all users with their role name.
    pub async fn list(db: &DatabaseConnection) -> Result<Vec<(user::Model, String)>, AppError> {
        let users = UserRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let roles = role::Entity::find()
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = users
            .into_iter()
            .map(|u| {
                let role_name = roles
                    .iter()
                    .find(|r| r.id == u.role_id)
                    .map(|r| r.name.clone())
                    .unwrap_or_default();
                (u, role_name)
            })
            .collect();

        Ok(result)
    }

    /// Get a single user by ID with role name.
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<(user::Model, String), AppError> {
        let user = UserRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let role = role::Entity::find_by_id(&user.role_id)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .map(|r| r.name)
            .unwrap_or_default();

        Ok((user, role))
    }

    /// Create a new user (email must be unique).
    pub async fn create(
        db: &DatabaseConnection,
        email: &str,
        password: &str,
        full_name: &str,
        role_id: &str,
        language: Option<String>,
    ) -> Result<user::Model, AppError> {
        // Check email uniqueness
        if UserRepo::find_by_email(db, email)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .is_some()
        {
            return Err(AppError::Conflict("Email already in use".to_string()));
        }

        // Verify role exists
        role::Entity::find_by_id(role_id)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

        let now = Utc::now().naive_utc();
        let password_hash = hash_password(password)?;

        let model = user::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            email: Set(email.to_string()),
            password_hash: Set(password_hash),
            full_name: Set(full_name.to_string()),
            language: Set(normalize_or_default(language.as_deref(), "en")),
            avatar_url: Set(None),
            role_id: Set(role_id.to_string()),
            is_active: Set(true),
            employee_id: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        UserRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Update user profile (not password).
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        email: &str,
        full_name: &str,
        role_id: &str,
        is_active: bool,
        language: Option<String>,
    ) -> Result<user::Model, AppError> {
        let existing = UserRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Check email uniqueness if changed
        if existing.email != email
            && UserRepo::find_by_email(db, email)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .is_some()
        {
            return Err(AppError::Conflict("Email already in use".to_string()));
        }

        // Verify role exists
        role::Entity::find_by_id(role_id)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

        let now = Utc::now().naive_utc();
        let mut model: user::ActiveModel = existing.into();
        model.email = Set(email.to_string());
        model.full_name = Set(full_name.to_string());
        model.role_id = Set(role_id.to_string());
        model.is_active = Set(is_active);
        if let Some(lang) = language {
            model.language = Set(normalize_or_default(Some(&lang), "en"));
        }
        model.updated_at = Set(now);

        UserRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Change a user's password.
    pub async fn change_password(
        db: &DatabaseConnection,
        id: &str,
        new_password: &str,
    ) -> Result<(), AppError> {
        let password_hash = hash_password(new_password)?;
        UserRepo::set_password(db, id, &password_hash)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    /// List all available roles.
    pub async fn list_roles(db: &DatabaseConnection) -> Result<Vec<role::Model>, AppError> {
        role::Entity::find()
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}
