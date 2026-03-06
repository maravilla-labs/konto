use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add report-related columns to expenses
        let db = manager.get_connection();

        db.execute_unprepared(
            "ALTER TABLE expenses ADD COLUMN expense_type TEXT NOT NULL DEFAULT 'single'"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE expenses ADD COLUMN purpose TEXT"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE expenses ADD COLUMN employee_id TEXT REFERENCES users(id)"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE expenses ADD COLUMN period_from DATE"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE expenses ADD COLUMN period_to DATE"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE expenses ADD COLUMN advances DECIMAL(15,2) NOT NULL DEFAULT 0.0"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE expenses ADD COLUMN total_reimbursement DECIMAL(15,2) NOT NULL DEFAULT 0.0"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE expenses ADD COLUMN approved_by TEXT REFERENCES users(id)"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE expenses ADD COLUMN approved_at DATETIME"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE expenses ADD COLUMN rejected_reason TEXT"
        ).await?;

        // Create expense_lines table for report-type expenses
        manager
            .create_table(
                Table::create()
                    .table(ExpenseLine::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ExpenseLine::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(ExpenseLine::ExpenseId).string().not_null())
                    .col(ColumnDef::new(ExpenseLine::Position).integer().not_null())
                    .col(ColumnDef::new(ExpenseLine::ExpenseDate).date().not_null())
                    .col(ColumnDef::new(ExpenseLine::Description).text().not_null())
                    .col(ColumnDef::new(ExpenseLine::AirTransport).decimal_len(15, 2).not_null().default(0.0))
                    .col(ColumnDef::new(ExpenseLine::Lodging).decimal_len(15, 2).not_null().default(0.0))
                    .col(ColumnDef::new(ExpenseLine::FuelMileage).decimal_len(15, 2).not_null().default(0.0))
                    .col(ColumnDef::new(ExpenseLine::Phone).decimal_len(15, 2).not_null().default(0.0))
                    .col(ColumnDef::new(ExpenseLine::MealsTips).decimal_len(15, 2).not_null().default(0.0))
                    .col(ColumnDef::new(ExpenseLine::Entertainment).decimal_len(15, 2).not_null().default(0.0))
                    .col(ColumnDef::new(ExpenseLine::Other).decimal_len(15, 2).not_null().default(0.0))
                    .col(ColumnDef::new(ExpenseLine::LineTotal).decimal_len(15, 2).not_null().default(0.0))
                    .col(ColumnDef::new(ExpenseLine::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(ExpenseLine::UpdatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ExpenseLine::Table, ExpenseLine::ExpenseId)
                            .to(Expense::Table, Expense::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create expense_receipts table
        manager
            .create_table(
                Table::create()
                    .table(ExpenseReceipt::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ExpenseReceipt::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(ExpenseReceipt::ExpenseId).string().not_null())
                    .col(ColumnDef::new(ExpenseReceipt::LineId).string().null())
                    .col(ColumnDef::new(ExpenseReceipt::FileName).string().not_null())
                    .col(ColumnDef::new(ExpenseReceipt::StorageKey).string().not_null())
                    .col(ColumnDef::new(ExpenseReceipt::FileSize).big_integer().not_null())
                    .col(ColumnDef::new(ExpenseReceipt::MimeType).string().not_null())
                    .col(ColumnDef::new(ExpenseReceipt::UploadedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ExpenseReceipt::Table, ExpenseReceipt::ExpenseId)
                            .to(Expense::Table, Expense::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Add default_account_id to expense_categories
        db.execute_unprepared(
            "ALTER TABLE expense_categories ADD COLUMN default_account_id TEXT REFERENCES accounts(id)"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ExpenseReceipt::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(ExpenseLine::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Expense {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum ExpenseLine {
    Table,
    Id,
    ExpenseId,
    Position,
    ExpenseDate,
    Description,
    AirTransport,
    Lodging,
    FuelMileage,
    Phone,
    MealsTips,
    Entertainment,
    Other,
    LineTotal,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ExpenseReceipt {
    Table,
    Id,
    ExpenseId,
    LineId,
    FileName,
    StorageKey,
    FileSize,
    MimeType,
    UploadedAt,
}
