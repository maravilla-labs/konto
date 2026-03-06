use sea_orm_migration::prelude::*;

use crate::m20240101_000004_create_projects::TimeEntries;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TimeEntries::Table)
                    .add_column(ColumnDef::new(Alias::new("task_id")).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(TimeEntries::Table)
                    .drop_column(Alias::new("task_id"))
                    .to_owned(),
            )
            .await;
        Ok(())
    }
}
