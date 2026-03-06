use sea_orm::*;

use crate::entities::contact_relationship::{
    self, ActiveModel, Column, Entity as ContactRelationshipEntity,
};

pub struct ContactRelationshipRepo;

impl ContactRelationshipRepo {
    pub async fn find_by_contact(
        db: &DatabaseConnection,
        contact_id: &str,
    ) -> Result<Vec<contact_relationship::Model>, DbErr> {
        ContactRelationshipEntity::find()
            .filter(
                Condition::any()
                    .add(Column::PersonContactId.eq(contact_id))
                    .add(Column::OrgContactId.eq(contact_id)),
            )
            .order_by_desc(Column::IsPrimary)
            .order_by_asc(Column::CreatedAt)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<contact_relationship::Model>, DbErr> {
        ContactRelationshipEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<contact_relationship::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<contact_relationship::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        ContactRelationshipEntity::delete_by_id(id).exec(db).await
    }
}
