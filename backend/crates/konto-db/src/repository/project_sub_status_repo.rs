use sea_orm::*;

use crate::entities::project_sub_status::{self, Entity as ProjectSubStatus};

pub struct ProjectSubStatusRepo;

impl ProjectSubStatusRepo {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<project_sub_status::Model>, DbErr> {
        ProjectSubStatus::find()
            .order_by_asc(project_sub_status::Column::SortOrder)
            .all(db)
            .await
    }

    pub async fn find_active(db: &DatabaseConnection) -> Result<Vec<project_sub_status::Model>, DbErr> {
        ProjectSubStatus::find()
            .filter(project_sub_status::Column::IsActive.eq(true))
            .order_by_asc(project_sub_status::Column::SortOrder)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<project_sub_status::Model>, DbErr> {
        ProjectSubStatus::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: project_sub_status::ActiveModel,
    ) -> Result<project_sub_status::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: project_sub_status::ActiveModel,
    ) -> Result<project_sub_status::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        ProjectSubStatus::delete_by_id(id).exec(db).await
    }
}
