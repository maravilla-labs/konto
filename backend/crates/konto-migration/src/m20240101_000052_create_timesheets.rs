use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Timesheets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Timesheets::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Timesheets::UserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Timesheets::PeriodStart)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Timesheets::PeriodEnd)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Timesheets::Status)
                            .string()
                            .not_null()
                            .default("draft"),
                    )
                    .col(
                        ColumnDef::new(Timesheets::SubmittedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Timesheets::ApprovedBy)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Timesheets::ApprovedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Timesheets::Notes)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Timesheets::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Timesheets::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Timesheets::Table, Timesheets::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Add unique constraint on (user_id, period_start, period_end)
        manager
            .create_index(
                Index::create()
                    .name("idx_timesheets_user_period")
                    .table(Timesheets::Table)
                    .col(Timesheets::UserId)
                    .col(Timesheets::PeriodStart)
                    .col(Timesheets::PeriodEnd)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Timesheets::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Timesheets {
    Table,
    Id,
    UserId,
    PeriodStart,
    PeriodEnd,
    Status,
    SubmittedAt,
    ApprovedBy,
    ApprovedAt,
    Notes,
    CreatedAt,
    UpdatedAt,
}
