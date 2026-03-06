use sea_orm_migration::prelude::*;

use crate::m20240101_000063_create_employees::Employees;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite requires one alter_table per column
        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .add_column(ColumnDef::new(Alias::new("email")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .add_column(ColumnDef::new(Alias::new("phone")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("nationality"))
                            .string()
                            .not_null()
                            .default("CH"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .add_column(ColumnDef::new(Alias::new("position")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .add_column(ColumnDef::new(Alias::new("department")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("annual_salary_13th"))
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("bvg_insured"))
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("uvg_insured"))
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("ktg_insured"))
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .add_column(ColumnDef::new(Alias::new("notes")).text().null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for col in [
            "notes", "ktg_insured", "uvg_insured", "bvg_insured",
            "annual_salary_13th", "department", "position", "nationality",
            "phone", "email",
        ] {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Employees::Table)
                        .drop_column(Alias::new(col))
                        .to_owned(),
                )
                .await;
        }
        Ok(())
    }
}
