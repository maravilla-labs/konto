use sea_orm_migration::prelude::*;

use crate::m20240101_000004_create_projects::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // invoicing_method: hourly/fixed_price/flat_rate/non_billable
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("invoicing_method"))
                            .string()
                            .not_null()
                            .default("hourly"),
                    )
                    .to_owned(),
            )
            .await?;

        // currency per project
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("currency"))
                            .string()
                            .not_null()
                            .default("CHF"),
                    )
                    .to_owned(),
            )
            .await?;

        // rounding_method: up/down/nearest (nullable)
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("rounding_method"))
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // rounding_factor_minutes: 5/10/15/30/60 (nullable)
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("rounding_factor_minutes"))
                            .integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // flat_rate_total for flat_rate invoicing method
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("flat_rate_total"))
                            .decimal_len(15, 2)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for col in ["flat_rate_total", "rounding_factor_minutes", "rounding_method", "currency", "invoicing_method"] {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Projects::Table)
                        .drop_column(Alias::new(col))
                        .to_owned(),
                )
                .await;
        }
        Ok(())
    }
}
