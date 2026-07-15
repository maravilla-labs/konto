use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::Currencies;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Currencies::Table)
                    .add_column(ColumnDef::new(Alias::new("default_bank_account_id")).string().null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Currencies::Table)
                    .drop_column(Alias::new("default_bank_account_id"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
