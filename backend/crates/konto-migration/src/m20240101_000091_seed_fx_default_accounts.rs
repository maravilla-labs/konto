use sea_orm_migration::prelude::*;

use crate::m20240101_000028_create_default_accounts::DefaultAccounts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 6999 Währungsgewinne was seeded as account_type "expense" — a gain account
        // belongs on the revenue side of the Erfolgsrechnung.
        db.execute_unprepared("UPDATE accounts SET account_type = 'revenue' WHERE number = 6999")
            .await?;

        // SQLite keeps whatever an aborted migration already wrote, so clear any
        // rows a previous partial run left behind before re-seeding.
        db.execute_unprepared(
            "DELETE FROM default_accounts WHERE setting_key IN ('fx_gain_account', 'fx_loss_account')",
        )
        .await?;

        let defaults = vec![
            ("fx_gain_account", "6999", "Währungsgewinne (Realized FX Gain)"),
            ("fx_loss_account", "6949", "Währungsverluste (Realized FX Loss)"),
        ];

        for (key, account_number, description) in defaults {
            let id = uuid::Uuid::new_v4().to_string();
            let account_id = find_account_by_number(manager, account_number).await;

            let account_val: SimpleExpr = match &account_id {
                Some(aid) => aid.clone().into(),
                None => Keyword::Null.into(),
            };

            let insert = Query::insert()
                .into_table(DefaultAccounts::Table)
                .columns([
                    DefaultAccounts::Id,
                    DefaultAccounts::SettingKey,
                    DefaultAccounts::AccountId,
                    DefaultAccounts::Description,
                ])
                .values_panic([id.into(), key.into(), account_val, description.into()])
                .to_owned();
            manager.exec_stmt(insert).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "DELETE FROM default_accounts WHERE setting_key IN ('fx_gain_account', 'fx_loss_account')",
        )
        .await?;
        db.execute_unprepared("UPDATE accounts SET account_type = 'expense' WHERE number = 6999")
            .await?;
        Ok(())
    }
}

async fn find_account_by_number(manager: &SchemaManager<'_>, number: &str) -> Option<String> {
    let db = manager.get_connection();
    let result = db
        .query_one(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            format!("SELECT id FROM accounts WHERE number = {number} LIMIT 1"),
        ))
        .await
        .ok()?;
    let row = result?;
    row.try_get::<String>("", "id").ok()
}
