use sea_orm::*;

use crate::entities::expense::{self, Entity as ExpenseEntity};

pub struct ExpenseRepo;

impl ExpenseRepo {
    #[allow(clippy::too_many_arguments)]
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        status_filter: Option<&str>,
        category_filter: Option<&str>,
        contact_filter: Option<&str>,
        date_from: Option<&str>,
        date_to: Option<&str>,
        search: Option<&str>,
    ) -> Result<(Vec<expense::Model>, u64), DbErr> {
        let mut query = ExpenseEntity::find().order_by_desc(expense::Column::ExpenseDate);

        if let Some(status) = status_filter {
            query = query.filter(expense::Column::Status.eq(status));
        }
        if let Some(category_id) = category_filter {
            query = query.filter(expense::Column::CategoryId.eq(category_id));
        }
        if let Some(contact_id) = contact_filter {
            query = query.filter(expense::Column::ContactId.eq(contact_id));
        }
        if let Some(from) = date_from {
            query = query.filter(expense::Column::ExpenseDate.gte(from));
        }
        if let Some(to) = date_to {
            query = query.filter(expense::Column::ExpenseDate.lte(to));
        }
        if let Some(search) = search {
            query = query.filter(
                Condition::any()
                    .add(expense::Column::ExpenseNumber.contains(search))
                    .add(expense::Column::Description.contains(search)),
            );
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<expense::Model>, DbErr> {
        ExpenseEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: expense::ActiveModel,
    ) -> Result<expense::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: expense::ActiveModel,
    ) -> Result<expense::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        ExpenseEntity::delete_by_id(id).exec(db).await
    }

    pub async fn next_expense_number(
        db: &DatabaseConnection,
        year: i32,
    ) -> Result<String, DbErr> {
        let prefix = format!("EX-{year}-");
        let count = ExpenseEntity::find()
            .filter(expense::Column::ExpenseNumber.starts_with(&prefix))
            .count(db)
            .await?;
        Ok(format!("EX-{year}-{:03}", count + 1))
    }
}
