use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PayrollRunLines::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PayrollRunLines::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PayrollRunLines::PayrollRunId).string().not_null())
                    .col(ColumnDef::new(PayrollRunLines::EmployeeId).string().not_null())
                    .col(ColumnDef::new(PayrollRunLines::GrossSalary).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::AhvEmployee).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::AhvEmployer).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::AlvEmployee).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::AlvEmployer).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::BvgEmployee).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::BvgEmployer).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::NbuEmployee).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::BuEmployer).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::KtgEmployee).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::KtgEmployer).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::FakEmployer).decimal_len(15, 2).not_null())
                    .col(
                        ColumnDef::new(PayrollRunLines::Quellensteuer)
                            .decimal_len(15, 2)
                            .not_null()
                            .default("0.00"),
                    )
                    .col(
                        ColumnDef::new(PayrollRunLines::ChildAllowance)
                            .decimal_len(15, 2)
                            .not_null()
                            .default("0.00"),
                    )
                    .col(ColumnDef::new(PayrollRunLines::NetSalary).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::PayoutAmount).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::TotalEmployerCost).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(PayrollRunLines::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(PayrollRunLines::Table, PayrollRunLines::PayrollRunId)
                            .to(PayrollRuns::Table, PayrollRuns::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PayrollRunLines::Table, PayrollRunLines::EmployeeId)
                            .to(Employees::Table, Employees::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PayrollRunLines::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum PayrollRunLines {
    Table,
    Id,
    PayrollRunId,
    EmployeeId,
    GrossSalary,
    AhvEmployee,
    AhvEmployer,
    AlvEmployee,
    AlvEmployer,
    BvgEmployee,
    BvgEmployer,
    NbuEmployee,
    BuEmployer,
    KtgEmployee,
    KtgEmployer,
    FakEmployer,
    Quellensteuer,
    ChildAllowance,
    NetSalary,
    PayoutAmount,
    TotalEmployerCost,
    CreatedAt,
}

#[derive(DeriveIden)]
enum PayrollRuns {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Employees {
    Table,
    Id,
}
