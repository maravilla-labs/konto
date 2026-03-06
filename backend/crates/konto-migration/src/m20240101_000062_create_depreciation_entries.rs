use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DepreciationEntries::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DepreciationEntries::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DepreciationEntries::FixedAssetId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DepreciationEntries::FiscalYearId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DepreciationEntries::JournalEntryId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DepreciationEntries::Amount)
                            .decimal_len(15, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DepreciationEntries::Accumulated)
                            .decimal_len(15, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DepreciationEntries::BookValue)
                            .decimal_len(15, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DepreciationEntries::PeriodDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DepreciationEntries::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(DepreciationEntries::Table, DepreciationEntries::FixedAssetId)
                            .to(FixedAssets::Table, FixedAssets::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(DepreciationEntries::Table, DepreciationEntries::FiscalYearId)
                            .to(FiscalYears::Table, FiscalYears::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(DepreciationEntries::Table, DepreciationEntries::JournalEntryId)
                            .to(JournalEntries::Table, JournalEntries::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DepreciationEntries::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum DepreciationEntries {
    Table,
    Id,
    FixedAssetId,
    FiscalYearId,
    JournalEntryId,
    Amount,
    Accumulated,
    BookValue,
    PeriodDate,
    CreatedAt,
}

#[derive(DeriveIden)]
enum FixedAssets {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum FiscalYears {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum JournalEntries {
    Table,
    Id,
}
