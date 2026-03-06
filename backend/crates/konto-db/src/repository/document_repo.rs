use sea_orm::*;

use crate::entities::document::{self, Column, Entity as DocumentEntity};
use crate::entities::document_line_item::{self, Entity as LineItemEntity};

pub struct DocumentRepo;

impl DocumentRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        doc_type: Option<&str>,
        status: Option<&str>,
        contact_id: Option<&str>,
        search: Option<&str>,
    ) -> Result<(Vec<document::Model>, u64), DbErr> {
        let mut query = DocumentEntity::find().order_by_desc(Column::UpdatedAt);

        if let Some(dt) = doc_type {
            query = query.filter(Column::DocType.eq(dt));
        }
        if let Some(s) = status {
            query = query.filter(Column::Status.eq(s));
        }
        if let Some(cid) = contact_id {
            query = query.filter(Column::ContactId.eq(cid));
        }
        if let Some(search) = search {
            query = query.filter(
                Condition::any()
                    .add(Column::Title.contains(search))
                    .add(Column::DocNumber.contains(search)),
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
    ) -> Result<Option<document::Model>, DbErr> {
        DocumentEntity::find_by_id(id).one(db).await
    }

    pub async fn find_lines_by_document(
        db: &DatabaseConnection,
        document_id: &str,
    ) -> Result<Vec<document_line_item::Model>, DbErr> {
        LineItemEntity::find()
            .filter(document_line_item::Column::DocumentId.eq(document_id))
            .order_by_asc(document_line_item::Column::Position)
            .all(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: document::ActiveModel,
    ) -> Result<document::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: document::ActiveModel,
    ) -> Result<document::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        DocumentEntity::delete_by_id(id).exec(db).await
    }

    pub async fn create_line(
        db: &DatabaseConnection,
        model: document_line_item::ActiveModel,
    ) -> Result<document_line_item::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn delete_lines_by_document(
        db: &DatabaseConnection,
        document_id: &str,
    ) -> Result<DeleteResult, DbErr> {
        LineItemEntity::delete_many()
            .filter(document_line_item::Column::DocumentId.eq(document_id))
            .exec(db)
            .await
    }

    pub async fn next_doc_number(db: &DatabaseConnection) -> Result<String, DbErr> {
        let prefix = "AN-";
        let count = DocumentEntity::find()
            .filter(Column::DocNumber.starts_with(prefix))
            .count(db)
            .await?;
        Ok(format!("AN-{:05}", count + 1))
    }
}
