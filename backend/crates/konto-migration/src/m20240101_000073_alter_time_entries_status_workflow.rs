use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum TimeEntries {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // billable boolean (default true)
        manager
            .alter_table(
                Table::alter()
                    .table(TimeEntries::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("billable"))
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await?;

        // start_time (HH:MM for from-to mode)
        manager
            .alter_table(
                Table::alter()
                    .table(TimeEntries::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("start_time"))
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // end_time (HH:MM for from-to mode)
        manager
            .alter_table(
                Table::alter()
                    .table(TimeEntries::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("end_time"))
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Data migration: set billable=false WHERE status='non_billable'
        let db = manager.get_connection();
        db.execute_unprepared(
            "UPDATE time_entries SET billable = 0 WHERE status = 'non_billable'"
        ).await?;

        // Data migration: set status='done' WHERE billed=true
        db.execute_unprepared(
            "UPDATE time_entries SET status = 'done' WHERE billed = 1 AND status != 'done'"
        ).await?;

        // Data migration: set status='pending' WHERE status='active'
        db.execute_unprepared(
            "UPDATE time_entries SET status = 'pending' WHERE status = 'active'"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Reverse data migration
        let db = manager.get_connection();
        db.execute_unprepared(
            "UPDATE time_entries SET status = 'active' WHERE status = 'pending'"
        ).await?;

        for col in ["end_time", "start_time", "billable"] {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(TimeEntries::Table)
                        .drop_column(Alias::new(col))
                        .to_owned(),
                )
                .await;
        }
        Ok(())
    }
}
