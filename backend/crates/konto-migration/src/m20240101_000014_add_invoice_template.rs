use sea_orm_migration::prelude::*;

use crate::m20240101_000010_create_invoices::Invoices;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite only supports one ADD COLUMN per ALTER TABLE, and no ADD FK
        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .add_column(ColumnDef::new(InvoiceExtra::TemplateId).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .add_column(ColumnDef::new(InvoiceExtra::ContentJson).text().null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .drop_column(InvoiceExtra::ContentJson)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .drop_column(InvoiceExtra::TemplateId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum InvoiceExtra {
    TemplateId,
    ContentJson,
}
