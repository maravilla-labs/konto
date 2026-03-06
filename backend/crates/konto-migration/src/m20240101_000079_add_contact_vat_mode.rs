use sea_orm_migration::prelude::*;

use crate::m20240101_000003_create_contacts::Contacts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("vat_mode"))
                            .string()
                            .not_null()
                            .default("auto"),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .drop_column(Alias::new("vat_mode"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
