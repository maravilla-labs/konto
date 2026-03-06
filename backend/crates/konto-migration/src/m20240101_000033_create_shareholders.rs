use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Shareholders::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Shareholders::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Shareholders::Name).string().not_null())
                    .col(ColumnDef::new(Shareholders::City).string().not_null())
                    .col(ColumnDef::new(Shareholders::Role).string().not_null())
                    .col(ColumnDef::new(Shareholders::SigningRights).string().null())
                    .col(
                        ColumnDef::new(Shareholders::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Shareholders::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Shareholders::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Shareholders::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Shareholders {
    Table,
    Id,
    Name,
    City,
    Role,
    SigningRights,
    SortOrder,
    CreatedAt,
    UpdatedAt,
}
