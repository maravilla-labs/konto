use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Remove seeded shareholders (from migration 000036)
        // Use name match since IDs were random UUIDs
        db.execute_unprepared(
            "DELETE FROM shareholders WHERE name IN ('Doe, John', 'Doe, Jane')",
        )
        .await?;

        // Remove seed admin user only if no other tables reference it.
        // Use a subquery guard: skip if time_entries, timesheets, or project_members reference it.
        db.execute_unprepared(
            "DELETE FROM users WHERE id = 'user-admin' \
             AND NOT EXISTS (SELECT 1 FROM time_entries WHERE user_id = 'user-admin') \
             AND NOT EXISTS (SELECT 1 FROM timesheets WHERE user_id = 'user-admin') \
             AND NOT EXISTS (SELECT 1 FROM project_members WHERE user_id = 'user-admin')",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
