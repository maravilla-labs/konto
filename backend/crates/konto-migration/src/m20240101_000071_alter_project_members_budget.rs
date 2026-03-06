use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum ProjectMembers {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ProjectMembers::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("budget_hours"))
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(ProjectMembers::Table)
                    .drop_column(Alias::new("budget_hours"))
                    .to_owned(),
            )
            .await;
        Ok(())
    }
}
