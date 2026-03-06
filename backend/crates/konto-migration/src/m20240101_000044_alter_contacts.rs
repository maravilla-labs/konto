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
                    .add_column(ColumnDef::new(Alias::new("birthday")).date().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("employee_count"))
                            .integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("trade_register_number"))
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("salutation_form"))
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .drop_column(Alias::new("salutation_form"))
                    .to_owned(),
            )
            .await;

        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .drop_column(Alias::new("trade_register_number"))
                    .to_owned(),
            )
            .await;

        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .drop_column(Alias::new("employee_count"))
                    .to_owned(),
            )
            .await;

        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .drop_column(Alias::new("birthday"))
                    .to_owned(),
            )
            .await;

        Ok(())
    }
}
