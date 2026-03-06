use sea_orm::*;

use crate::entities::contact::{self, ActiveModel, Entity as ContactEntity};

pub struct ContactRepo;

impl ContactRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        search: Option<&str>,
        category: Option<&str>,
    ) -> Result<(Vec<contact::Model>, u64), DbErr> {
        let mut query = ContactEntity::find().order_by_desc(contact::Column::UpdatedAt);

        if let Some(search) = search {
            query = query.filter(
                Condition::any()
                    .add(contact::Column::Name1.contains(search))
                    .add(contact::Column::Name2.contains(search))
                    .add(contact::Column::Email.contains(search))
                    .add(contact::Column::City.contains(search)),
            );
        }

        if let Some(cat) = category {
            query = query.filter(contact::Column::Category.eq(cat));
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> Result<Option<contact::Model>, DbErr> {
        ContactEntity::find_by_id(id).one(db).await
    }

    pub async fn create(db: &DatabaseConnection, model: ActiveModel) -> Result<contact::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(db: &DatabaseConnection, model: ActiveModel) -> Result<contact::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        ContactEntity::delete_by_id(id).exec(db).await
    }
}
