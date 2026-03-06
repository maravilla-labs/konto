use sea_orm::*;

use crate::entities::account::{self, ActiveModel, Entity as AccountEntity};

pub struct AccountRepo;

impl AccountRepo {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<account::Model>, DbErr> {
        AccountEntity::find()
            .order_by_asc(account::Column::Number)
            .all(db)
            .await
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> Result<Option<account::Model>, DbErr> {
        AccountEntity::find_by_id(id).one(db).await
    }

    pub async fn find_by_number(db: &DatabaseConnection, number: i32) -> Result<Option<account::Model>, DbErr> {
        AccountEntity::find()
            .filter(account::Column::Number.eq(number))
            .one(db)
            .await
    }

    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        search: Option<&str>,
    ) -> Result<(Vec<account::Model>, u64), DbErr> {
        let mut query = AccountEntity::find().order_by_asc(account::Column::Number);

        if let Some(search) = search {
            let mut cond = Condition::any().add(account::Column::Name.contains(search));
            if let Ok(num) = search.parse::<i32>() {
                cond = cond.add(account::Column::Number.eq(num));
            }
            query = query.filter(cond);
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn create(db: &DatabaseConnection, model: ActiveModel) -> Result<account::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(db: &DatabaseConnection, model: ActiveModel) -> Result<account::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        AccountEntity::delete_by_id(id).exec(db).await
    }
}
