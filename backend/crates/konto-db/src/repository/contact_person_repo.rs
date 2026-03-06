use sea_orm::*;

use crate::entities::contact_person::{self, Entity as PersonEntity};

pub struct ContactPersonRepo;

impl ContactPersonRepo {
    pub async fn find_by_contact_id(
        db: &DatabaseConnection,
        contact_id: &str,
    ) -> Result<Vec<contact_person::Model>, DbErr> {
        PersonEntity::find()
            .filter(contact_person::Column::ContactId.eq(contact_id))
            .order_by_asc(contact_person::Column::LastName)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<contact_person::Model>, DbErr> {
        PersonEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: contact_person::ActiveModel,
    ) -> Result<contact_person::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: contact_person::ActiveModel,
    ) -> Result<contact_person::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        PersonEntity::delete_by_id(id).exec(db).await
    }
}
