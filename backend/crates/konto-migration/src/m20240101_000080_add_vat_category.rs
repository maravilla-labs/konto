use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum VatRates {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add vat_category column
        manager
            .alter_table(
                Table::alter()
                    .table(VatRates::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("vat_category"))
                            .string()
                            .not_null()
                            .default("standard"),
                    )
                    .to_owned(),
            )
            .await?;

        // Seed: Reverse Charge 0%
        let db = manager.get_connection();
        db.execute_unprepared(
            "INSERT INTO vat_rates (id, code, name, rate, vat_type, vat_category, is_active) \
             VALUES ('vat-rc0', 'RC0', 'Reverse Charge 0%', 0.00, 'output', 'zero_reverse_charge', 1)"
        ).await?;

        // Seed: Export Exempt 0%
        db.execute_unprepared(
            "INSERT INTO vat_rates (id, code, name, rate, vat_type, vat_category, is_active) \
             VALUES ('vat-ex0', 'EX0', 'Export Exempt 0%', 0.00, 'output', 'zero_export', 1)"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DELETE FROM vat_rates WHERE id IN ('vat-rc0', 'vat-ex0')").await?;

        manager
            .alter_table(
                Table::alter()
                    .table(VatRates::Table)
                    .drop_column(Alias::new("vat_category"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
