use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TimeEntries::Table)
                    .add_column(
                        ColumnDef::new(TimeEntries::Billed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TimeEntries::Table)
                    .drop_column(TimeEntries::Billed)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum TimeEntries {
    Table,
    Billed,
}
