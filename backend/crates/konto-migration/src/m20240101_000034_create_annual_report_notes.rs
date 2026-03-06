use sea_orm_migration::prelude::*;

use crate::m20240101_000008_create_fiscal_exchange::FiscalYears;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AnnualReportNotes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AnnualReportNotes::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AnnualReportNotes::FiscalYearId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AnnualReportNotes::SectionKey)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AnnualReportNotes::ContentJson)
                            .text()
                            .not_null()
                            .default("{}"),
                    )
                    .col(
                        ColumnDef::new(AnnualReportNotes::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AnnualReportNotes::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                AnnualReportNotes::Table,
                                AnnualReportNotes::FiscalYearId,
                            )
                            .to(FiscalYears::Table, FiscalYears::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Add unique index
        manager
            .create_index(
                Index::create()
                    .name("idx_annual_report_notes_fy_section")
                    .table(AnnualReportNotes::Table)
                    .col(AnnualReportNotes::FiscalYearId)
                    .col(AnnualReportNotes::SectionKey)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AnnualReportNotes::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum AnnualReportNotes {
    Table,
    Id,
    FiscalYearId,
    SectionKey,
    ContentJson,
    CreatedAt,
    UpdatedAt,
}
