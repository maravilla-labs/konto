use konto_common::error::AppError;
use konto_db::entities::{employee, user};
use konto_db::repository::employee_repo::EmployeeRepo;
use konto_db::repository::user_repo::UserRepo;
use rand::Rng;
use sea_orm::{DatabaseConnection, Set};

use super::user_service::UserService;

pub struct EmployeeUserLinkService;

pub struct ProvisionedUser {
    pub user_id: String,
    pub temp_password: String,
}

impl EmployeeUserLinkService {
    /// Create a new user account for an employee with a temporary password.
    pub async fn provision_user(
        db: &DatabaseConnection,
        employee: &employee::Model,
        role_id: &str,
    ) -> Result<ProvisionedUser, AppError> {
        let email = employee.email.as_deref().ok_or_else(|| {
            AppError::BadRequest("Employee must have an email to create a user account".into())
        })?;

        let temp_password = generate_temp_password();
        let full_name = format!("{} {}", employee.first_name, employee.last_name);

        let new_user = UserService::create(db, email, &temp_password, &full_name, role_id, None).await?;

        // Set bidirectional links
        Self::set_links(db, &employee.id, &new_user.id).await?;

        Ok(ProvisionedUser {
            user_id: new_user.id,
            temp_password,
        })
    }

    /// Link an existing employee and user bidirectionally.
    pub async fn link(
        db: &DatabaseConnection,
        employee_id: &str,
        user_id: &str,
    ) -> Result<(), AppError> {
        Self::set_links(db, employee_id, user_id).await
    }

    /// Unlink employee and user (clear both FKs).
    #[allow(clippy::collapsible_if)]
    pub async fn unlink(
        db: &DatabaseConnection,
        employee_id: &str,
    ) -> Result<(), AppError> {
        let emp = EmployeeRepo::find_by_id(db, employee_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Employee not found".into()))?;

        // Clear user.employee_id if linked
        if let Some(ref uid) = emp.user_id {
            if let Some(u) = UserRepo::find_by_id(db, uid)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
            {
                let mut user_model: user::ActiveModel = u.into();
                user_model.employee_id = Set(None);
                UserRepo::update(db, user_model)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?;
            }
        }

        // Clear employee.user_id
        let mut emp_model: employee::ActiveModel = emp.into();
        emp_model.user_id = Set(None);
        EmployeeRepo::update(db, emp_model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn set_links(
        db: &DatabaseConnection,
        employee_id: &str,
        user_id: &str,
    ) -> Result<(), AppError> {
        // Set employee.user_id
        let emp = EmployeeRepo::find_by_id(db, employee_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Employee not found".into()))?;

        let mut emp_model: employee::ActiveModel = emp.into();
        emp_model.user_id = Set(Some(user_id.to_string()));
        EmployeeRepo::update(db, emp_model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Set user.employee_id
        let usr = UserRepo::find_by_id(db, user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        let mut user_model: user::ActiveModel = usr.into();
        user_model.employee_id = Set(Some(employee_id.to_string()));
        UserRepo::update(db, user_model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}

fn generate_temp_password() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz23456789!@#$";
    let mut rng = rand::rng();
    (0..16)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
