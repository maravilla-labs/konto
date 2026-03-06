use sea_orm::*;

use crate::entities::project_item::{self, ActiveModel, Entity as ProjectItemEntity};

pub struct ProjectItemRepo;

impl ProjectItemRepo {
    pub async fn find_by_project(
        db: &DatabaseConnection,
        project_id: &str,
    ) -> Result<Vec<project_item::Model>, DbErr> {
        ProjectItemEntity::find()
            .filter(project_item::Column::ProjectId.eq(project_id))
            .order_by_asc(project_item::Column::SortOrder)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<project_item::Model>, DbErr> {
        ProjectItemEntity::find_by_id(id).one(db).await
    }

    pub async fn find_children(
        db: &DatabaseConnection,
        parent_id: &str,
    ) -> Result<Vec<project_item::Model>, DbErr> {
        ProjectItemEntity::find()
            .filter(project_item::Column::ParentId.eq(parent_id))
            .order_by_asc(project_item::Column::SortOrder)
            .all(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<project_item::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<project_item::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        ProjectItemEntity::delete_by_id(id).exec(db).await
    }
}
