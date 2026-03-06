use sea_orm_migration::prelude::*;

use crate::m20240101_000011_create_settings::CompanySettings;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum CompanySettingsExtra {
    FlatRatePercentage,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(CompanySettings::Table)
                    .add_column(
                        ColumnDef::new(CompanySettingsExtra::FlatRatePercentage)
                            .decimal_len(5, 2)
                            .null()
                            .default(Value::Double(None)),
                    )
                    .to_owned(),
            )
            .await?;

        // Set default 6.2% for existing flat_rate companies
        let db = manager.get_connection();
        db.execute_unprepared(
            "UPDATE company_settings SET flat_rate_percentage = 6.2 WHERE vat_method = 'flat_rate'",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(CompanySettings::Table)
                    .drop_column(CompanySettingsExtra::FlatRatePercentage)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
