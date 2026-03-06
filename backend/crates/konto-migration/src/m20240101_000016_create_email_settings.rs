use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EmailSettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmailSettings::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(EmailSettings::SmtpHost).string().not_null())
                    .col(
                        ColumnDef::new(EmailSettings::SmtpPort)
                            .integer()
                            .not_null()
                            .default(587),
                    )
                    .col(
                        ColumnDef::new(EmailSettings::SmtpUsername)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailSettings::SmtpPassword)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailSettings::SmtpEncryption)
                            .string()
                            .not_null()
                            .default("starttls"),
                    )
                    .col(
                        ColumnDef::new(EmailSettings::FromEmail)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailSettings::FromName)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(EmailSettings::ReplyToEmail).string())
                    .col(ColumnDef::new(EmailSettings::BccEmail).string())
                    .col(
                        ColumnDef::new(EmailSettings::IsActive)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(EmailSettings::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailSettings::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EmailSettings::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum EmailSettings {
    Table,
    Id,
    SmtpHost,
    SmtpPort,
    SmtpUsername,
    SmtpPassword,
    SmtpEncryption,
    FromEmail,
    FromName,
    ReplyToEmail,
    BccEmail,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
