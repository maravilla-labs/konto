use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::Currencies;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Chart of Accounts
        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Accounts::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Accounts::Number).integer().not_null().unique_key())
                    .col(ColumnDef::new(Accounts::Name).string().not_null())
                    .col(ColumnDef::new(Accounts::AccountType).string().not_null())
                    .col(ColumnDef::new(Accounts::ParentId).string().null())
                    .col(ColumnDef::new(Accounts::CurrencyId).string().null())
                    .col(ColumnDef::new(Accounts::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Accounts::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Accounts::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Accounts::Table, Accounts::ParentId)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Accounts::Table, Accounts::CurrencyId)
                            .to(Currencies::Table, Currencies::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // VAT Rates
        manager
            .create_table(
                Table::create()
                    .table(VatRates::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(VatRates::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(VatRates::Code).string().not_null().unique_key())
                    .col(ColumnDef::new(VatRates::Name).string().not_null())
                    .col(ColumnDef::new(VatRates::Rate).decimal().not_null())
                    .col(ColumnDef::new(VatRates::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(VatRates::ValidFrom).date().null())
                    .col(ColumnDef::new(VatRates::ValidTo).date().null())
                    .to_owned(),
            )
            .await?;

        // Journal Entries (header)
        manager
            .create_table(
                Table::create()
                    .table(JournalEntries::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(JournalEntries::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(JournalEntries::Date).date().not_null())
                    .col(ColumnDef::new(JournalEntries::Reference).string().null())
                    .col(ColumnDef::new(JournalEntries::Description).text().not_null())
                    .col(ColumnDef::new(JournalEntries::Status).string().not_null().default("draft"))
                    .col(ColumnDef::new(JournalEntries::CurrencyId).string().null())
                    .col(ColumnDef::new(JournalEntries::ExchangeRate).decimal().null())
                    .col(ColumnDef::new(JournalEntries::CreatedBy).string().null())
                    .col(ColumnDef::new(JournalEntries::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(JournalEntries::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Journal Lines (detail)
        manager
            .create_table(
                Table::create()
                    .table(JournalLines::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(JournalLines::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(JournalLines::JournalEntryId).string().not_null())
                    .col(ColumnDef::new(JournalLines::AccountId).string().not_null())
                    .col(ColumnDef::new(JournalLines::DebitAmount).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(JournalLines::CreditAmount).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(JournalLines::Description).text().null())
                    .col(ColumnDef::new(JournalLines::VatRateId).string().null())
                    .col(ColumnDef::new(JournalLines::CurrencyAmount).decimal().null())
                    .col(ColumnDef::new(JournalLines::BaseCurrencyAmount).decimal().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(JournalLines::Table, JournalLines::JournalEntryId)
                            .to(JournalEntries::Table, JournalEntries::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(JournalLines::Table, JournalLines::AccountId)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(JournalLines::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(JournalEntries::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(VatRates::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Accounts::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Accounts {
    Table,
    Id,
    Number,
    Name,
    AccountType,
    ParentId,
    CurrencyId,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum VatRates {
    Table,
    Id,
    Code,
    Name,
    Rate,
    IsActive,
    ValidFrom,
    ValidTo,
}

#[derive(DeriveIden)]
pub enum JournalEntries {
    Table,
    Id,
    Date,
    Reference,
    Description,
    Status,
    CurrencyId,
    ExchangeRate,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum JournalLines {
    Table,
    Id,
    JournalEntryId,
    AccountId,
    DebitAmount,
    CreditAmount,
    Description,
    VatRateId,
    CurrencyAmount,
    BaseCurrencyAmount,
}
