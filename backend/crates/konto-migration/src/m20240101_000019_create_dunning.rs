use sea_orm_migration::prelude::*;

use crate::m20240101_000010_create_invoices::Invoices;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Dunning Levels
        manager
            .create_table(
                Table::create()
                    .table(DunningLevels::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DunningLevels::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(DunningLevels::Level).integer().not_null())
                    .col(ColumnDef::new(DunningLevels::DaysAfterDue).integer().not_null())
                    .col(
                        ColumnDef::new(DunningLevels::FeeAmount)
                            .decimal_len(15, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(DunningLevels::SubjectTemplate).text().not_null())
                    .col(ColumnDef::new(DunningLevels::BodyTemplate).text().not_null())
                    .col(
                        ColumnDef::new(DunningLevels::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(DunningLevels::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(DunningLevels::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Dunning Entries
        manager
            .create_table(
                Table::create()
                    .table(DunningEntries::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DunningEntries::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(DunningEntries::InvoiceId).string().not_null())
                    .col(ColumnDef::new(DunningEntries::DunningLevelId).string().not_null())
                    .col(ColumnDef::new(DunningEntries::SentAt).timestamp().not_null())
                    .col(
                        ColumnDef::new(DunningEntries::FeeAmount)
                            .decimal_len(15, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(DunningEntries::EmailSent)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(DunningEntries::JournalEntryId).string().null())
                    .col(ColumnDef::new(DunningEntries::Notes).text().null())
                    .col(ColumnDef::new(DunningEntries::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(DunningEntries::Table, DunningEntries::InvoiceId)
                            .to(Invoices::Table, Invoices::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(DunningEntries::Table, DunningEntries::DunningLevelId)
                            .to(DunningLevels::Table, DunningLevels::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Seed default dunning levels
        let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();
        let db = manager.get_connection();

        // Level 1: Friendly reminder (14 days)
        db.execute_unprepared(&format!(
            "INSERT INTO dunning_levels (id, level, days_after_due, fee_amount, subject_template, body_template, is_active, created_at, updated_at) VALUES ('{}', 1, 14, 0, 'Payment Reminder: Invoice {{{{invoice_number}}}}', 'Dear {{{{contact_name}}}},\n\nThis is a friendly reminder that invoice {{{{invoice_number}}}} for {{{{amount}}}} was due on {{{{due_date}}}}.\n\nPlease arrange payment at your earliest convenience.\n\nBest regards,\n{{{{company_name}}}}', 1, '{}', '{}')",
            uuid::Uuid::new_v4(), now, now
        )).await?;

        // Level 2: Second reminder (30 days, 10 CHF fee)
        db.execute_unprepared(&format!(
            "INSERT INTO dunning_levels (id, level, days_after_due, fee_amount, subject_template, body_template, is_active, created_at, updated_at) VALUES ('{}', 2, 30, 10, 'Second Payment Reminder: Invoice {{{{invoice_number}}}}', 'Dear {{{{contact_name}}}},\n\nWe have not yet received payment for invoice {{{{invoice_number}}}} for {{{{amount}}}}, which was due on {{{{due_date}}}}.\n\nA reminder fee of CHF 10.00 has been applied.\n\nPlease arrange payment within 10 days.\n\nBest regards,\n{{{{company_name}}}}', 1, '{}', '{}')",
            uuid::Uuid::new_v4(), now, now
        )).await?;

        // Level 3: Final warning (45 days, 20 CHF fee)
        db.execute_unprepared(&format!(
            "INSERT INTO dunning_levels (id, level, days_after_due, fee_amount, subject_template, body_template, is_active, created_at, updated_at) VALUES ('{}', 3, 45, 20, 'Final Payment Warning: Invoice {{{{invoice_number}}}}', 'Dear {{{{contact_name}}}},\n\nDespite our previous reminders, invoice {{{{invoice_number}}}} for {{{{amount}}}} remains unpaid since {{{{due_date}}}}.\n\nA reminder fee of CHF 20.00 has been applied.\n\nIf payment is not received within 10 days, we may be forced to take further action.\n\nBest regards,\n{{{{company_name}}}}', 1, '{}', '{}')",
            uuid::Uuid::new_v4(), now, now
        )).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(DunningEntries::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(DunningLevels::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum DunningLevels {
    Table,
    Id,
    Level,
    DaysAfterDue,
    FeeAmount,
    SubjectTemplate,
    BodyTemplate,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum DunningEntries {
    Table,
    Id,
    InvoiceId,
    DunningLevelId,
    SentAt,
    FeeAmount,
    EmailSent,
    JournalEntryId,
    Notes,
    CreatedAt,
}
