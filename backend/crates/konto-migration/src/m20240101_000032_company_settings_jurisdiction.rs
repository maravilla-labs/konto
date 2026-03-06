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
                        ColumnDef::new(Alias::new("jurisdiction"))
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
                    .table(CompanySettings::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("legal_entity_type"))
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Set default legal entity type
        let db = manager.get_connection();
        db.execute_unprepared(
            "UPDATE company_settings SET legal_entity_type = 'GmbH' WHERE id IS NOT NULL",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for col in ["jurisdiction", "legal_entity_type"] {
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
