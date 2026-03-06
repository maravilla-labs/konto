use sea_orm::*;

use crate::entities::project_document;

pub struct ProjectDocumentRepo;

impl ProjectDocumentRepo {
    pub async fn find_by_project(
        db: &DatabaseConnection,
        project_id: &str,
    ) -> Result<Vec<project_document::Model>, DbErr> {
        project_document::Entity::find()
            .filter(project_document::Column::ProjectId.eq(project_id))
            .order_by_asc(project_document::Column::CreatedAt)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<project_document::Model>, DbErr> {
        project_document::Entity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: project_document::ActiveModel,
    ) -> Result<project_document::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        project_document::Entity::delete_by_id(id).exec(db).await
    }
}
