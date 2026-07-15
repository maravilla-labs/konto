use sea_orm_migration::prelude::*;

use crate::m20240101_000002_create_accounting::JournalEntries;
use crate::m20240101_000011_create_settings::BankAccounts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CurrencyExchanges::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CurrencyExchanges::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CurrencyExchanges::FromBankAccountId).string().not_null())
                    .col(ColumnDef::new(CurrencyExchanges::ToBankAccountId).string().not_null())
                    .col(
                        ColumnDef::new(CurrencyExchanges::FromAmount)
                            .decimal_len(15, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CurrencyExchanges::ToAmount)
                            .decimal_len(15, 2)
                            .not_null(),
                    )
                    .col(ColumnDef::new(CurrencyExchanges::Date).date().not_null())
                    .col(ColumnDef::new(CurrencyExchanges::JournalEntryId).string().not_null())
                    .col(ColumnDef::new(CurrencyExchanges::Notes).text().null())
                    .col(ColumnDef::new(CurrencyExchanges::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(CurrencyExchanges::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(CurrencyExchanges::Table, CurrencyExchanges::FromBankAccountId)
                            .to(BankAccounts::Table, BankAccounts::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CurrencyExchanges::Table, CurrencyExchanges::ToBankAccountId)
                            .to(BankAccounts::Table, BankAccounts::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CurrencyExchanges::Table, CurrencyExchanges::JournalEntryId)
                            .to(JournalEntries::Table, JournalEntries::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CurrencyExchanges::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum CurrencyExchanges {
    Table,
    Id,
    FromBankAccountId,
    ToBankAccountId,
    FromAmount,
    ToAmount,
    Date,
    JournalEntryId,
    Notes,
    CreatedAt,
    UpdatedAt,
}
