use sea_orm::*;

use crate::entities::contact_tag::{self, Entity as TagEntity};
use crate::entities::contact_tag_assignment::{self, Entity as AssignEntity};

pub struct ContactTagRepo;

impl ContactTagRepo {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<contact_tag::Model>, DbErr> {
        TagEntity::find()
            .order_by_asc(contact_tag::Column::Name)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<contact_tag::Model>, DbErr> {
        TagEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: contact_tag::ActiveModel,
    ) -> Result<contact_tag::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        TagEntity::delete_by_id(id).exec(db).await
    }

    pub async fn find_tags_for_contact(
        db: &DatabaseConnection,
        contact_id: &str,
    ) -> Result<Vec<contact_tag::Model>, DbErr> {
        let assignments = AssignEntity::find()
            .filter(contact_tag_assignment::Column::ContactId.eq(contact_id))
            .all(db)
            .await?;

        let tag_ids: Vec<String> = assignments.into_iter().map(|a| a.tag_id).collect();
        if tag_ids.is_empty() {
            return Ok(vec![]);
        }

        TagEntity::find()
            .filter(contact_tag::Column::Id.is_in(tag_ids))
            .order_by_asc(contact_tag::Column::Name)
            .all(db)
            .await
    }

    pub async fn assign_tag(
        db: &DatabaseConnection,
        contact_id: &str,
        tag_id: &str,
    ) -> Result<(), DbErr> {
        let model = contact_tag_assignment::ActiveModel {
            contact_id: Set(contact_id.to_string()),
            tag_id: Set(tag_id.to_string()),
        };
        // Use insert, ignore duplicate
        match model.insert(db).await {
            Ok(_) => Ok(()),
            Err(DbErr::Exec(RuntimeErr::SqlxError(e)))
                if e.to_string().contains("UNIQUE") =>
            {
                Ok(()) // Already assigned
            }
            Err(e) => Err(e),
        }
    }

    pub async fn remove_tag(
        db: &DatabaseConnection,
        contact_id: &str,
        tag_id: &str,
    ) -> Result<DeleteResult, DbErr> {
        AssignEntity::delete_many()
            .filter(contact_tag_assignment::Column::ContactId.eq(contact_id))
            .filter(contact_tag_assignment::Column::TagId.eq(tag_id))
            .exec(db)
            .await
    }
}
