use sea_orm_migration::prelude::*;

use crate::m20240101_000003_create_contacts::Contacts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ContactTags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ContactTags::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(ContactTags::Name).string().not_null())
                    .col(ColumnDef::new(ContactTags::Color).string().not_null())
                    .col(ColumnDef::new(ContactTags::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(ContactTags::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ContactTagAssignments::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ContactTagAssignments::ContactId).string().not_null())
                    .col(ColumnDef::new(ContactTagAssignments::TagId).string().not_null())
                    .primary_key(
                        Index::create()
                            .col(ContactTagAssignments::ContactId)
                            .col(ContactTagAssignments::TagId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ContactTagAssignments::Table, ContactTagAssignments::ContactId)
                            .to(Contacts::Table, Contacts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ContactTagAssignments::Table, ContactTagAssignments::TagId)
                            .to(ContactTags::Table, ContactTags::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Seed default tags
        seed_tags(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ContactTagAssignments::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ContactTags::Table).to_owned())
            .await?;
        Ok(())
    }
}

async fn seed_tags(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let now = chrono::Utc::now().naive_utc().to_string();
    let tags = vec![
        ("customer-tag-001", "Customer", "#3B82F6"),
        ("supplier-tag-002", "Supplier", "#F59E0B"),
        ("partner-tag-003", "Partner", "#10B981"),
    ];

    for (id, name, color) in tags {
        let insert = Query::insert()
            .into_table(ContactTags::Table)
            .columns([
                ContactTags::Id,
                ContactTags::Name,
                ContactTags::Color,
                ContactTags::CreatedAt,
                ContactTags::UpdatedAt,
            ])
            .values_panic([id.into(), name.into(), color.into(), now.clone().into(), now.clone().into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
    }
    Ok(())
}

#[derive(DeriveIden)]
pub enum ContactTags {
    Table,
    Id,
    Name,
    Color,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum ContactTagAssignments {
    Table,
    ContactId,
    TagId,
}
