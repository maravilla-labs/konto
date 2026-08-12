use sea_orm_migration::prelude::*;

use crate::m20240101_000004_create_projects::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // SQLite commits DDL as it goes and sea-orm only records a migration
        // once `up` returns Ok, so a migration that fails partway leaves its
        // completed statements in place and is retried from the top on the next
        // boot. Every step below is therefore written to be re-runnable.
        if !manager.has_column("projects", "currency_id").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Projects::Table)
                        .add_column(ColumnDef::new(Alias::new("currency_id")).string().null())
                        .to_owned(),
                )
                .await?;
        }

        // The source column is already gone: a previous run got past the drop
        // below and there is nothing left to backfill.
        if !manager.has_column("projects", "currency").await? {
            return Ok(());
        }

        // Backfill from the free-text `currency` column. Being free text, it
        // holds an ISO code ("CHF", "chf", " USD ") for values picked from the
        // old UI, but hand-entered rows also carry the currency symbol ("€").
        // Match on the code, on the currency's own (user-editable) symbol, and
        // on a fixed table of common symbols — the seeded symbols are the ISO
        // codes themselves, so the symbol column alone does not catch "€".
        // An exact code match always wins over a symbol match.
        // Applied in order of confidence, each pass only touching rows the
        // previous ones left unset, so an exact code match always wins.
        for source in [
            "(SELECT c.id FROM currencies c WHERE UPPER(c.code) = UPPER(TRIM(projects.currency)))",
            "(SELECT c.id FROM currencies c WHERE c.symbol = TRIM(projects.currency))",
            "(SELECT c.id FROM currencies c WHERE UPPER(c.code) = CASE TRIM(projects.currency)
                WHEN '€' THEN 'EUR'
                WHEN '$' THEN 'USD'
                WHEN '£' THEN 'GBP'
                WHEN '¥' THEN 'JPY'
                WHEN 'Fr.' THEN 'CHF'
                WHEN 'SFr.' THEN 'CHF'
              END)",
        ] {
            db.execute_unprepared(&format!(
                "UPDATE projects SET currency_id = {source} WHERE currency_id IS NULL"
            ))
            .await?;
        }

        // Anything still unmatched is a currency string we cannot map to a known
        // currency. Leave it NULL and say so, rather than guessing a rate-bearing
        // value: the column is nullable, the UI renders an empty currency picker,
        // and the user fixes those projects in place. Aborting here instead would
        // fail the whole migration run on every subsequent launch, and the app
        // needs to boot for the user to be able to correct the data at all.
        let unmapped = db
            .query_all(sea_orm::Statement::from_string(
                manager.get_database_backend(),
                "SELECT id, currency FROM projects WHERE currency_id IS NULL".to_owned(),
            ))
            .await?;
        if !unmapped.is_empty() {
            let ids: Vec<String> = unmapped
                .iter()
                .filter_map(|row| row.try_get::<String>("", "id").ok())
                .collect();
            tracing::warn!(
                "migration 089: {} project(s) have a currency that matches no known \
                 currency code or symbol; their currency was left unset and must be \
                 chosen again in the project editor: {}",
                unmapped.len(),
                ids.join(", ")
            );
        }

        // currency_id stays nullable at the DB level (SQLite can't add a NOT NULL
        // constraint post-hoc without a full table rebuild) and has no ALTER-time FK
        // constraint either, matching invoices.currency_id/bank_accounts.currency_id —
        // both enforced only at the entity/service layer. "Required" is enforced
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
        if !manager.has_column("projects", "currency").await? {
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
        }

        if manager.has_column("projects", "currency_id").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Projects::Table)
                        .drop_column(Alias::new("currency_id"))
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }
}
