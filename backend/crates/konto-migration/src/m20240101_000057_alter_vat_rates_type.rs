use sea_orm_migration::prelude::*;

use crate::m20240101_000002_create_accounting::VatRates;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// Iden for the new column.
#[derive(DeriveIden)]
enum VatRatesExtra {
    VatType,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Add vat_type column with default 'output'
        manager
            .alter_table(
                Table::alter()
                    .table(VatRates::Table)
                    .add_column(
                        ColumnDef::new(VatRatesExtra::VatType)
                            .string()
                            .not_null()
                            .default("output"),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Set input type on Vorsteuer codes (V* prefix)
        let input_codes = ["VB77", "VIM", "VM77", "VM81", "VSF"];
        for code in input_codes {
            manager
                .exec_stmt(
                    Query::update()
                        .table(VatRates::Table)
                        .value(VatRatesExtra::VatType, "input")
                        .and_where(Expr::col(VatRates::Code).eq(code))
                        .to_owned(),
                )
                .await?;
        }

        // 3. Seed missing code US77/23
        let db = manager.get_connection();
        db.execute_unprepared(
            "INSERT INTO vat_rates (id, code, name, rate, is_active, vat_type) VALUES ('vat-us7723', 'US77/23', 'Umsatzsteuer 7.7% (2023)', 7.7, 1, 'output')",
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove seeded code
        manager
            .exec_stmt(
                Query::delete()
                    .from_table(VatRates::Table)
                    .and_where(Expr::col(VatRates::Id).eq("vat-us7723"))
                    .to_owned(),
            )
            .await?;

        // Drop column
        manager
            .alter_table(
                Table::alter()
                    .table(VatRates::Table)
                    .drop_column(VatRatesExtra::VatType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
