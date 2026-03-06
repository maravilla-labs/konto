use sea_orm_migration::prelude::*;

use crate::m20240101_000002_create_accounting::{Accounts, JournalEntries, VatRates};
use crate::m20240101_000003_create_contacts::Contacts;
use crate::m20240101_000001_create_tables::Currencies;
use crate::m20240101_000010_create_invoices::Invoices;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CreditNotes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CreditNotes::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(CreditNotes::CreditNoteNumber).string().null().unique_key())
                    .col(ColumnDef::new(CreditNotes::InvoiceId).string().null())
                    .col(ColumnDef::new(CreditNotes::ContactId).string().not_null())
                    .col(
                        ColumnDef::new(CreditNotes::Status)
                            .string()
                            .not_null()
                            .default("draft"),
                    )
                    .col(ColumnDef::new(CreditNotes::IssueDate).date().not_null())
                    .col(ColumnDef::new(CreditNotes::CurrencyId).string().null())
                    .col(ColumnDef::new(CreditNotes::Subtotal).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(CreditNotes::VatAmount).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(CreditNotes::Total).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(CreditNotes::Notes).text().null())
                    .col(ColumnDef::new(CreditNotes::JournalEntryId).string().null())
                    .col(ColumnDef::new(CreditNotes::CreatedBy).string().null())
                    .col(ColumnDef::new(CreditNotes::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(CreditNotes::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(CreditNotes::Table, CreditNotes::InvoiceId)
                            .to(Invoices::Table, Invoices::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CreditNotes::Table, CreditNotes::ContactId)
                            .to(Contacts::Table, Contacts::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CreditNotes::Table, CreditNotes::CurrencyId)
                            .to(Currencies::Table, Currencies::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CreditNotes::Table, CreditNotes::JournalEntryId)
                            .to(JournalEntries::Table, JournalEntries::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CreditNoteLines::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CreditNoteLines::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(CreditNoteLines::CreditNoteId).string().not_null())
                    .col(ColumnDef::new(CreditNoteLines::SortOrder).integer().not_null())
                    .col(ColumnDef::new(CreditNoteLines::Description).text().not_null())
                    .col(ColumnDef::new(CreditNoteLines::Quantity).decimal().not_null())
                    .col(ColumnDef::new(CreditNoteLines::UnitPrice).decimal().not_null())
                    .col(ColumnDef::new(CreditNoteLines::VatRateId).string().null())
                    .col(ColumnDef::new(CreditNoteLines::VatAmount).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(CreditNoteLines::LineTotal).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(CreditNoteLines::AccountId).string().not_null())
                    .col(ColumnDef::new(CreditNoteLines::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(CreditNoteLines::Table, CreditNoteLines::CreditNoteId)
                            .to(CreditNotes::Table, CreditNotes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CreditNoteLines::Table, CreditNoteLines::VatRateId)
                            .to(VatRates::Table, VatRates::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CreditNoteLines::Table, CreditNoteLines::AccountId)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CreditNoteLines::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CreditNotes::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum CreditNotes {
    Table,
    Id,
    CreditNoteNumber,
    InvoiceId,
    ContactId,
    Status,
    IssueDate,
    CurrencyId,
    Subtotal,
    VatAmount,
    Total,
    Notes,
    JournalEntryId,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum CreditNoteLines {
    Table,
    Id,
    CreditNoteId,
    SortOrder,
    Description,
    Quantity,
    UnitPrice,
    VatRateId,
    VatAmount,
    LineTotal,
    AccountId,
    CreatedAt,
}
