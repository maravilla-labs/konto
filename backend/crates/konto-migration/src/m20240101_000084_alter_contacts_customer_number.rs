use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Contacts {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .add_column(ColumnDef::new(Alias::new("customer_number")).string().null())
                    .to_owned(),
            )
            .await?;

        // Partial unique index on customer_number WHERE NOT NULL
        let db = manager.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX idx_contacts_customer_number ON contacts(customer_number) WHERE customer_number IS NOT NULL"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let _ = db.execute_unprepared("DROP INDEX IF EXISTS idx_contacts_customer_number").await;

        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Contacts::Table)
                    .drop_column(Alias::new("customer_number"))
                    .to_owned(),
            )
            .await;
        Ok(())
    }
}
