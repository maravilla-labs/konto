use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(JournalAttachment::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(JournalAttachment::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(JournalAttachment::JournalEntryId).string().not_null())
                    .col(ColumnDef::new(JournalAttachment::FileName).string().not_null())
                    .col(ColumnDef::new(JournalAttachment::StorageKey).string().not_null())
                    .col(ColumnDef::new(JournalAttachment::FileSize).big_integer().not_null())
                    .col(ColumnDef::new(JournalAttachment::MimeType).string().not_null())
                    .col(ColumnDef::new(JournalAttachment::UploadedBy).string().null())
                    .col(ColumnDef::new(JournalAttachment::CreatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(JournalAttachment::Table, JournalAttachment::JournalEntryId)
                            .to(JournalEntries::Table, JournalEntries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(JournalAttachment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum JournalAttachment {
    Table,
    Id,
    JournalEntryId,
    FileName,
    StorageKey,
    FileSize,
    MimeType,
    UploadedBy,
    CreatedAt,
}

#[derive(DeriveIden)]
enum JournalEntries {
    Table,
    Id,
}
