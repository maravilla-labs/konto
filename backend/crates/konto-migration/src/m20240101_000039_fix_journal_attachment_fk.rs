use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// SQLite does not support ALTER TABLE ... DROP FOREIGN KEY.
/// The only way to fix a broken FK is to recreate the table.
/// This migration copies data out, drops the old table, creates
/// it with the correct FK (journal_entries), and copies data back.
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 1. Copy existing rows into a temp table
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS _journal_attachment_backup AS SELECT * FROM journal_attachment",
        ).await?;

        // 2. Drop the old table (with broken FK)
        db.execute_unprepared("DROP TABLE IF EXISTS journal_attachment").await?;

        // 3. Recreate with correct FK → journal_entries
        db.execute_unprepared(
            "CREATE TABLE journal_attachment (
                id TEXT NOT NULL PRIMARY KEY,
                journal_entry_id TEXT NOT NULL,
                file_name TEXT NOT NULL,
                storage_key TEXT NOT NULL,
                file_size BIGINT NOT NULL,
                mime_type TEXT NOT NULL,
                uploaded_by TEXT,
                created_at DATETIME NOT NULL,
                FOREIGN KEY (journal_entry_id) REFERENCES journal_entries(id) ON DELETE CASCADE
            )",
        ).await?;

        // 4. Copy data back
        db.execute_unprepared(
            "INSERT INTO journal_attachment SELECT * FROM _journal_attachment_backup",
        ).await?;

        // 5. Drop temp table
        db.execute_unprepared("DROP TABLE _journal_attachment_backup").await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // No-op: the table is already correct after up()
        Ok(())
    }
}
