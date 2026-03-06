use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::Currencies;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Fiscal Years
        manager
            .create_table(
                Table::create()
                    .table(FiscalYears::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(FiscalYears::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(FiscalYears::Name).string().not_null())
                    .col(ColumnDef::new(FiscalYears::StartDate).date().not_null())
                    .col(ColumnDef::new(FiscalYears::EndDate).date().not_null())
                    .col(ColumnDef::new(FiscalYears::Status).string().not_null().default("open"))
                    .col(ColumnDef::new(FiscalYears::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(FiscalYears::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Fiscal Periods
        manager
            .create_table(
                Table::create()
                    .table(FiscalPeriods::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(FiscalPeriods::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(FiscalPeriods::FiscalYearId).string().not_null())
                    .col(ColumnDef::new(FiscalPeriods::Name).string().not_null())
                    .col(ColumnDef::new(FiscalPeriods::StartDate).date().null())
                    .col(ColumnDef::new(FiscalPeriods::EndDate).date().null())
                    .col(ColumnDef::new(FiscalPeriods::PeriodNumber).integer().null())
                    .col(ColumnDef::new(FiscalPeriods::Status).string().not_null().default("open"))
                    .col(ColumnDef::new(FiscalPeriods::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(FiscalPeriods::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(FiscalPeriods::Table, FiscalPeriods::FiscalYearId)
                            .to(FiscalYears::Table, FiscalYears::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Exchange Rates
        manager
            .create_table(
                Table::create()
                    .table(ExchangeRates::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ExchangeRates::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(ExchangeRates::FromCurrencyId).string().not_null())
                    .col(ColumnDef::new(ExchangeRates::ToCurrencyId).string().not_null())
                    .col(
                        ColumnDef::new(ExchangeRates::Rate)
                            .decimal_len(10, 6)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ExchangeRates::ValidDate).date().not_null())
                    .col(ColumnDef::new(ExchangeRates::Source).string().null())
                    .col(ColumnDef::new(ExchangeRates::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(ExchangeRates::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ExchangeRates::Table, ExchangeRates::FromCurrencyId)
                            .to(Currencies::Table, Currencies::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ExchangeRates::Table, ExchangeRates::ToCurrencyId)
                            .to(Currencies::Table, Currencies::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ExchangeRates::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(FiscalPeriods::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(FiscalYears::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum FiscalYears {
    Table,
    Id,
    Name,
    StartDate,
    EndDate,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum FiscalPeriods {
    Table,
    Id,
    FiscalYearId,
    Name,
    StartDate,
    EndDate,
    PeriodNumber,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum ExchangeRates {
    Table,
    Id,
    FromCurrencyId,
    ToCurrencyId,
    Rate,
    ValidDate,
    Source,
    CreatedAt,
    UpdatedAt,
}
