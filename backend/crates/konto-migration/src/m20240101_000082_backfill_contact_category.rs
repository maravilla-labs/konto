use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        // Backfill category from contact_type for imported contacts
        // that have contact_type set but category is NULL
        db.execute_unprepared(
            "UPDATE contacts SET category = 'company' \
             WHERE contact_type = 'company' AND category IS NULL"
        ).await?;
        db.execute_unprepared(
            "UPDATE contacts SET category = 'person' \
             WHERE contact_type != 'company' AND category IS NULL"
        ).await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // No rollback — category was NULL, setting it is non-destructive
        Ok(())
    }
}
