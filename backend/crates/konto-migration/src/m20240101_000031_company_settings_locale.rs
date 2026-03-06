use sea_orm_migration::prelude::*;

use crate::m20240101_000011_create_settings::CompanySettings;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add locale/regional columns to company_settings
        manager
            .alter_table(
                Table::alter()
                    .table(CompanySettings::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("default_currency_id"))
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(CompanySettings::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("date_format"))
                            .string()
                            .not_null()
                            .default("dd.MM.yyyy"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(CompanySettings::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("number_format"))
                            .string()
                            .not_null()
                            .default("ch"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(CompanySettings::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("fiscal_year_start_month"))
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(CompanySettings::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("tax_id_label"))
                            .string()
                            .not_null()
                            .default("UID/MWST"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite doesn't support DROP COLUMN directly in all versions,
        // but SeaORM migration handles this
        let cols = [
            "default_currency_id",
            "date_format",
            "number_format",
            "fiscal_year_start_month",
            "tax_id_label",
        ];
        for col in cols {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(CompanySettings::Table)
                        .drop_column(Alias::new(col))
                        .to_owned(),
                )
                .await;
        }
        Ok(())
    }
}
