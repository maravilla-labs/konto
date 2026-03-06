use sea_orm::*;

use crate::entities::invoice::{self, Entity as InvoiceEntity};
use crate::entities::invoice_line::{self, Entity as InvoiceLineEntity};

pub struct InvoiceRepo;

impl InvoiceRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        status_filter: Option<&str>,
        contact_filter: Option<&str>,
        project_filter: Option<&str>,
        search: Option<&str>,
    ) -> Result<(Vec<invoice::Model>, u64), DbErr> {
        let mut query = InvoiceEntity::find().order_by_desc(invoice::Column::IssueDate);

        if let Some(status) = status_filter {
            query = query.filter(invoice::Column::Status.eq(status));
        }

        if let Some(contact_id) = contact_filter {
            query = query.filter(invoice::Column::ContactId.eq(contact_id));
        }

        if let Some(project_id) = project_filter {
            query = query.filter(invoice::Column::ProjectId.eq(project_id));
        }

        if let Some(search) = search {
            query = query.filter(
                Condition::any()
                    .add(invoice::Column::InvoiceNumber.contains(search))
                    .add(invoice::Column::Notes.contains(search)),
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
    ) -> Result<Option<invoice::Model>, DbErr> {
        InvoiceEntity::find_by_id(id).one(db).await
    }

    pub async fn find_lines_by_invoice(
        db: &DatabaseConnection,
        invoice_id: &str,
    ) -> Result<Vec<invoice_line::Model>, DbErr> {
        InvoiceLineEntity::find()
            .filter(invoice_line::Column::InvoiceId.eq(invoice_id))
            .order_by_asc(invoice_line::Column::Position)
            .all(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: invoice::ActiveModel,
    ) -> Result<invoice::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: invoice::ActiveModel,
    ) -> Result<invoice::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        InvoiceEntity::delete_by_id(id).exec(db).await
    }

    pub async fn create_line(
        db: &DatabaseConnection,
        model: invoice_line::ActiveModel,
    ) -> Result<invoice_line::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn delete_lines_by_invoice(
        db: &DatabaseConnection,
        invoice_id: &str,
    ) -> Result<DeleteResult, DbErr> {
        InvoiceLineEntity::delete_many()
            .filter(invoice_line::Column::InvoiceId.eq(invoice_id))
            .exec(db)
            .await
    }

    /// Find all invoices matching any of the given statuses.
    pub async fn find_by_status_list(
        db: &DatabaseConnection,
        statuses: &[&str],
    ) -> Result<Vec<invoice::Model>, DbErr> {
        InvoiceEntity::find()
            .filter(invoice::Column::Status.is_in(statuses.iter().map(ToString::to_string)))
            .order_by_asc(invoice::Column::DueDate)
            .all(db)
            .await
    }

    pub async fn next_invoice_number(
        db: &DatabaseConnection,
        year: i32,
    ) -> Result<String, DbErr> {
        let prefix = format!("RE-{year}-");
        let count = InvoiceEntity::find()
            .filter(invoice::Column::InvoiceNumber.starts_with(&prefix))
            .count(db)
            .await?;
        Ok(format!("RE-{year}-{:03}", count + 1))
    }
}
