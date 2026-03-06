use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FixedAssets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FixedAssets::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FixedAssets::Name).string().not_null())
                    .col(ColumnDef::new(FixedAssets::Description).text().null())
                    .col(ColumnDef::new(FixedAssets::AccountId).string().not_null())
                    .col(ColumnDef::new(FixedAssets::DepreciationAccountId).string().not_null())
                    .col(ColumnDef::new(FixedAssets::AcquisitionDate).date().not_null())
                    .col(
                        ColumnDef::new(FixedAssets::AcquisitionCost)
                            .decimal_len(15, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FixedAssets::ResidualValue)
                            .decimal_len(15, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FixedAssets::UsefulLifeYears)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FixedAssets::DepreciationMethod)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FixedAssets::DecliningRate)
                            .decimal_len(5, 4)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(FixedAssets::Status)
                            .string()
                            .not_null()
                            .default("active"),
                    )
                    .col(ColumnDef::new(FixedAssets::DisposedDate).date().null())
                    .col(ColumnDef::new(FixedAssets::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(FixedAssets::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(FixedAssets::Table, FixedAssets::AccountId)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(FixedAssets::Table, FixedAssets::DepreciationAccountId)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FixedAssets::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum FixedAssets {
    Table,
    Id,
    Name,
    Description,
    AccountId,
    DepreciationAccountId,
    AcquisitionDate,
    AcquisitionCost,
    ResidualValue,
    UsefulLifeYears,
    DepreciationMethod,
    DecliningRate,
    Status,
    DisposedDate,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
}
