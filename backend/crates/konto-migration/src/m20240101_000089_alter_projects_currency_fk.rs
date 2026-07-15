use sea_orm_migration::prelude::*;

use crate::m20240101_000004_create_projects::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(ColumnDef::new(Alias::new("currency_id")).string().null())
                    .to_owned(),
            )
            .await?;

        // Backfill from the free-text `currency` code column, case-insensitively.
        db.execute_unprepared(
            "UPDATE projects SET currency_id = (
                SELECT currencies.id FROM currencies
                WHERE UPPER(currencies.code) = UPPER(projects.currency)
             )",
        )
        .await?;

        // Fail loudly rather than silently defaulting: any project whose currency
        // string doesn't match a known currency code needs a data-quality fix,
        // not a guess.
        let unmapped = db
            .query_all(sea_orm::Statement::from_string(
                manager.get_database_backend(),
                "SELECT id, currency FROM projects WHERE currency_id IS NULL".to_owned(),
            ))
            .await?;
        if !unmapped.is_empty() {
            return Err(DbErr::Custom(format!(
                "{} project(s) have a currency code not present in the currencies table; \
                 add the missing currency in Settings before migrating",
                unmapped.len()
            )));
        }

        // currency_id stays nullable at the DB level (SQLite can't add a NOT NULL
        // constraint post-hoc without a full table rebuild) and has no ALTER-time FK
        // constraint either, matching invoices.currency_id/bank_accounts.currency_id —
        // both enforced only at the entity/service layer. The backfill check above
        // already guarantees every existing row is populated; "required" is enforced
        // going forward by project_service on create/update.
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .drop_column(Alias::new("currency"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(ColumnDef::new(Alias::new("currency")).string().not_null().default("CHF"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .drop_column(Alias::new("currency_id"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
