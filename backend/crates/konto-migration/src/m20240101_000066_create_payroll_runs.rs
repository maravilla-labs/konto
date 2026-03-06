use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PayrollRuns::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PayrollRuns::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PayrollRuns::PeriodMonth).integer().not_null())
                    .col(ColumnDef::new(PayrollRuns::PeriodYear).integer().not_null())
                    .col(
                        ColumnDef::new(PayrollRuns::Status)
                            .string()
                            .not_null()
                            .default("draft"),
                    )
                    .col(ColumnDef::new(PayrollRuns::RunDate).date().not_null())
                    .col(ColumnDef::new(PayrollRuns::ApprovedBy).string().null())
                    .col(ColumnDef::new(PayrollRuns::ApprovedAt).timestamp().null())
                    .col(ColumnDef::new(PayrollRuns::PaidAt).timestamp().null())
                    .col(ColumnDef::new(PayrollRuns::JournalEntryId).string().null())
                    .col(
                        ColumnDef::new(PayrollRuns::PaymentFileGenerated)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(PayrollRuns::TotalGross)
                            .decimal_len(15, 2)
                            .not_null()
                            .default("0.00"),
                    )
                    .col(
                        ColumnDef::new(PayrollRuns::TotalNet)
                            .decimal_len(15, 2)
                            .not_null()
                            .default("0.00"),
                    )
                    .col(
                        ColumnDef::new(PayrollRuns::TotalEmployerCost)
                            .decimal_len(15, 2)
                            .not_null()
                            .default("0.00"),
                    )
                    .col(ColumnDef::new(PayrollRuns::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(PayrollRuns::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Unique constraint on (period_month, period_year)
        manager
            .create_index(
                Index::create()
                    .name("idx_payroll_runs_period")
                    .table(PayrollRuns::Table)
                    .col(PayrollRuns::PeriodMonth)
                    .col(PayrollRuns::PeriodYear)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PayrollRuns::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum PayrollRuns {
    Table,
    Id,
    PeriodMonth,
    PeriodYear,
    Status,
    RunDate,
    ApprovedBy,
    ApprovedAt,
    PaidAt,
    JournalEntryId,
    PaymentFileGenerated,
    TotalGross,
    TotalNet,
    TotalEmployerCost,
    CreatedAt,
    UpdatedAt,
}
