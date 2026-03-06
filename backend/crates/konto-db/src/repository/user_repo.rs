use sea_orm::*;

use crate::entities::user::{self, Entity as UserEntity};

pub struct UserRepo;

impl UserRepo {
    pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> Result<Option<user::Model>, DbErr> {
        UserEntity::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> Result<Option<user::Model>, DbErr> {
        UserEntity::find_by_id(id).one(db).await
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<user::Model>, DbErr> {
        UserEntity::find().all(db).await
    }

    pub async fn create(db: &DatabaseConnection, model: user::ActiveModel) -> Result<user::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(db: &DatabaseConnection, model: user::ActiveModel) -> Result<user::Model, DbErr> {
        model.update(db).await
    }

    pub async fn set_password(db: &DatabaseConnection, id: &str, password_hash: &str) -> Result<user::Model, DbErr> {
        let user = UserEntity::find_by_id(id).one(db).await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;
        let now = chrono::Utc::now().naive_utc();
        let mut model: user::ActiveModel = user.into();
        model.password_hash = Set(password_hash.to_string());
        model.updated_at = Set(now);
        model.update(db).await
    }

    pub async fn set_active(db: &DatabaseConnection, id: &str, is_active: bool) -> Result<user::Model, DbErr> {
        let user = UserEntity::find_by_id(id).one(db).await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;
        let now = chrono::Utc::now().naive_utc();
        let mut model: user::ActiveModel = user.into();
        model.is_active = Set(is_active);
        model.updated_at = Set(now);
        model.update(db).await
    }
}
