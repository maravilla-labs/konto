use sea_orm::*;

use crate::entities::project_activity_type::{self, ActiveModel, Entity as ProjectActivityTypeEntity};

pub struct ProjectActivityTypeRepo;

impl ProjectActivityTypeRepo {
    pub async fn find_by_project(
        db: &DatabaseConnection,
        project_id: &str,
    ) -> Result<Vec<project_activity_type::Model>, DbErr> {
        ProjectActivityTypeEntity::find()
            .filter(project_activity_type::Column::ProjectId.eq(project_id))
            .order_by_asc(project_activity_type::Column::CreatedAt)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<project_activity_type::Model>, DbErr> {
        ProjectActivityTypeEntity::find_by_id(id).one(db).await
    }

    pub async fn find_by_project_and_activity_type(
        db: &DatabaseConnection,
        project_id: &str,
        activity_type_id: &str,
    ) -> Result<Option<project_activity_type::Model>, DbErr> {
        ProjectActivityTypeEntity::find()
            .filter(project_activity_type::Column::ProjectId.eq(project_id))
            .filter(project_activity_type::Column::ActivityTypeId.eq(activity_type_id))
            .one(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<project_activity_type::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<project_activity_type::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        ProjectActivityTypeEntity::delete_by_id(id).exec(db).await
    }
}
