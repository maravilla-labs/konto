use sea_orm_migration::prelude::*;

use crate::m20240101_000011_create_settings::CompanySettings;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(CompanySettings::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("audit_optout"))
                            .boolean()
                            .not_null()
                            .default(true),
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
                    .table(CompanySettings::Table)
                    .drop_column(Alias::new("audit_optout"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
