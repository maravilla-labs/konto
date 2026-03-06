use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::Users;
use crate::m20240101_000008_create_fiscal_exchange::FiscalYears;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AnnualReports::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AnnualReports::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AnnualReports::FiscalYearId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(AnnualReports::Status)
                            .string()
                            .not_null()
                            .default("draft"),
                    )
                    .col(ColumnDef::new(AnnualReports::GeneratedAt).timestamp().null())
                    .col(ColumnDef::new(AnnualReports::GeneratedBy).string().null())
                    .col(ColumnDef::new(AnnualReports::PdfPath).string().null())
                    .col(
                        ColumnDef::new(AnnualReports::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AnnualReports::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AnnualReports::Table, AnnualReports::FiscalYearId)
                            .to(FiscalYears::Table, FiscalYears::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AnnualReports::Table, AnnualReports::GeneratedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AnnualReports::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum AnnualReports {
    Table,
    Id,
    FiscalYearId,
    Status,
    GeneratedAt,
    GeneratedBy,
    PdfPath,
    CreatedAt,
    UpdatedAt,
}
