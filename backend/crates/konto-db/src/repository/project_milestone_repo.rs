use sea_orm::*;

use crate::entities::project_milestone::{
    self, ActiveModel, Entity as ProjectMilestoneEntity,
};

pub struct ProjectMilestoneRepo;

impl ProjectMilestoneRepo {
    pub async fn find_by_project(
        db: &DatabaseConnection,
        project_id: &str,
    ) -> Result<Vec<project_milestone::Model>, DbErr> {
        ProjectMilestoneEntity::find()
            .filter(project_milestone::Column::ProjectId.eq(project_id))
            .order_by_asc(project_milestone::Column::TargetDate)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<project_milestone::Model>, DbErr> {
        ProjectMilestoneEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<project_milestone::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<project_milestone::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<DeleteResult, DbErr> {
        ProjectMilestoneEntity::delete_by_id(id).exec(db).await
    }
}
