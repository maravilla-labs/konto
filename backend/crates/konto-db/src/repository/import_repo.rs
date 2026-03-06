use sea_orm::*;

use crate::entities::import_job::{self, ActiveModel, Entity as ImportJobEntity};

pub struct ImportRepo;

impl ImportRepo {
    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> Result<Option<import_job::Model>, DbErr> {
        ImportJobEntity::find_by_id(id).one(db).await
    }

    pub async fn create(db: &DatabaseConnection, model: ActiveModel) -> Result<import_job::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(db: &DatabaseConnection, model: ActiveModel) -> Result<import_job::Model, DbErr> {
        model.update(db).await
    }
}
