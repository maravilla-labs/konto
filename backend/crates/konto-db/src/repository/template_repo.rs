use sea_orm::*;

use crate::entities::document_template::{self, Column, Entity as TemplateEntity};

pub struct TemplateRepo;

impl TemplateRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        template_type: Option<&str>,
        search: Option<&str>,
    ) -> Result<(Vec<document_template::Model>, u64), DbErr> {
        let mut query = TemplateEntity::find().order_by_desc(Column::UpdatedAt);

        if let Some(tt) = template_type {
            query = query.filter(Column::TemplateType.eq(tt));
        }

        if let Some(search) = search {
            query = query.filter(Column::Name.contains(search));
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<document_template::Model>, DbErr> {
        TemplateEntity::find_by_id(id).one(db).await
    }

    pub async fn find_default_by_type(
        db: &DatabaseConnection,
        template_type: &str,
    ) -> Result<Option<document_template::Model>, DbErr> {
        TemplateEntity::find()
            .filter(Column::TemplateType.eq(template_type))
            .filter(Column::IsDefault.eq(true))
            .one(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: document_template::ActiveModel,
    ) -> Result<document_template::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: document_template::ActiveModel,
    ) -> Result<document_template::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        TemplateEntity::delete_by_id(id).exec(db).await
    }

    /// Clear is_default for all templates of a given type.
    pub async fn clear_defaults_for_type(
        db: &DatabaseConnection,
        template_type: &str,
    ) -> Result<(), DbErr> {
        TemplateEntity::update_many()
            .col_expr(Column::IsDefault, sea_orm::prelude::Expr::value(false))
            .filter(Column::TemplateType.eq(template_type))
            .filter(Column::IsDefault.eq(true))
            .exec(db)
            .await?;
        Ok(())
    }
}
