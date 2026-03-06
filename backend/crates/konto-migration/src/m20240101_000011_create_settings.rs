use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::Currencies;
use crate::m20240101_000002_create_accounting::Accounts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Company Settings (singleton)
        manager
            .create_table(
                Table::create()
                    .table(CompanySettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CompanySettings::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CompanySettings::LegalName).string().not_null())
                    .col(ColumnDef::new(CompanySettings::TradeName).string().null())
                    .col(ColumnDef::new(CompanySettings::Street).string().not_null())
                    .col(ColumnDef::new(CompanySettings::PostalCode).string().not_null())
                    .col(ColumnDef::new(CompanySettings::City).string().not_null())
                    .col(
                        ColumnDef::new(CompanySettings::Country)
                            .string()
                            .not_null()
                            .default("CH"),
                    )
                    .col(ColumnDef::new(CompanySettings::Email).string().null())
                    .col(ColumnDef::new(CompanySettings::Phone).string().null())
                    .col(ColumnDef::new(CompanySettings::Website).string().null())
                    .col(ColumnDef::new(CompanySettings::VatNumber).string().null())
                    .col(
                        ColumnDef::new(CompanySettings::VatMethod)
                            .string()
                            .not_null()
                            .default("flat_rate"),
                    )
                    .col(ColumnDef::new(CompanySettings::RegisterNumber).string().null())
                    .col(ColumnDef::new(CompanySettings::LogoUrl).string().null())
                    .col(ColumnDef::new(CompanySettings::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(CompanySettings::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Bank Accounts
        manager
            .create_table(
                Table::create()
                    .table(BankAccounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BankAccounts::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BankAccounts::Name).string().not_null())
                    .col(ColumnDef::new(BankAccounts::BankName).string().not_null())
                    .col(ColumnDef::new(BankAccounts::Iban).string().not_null())
                    .col(ColumnDef::new(BankAccounts::Bic).string().null())
                    .col(ColumnDef::new(BankAccounts::CurrencyId).string().null())
                    .col(ColumnDef::new(BankAccounts::AccountId).string().null())
                    .col(
                        ColumnDef::new(BankAccounts::IsDefault)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(BankAccounts::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(BankAccounts::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(BankAccounts::Table, BankAccounts::CurrencyId)
                            .to(Currencies::Table, Currencies::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(BankAccounts::Table, BankAccounts::AccountId)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BankAccounts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CompanySettings::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum CompanySettings {
    Table,
    Id,
    LegalName,
    TradeName,
    Street,
    PostalCode,
    City,
    Country,
    Email,
    Phone,
    Website,
    VatNumber,
    VatMethod,
    RegisterNumber,
    LogoUrl,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum BankAccounts {
    Table,
    Id,
    Name,
    BankName,
    Iban,
    Bic,
    CurrencyId,
    AccountId,
    IsDefault,
    CreatedAt,
    UpdatedAt,
}
