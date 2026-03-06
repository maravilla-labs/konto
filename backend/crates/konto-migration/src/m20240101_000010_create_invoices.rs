use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::Currencies;
use crate::m20240101_000002_create_accounting::{Accounts, JournalEntries, VatRates};
use crate::m20240101_000003_create_contacts::Contacts;
use crate::m20240101_000004_create_projects::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Invoices (header)
        manager
            .create_table(
                Table::create()
                    .table(Invoices::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Invoices::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Invoices::InvoiceNumber).string().null().unique_key())
                    .col(ColumnDef::new(Invoices::ContactId).string().not_null())
                    .col(ColumnDef::new(Invoices::ProjectId).string().null())
                    .col(
                        ColumnDef::new(Invoices::Status)
                            .string()
                            .not_null()
                            .default("draft"),
                    )
                    .col(ColumnDef::new(Invoices::IssueDate).date().not_null())
                    .col(ColumnDef::new(Invoices::DueDate).date().not_null())
                    .col(ColumnDef::new(Invoices::CurrencyId).string().null())
                    .col(ColumnDef::new(Invoices::Subtotal).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(Invoices::VatAmount).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(Invoices::Total).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(Invoices::Notes).text().null())
                    .col(ColumnDef::new(Invoices::PaymentTerms).text().null())
                    .col(ColumnDef::new(Invoices::JournalEntryId).string().null())
                    .col(ColumnDef::new(Invoices::PaymentJournalEntryId).string().null())
                    .col(ColumnDef::new(Invoices::CreatedBy).string().null())
                    .col(ColumnDef::new(Invoices::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Invoices::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Invoices::Table, Invoices::ContactId)
                            .to(Contacts::Table, Contacts::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Invoices::Table, Invoices::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Invoices::Table, Invoices::CurrencyId)
                            .to(Currencies::Table, Currencies::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Invoices::Table, Invoices::JournalEntryId)
                            .to(JournalEntries::Table, JournalEntries::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Invoices::Table, Invoices::PaymentJournalEntryId)
                            .to(JournalEntries::Table, JournalEntries::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Invoice Lines (detail)
        manager
            .create_table(
                Table::create()
                    .table(InvoiceLines::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(InvoiceLines::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(InvoiceLines::InvoiceId).string().not_null())
                    .col(ColumnDef::new(InvoiceLines::Position).integer().not_null())
                    .col(ColumnDef::new(InvoiceLines::Description).text().not_null())
                    .col(ColumnDef::new(InvoiceLines::Quantity).decimal().not_null())
                    .col(ColumnDef::new(InvoiceLines::UnitPrice).decimal().not_null())
                    .col(ColumnDef::new(InvoiceLines::VatRateId).string().null())
                    .col(ColumnDef::new(InvoiceLines::VatAmount).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(InvoiceLines::LineTotal).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(InvoiceLines::AccountId).string().not_null())
                    .col(ColumnDef::new(InvoiceLines::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(InvoiceLines::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(InvoiceLines::Table, InvoiceLines::InvoiceId)
                            .to(Invoices::Table, Invoices::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(InvoiceLines::Table, InvoiceLines::VatRateId)
                            .to(VatRates::Table, VatRates::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(InvoiceLines::Table, InvoiceLines::AccountId)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(InvoiceLines::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Invoices::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Invoices {
    Table,
    Id,
    InvoiceNumber,
    ContactId,
    ProjectId,
    Status,
    IssueDate,
    DueDate,
    CurrencyId,
    Subtotal,
    VatAmount,
    Total,
    Notes,
    PaymentTerms,
    JournalEntryId,
    PaymentJournalEntryId,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum InvoiceLines {
    Table,
    Id,
    InvoiceId,
    Position,
    Description,
    Quantity,
    UnitPrice,
    VatRateId,
    VatAmount,
    LineTotal,
    AccountId,
    CreatedAt,
    UpdatedAt,
}
