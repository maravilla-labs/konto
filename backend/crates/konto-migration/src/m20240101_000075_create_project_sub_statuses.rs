use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum ProjectSubStatuses {
    Table,
    Id,
    Name,
    SortOrder,
    Color,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectSubStatuses::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectSubStatuses::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProjectSubStatuses::Name).string().not_null())
                    .col(
                        ColumnDef::new(ProjectSubStatuses::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ProjectSubStatuses::Color)
                            .string()
                            .not_null()
                            .default("#6b7280"),
                    )
                    .col(
                        ColumnDef::new(ProjectSubStatuses::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(ProjectSubStatuses::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectSubStatuses::UpdatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectSubStatuses::Table).to_owned())
            .await
    }
}
