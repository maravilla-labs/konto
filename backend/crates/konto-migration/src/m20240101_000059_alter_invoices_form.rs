use sea_orm_migration::prelude::*;

use crate::m20240101_000010_create_invoices::{InvoiceLines, Invoices};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum InvoicesExtra {
    HeaderText,
    FooterText,
    ContactPersonId,
}

#[derive(DeriveIden)]
enum InvoiceLinesExtra {
    DiscountPercent,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite requires one column per ALTER TABLE statement
        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .add_column(ColumnDef::new(InvoicesExtra::HeaderText).text().null())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .add_column(ColumnDef::new(InvoicesExtra::FooterText).text().null())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .add_column(
                        ColumnDef::new(InvoicesExtra::ContactPersonId)
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add discount_percent to invoice_lines
        manager
            .alter_table(
                Table::alter()
                    .table(InvoiceLines::Table)
                    .add_column(
                        ColumnDef::new(InvoiceLinesExtra::DiscountPercent)
                            .decimal_len(5, 2)
                            .null(),
                    )
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
                    .drop_column(InvoicesExtra::HeaderText)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .drop_column(InvoicesExtra::FooterText)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .drop_column(InvoicesExtra::ContactPersonId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(InvoiceLines::Table)
                    .drop_column(InvoiceLinesExtra::DiscountPercent)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
