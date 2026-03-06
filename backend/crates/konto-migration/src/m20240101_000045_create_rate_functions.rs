use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RateFunctions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RateFunctions::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RateFunctions::Name).string().not_null())
                    .col(ColumnDef::new(RateFunctions::Description).text().null())
                    .col(
                        ColumnDef::new(RateFunctions::HourlyRate)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RateFunctions::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(RateFunctions::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(RateFunctions::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(RateFunctions::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RateFunctions::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum RateFunctions {
    Table,
    Id,
    Name,
    Description,
    HourlyRate,
    IsActive,
    SortOrder,
    CreatedAt,
    UpdatedAt,
}
