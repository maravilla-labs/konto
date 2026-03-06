use sea_orm_migration::prelude::*;

use crate::m20240101_000004_create_projects::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("soft_budget_hours"))
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("hard_budget_hours"))
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("soft_budget_amount"))
                            .decimal_len(15, 2)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("hard_budget_amount"))
                            .decimal_len(15, 2)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("contact_person_id"))
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
                    .table(Projects::Table)
                    .drop_column(Alias::new("contact_person_id"))
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .drop_column(Alias::new("hard_budget_amount"))
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .drop_column(Alias::new("soft_budget_amount"))
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .drop_column(Alias::new("hard_budget_hours"))
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .drop_column(Alias::new("soft_budget_hours"))
                    .to_owned(),
            )
            .await;
        Ok(())
    }
}
