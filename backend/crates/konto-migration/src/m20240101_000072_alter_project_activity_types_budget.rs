use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum ProjectActivityTypes {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ProjectActivityTypes::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("budget_hours"))
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(ProjectActivityTypes::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("chargeable"))
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for col in ["chargeable", "budget_hours"] {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(ProjectActivityTypes::Table)
                        .drop_column(Alias::new(col))
                        .to_owned(),
                )
                .await;
        }
        Ok(())
    }
}
