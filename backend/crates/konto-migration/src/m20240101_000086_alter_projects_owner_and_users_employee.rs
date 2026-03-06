use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Projects {
    Table,
}

#[derive(DeriveIden)]
enum Users {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add owner_id to projects (FK → employees)
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(ColumnDef::new(Alias::new("owner_id")).string().null())
                    .to_owned(),
            )
            .await?;

        // Add employee_id to users (FK → employees)
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Alias::new("employee_id")).string().null())
                    .to_owned(),
            )
            .await?;

        // Unique index on users(employee_id) WHERE NOT NULL
        let db = manager.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX idx_users_employee_id ON users(employee_id) WHERE employee_id IS NOT NULL"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let _ = db.execute_unprepared("DROP INDEX IF EXISTS idx_users_employee_id").await;

        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Alias::new("employee_id"))
                    .to_owned(),
            )
            .await;

        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .drop_column(Alias::new("owner_id"))
                    .to_owned(),
            )
            .await;

        Ok(())
    }
}
