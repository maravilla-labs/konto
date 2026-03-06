use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Employees {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .add_column(ColumnDef::new(Alias::new("number")).string().null())
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX idx_employees_number ON employees(number) WHERE number IS NOT NULL"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let _ = db.execute_unprepared("DROP INDEX IF EXISTS idx_employees_number").await;

        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Employees::Table)
                    .drop_column(Alias::new("number"))
                    .to_owned(),
            )
            .await;
        Ok(())
    }
}
