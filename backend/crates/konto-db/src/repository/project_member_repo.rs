use sea_orm::*;

use crate::entities::project_member::{self, ActiveModel, Entity as ProjectMemberEntity};

pub struct ProjectMemberRepo;

impl ProjectMemberRepo {
    pub async fn find_by_project(
        db: &DatabaseConnection,
        project_id: &str,
    ) -> Result<Vec<project_member::Model>, DbErr> {
        ProjectMemberEntity::find()
            .filter(project_member::Column::ProjectId.eq(project_id))
            .order_by_asc(project_member::Column::JoinedAt)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<project_member::Model>, DbErr> {
        ProjectMemberEntity::find_by_id(id).one(db).await
    }

    pub async fn find_by_project_and_user(
        db: &DatabaseConnection,
        project_id: &str,
        user_id: &str,
    ) -> Result<Option<project_member::Model>, DbErr> {
        ProjectMemberEntity::find()
            .filter(project_member::Column::ProjectId.eq(project_id))
            .filter(project_member::Column::UserId.eq(user_id))
            .one(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<project_member::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<project_member::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        ProjectMemberEntity::delete_by_id(id).exec(db).await
    }
}
