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
                    .table(ContactRelationships::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ContactRelationships::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ContactRelationships::PersonContactId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ContactRelationships::OrgContactId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ContactRelationships::Role).string().null())
                    .col(ColumnDef::new(ContactRelationships::Position).string().null())
                    .col(ColumnDef::new(ContactRelationships::Department).string().null())
                    .col(
                        ColumnDef::new(ContactRelationships::IsPrimary)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(ContactRelationships::Notes).text().null())
                    .col(
                        ColumnDef::new(ContactRelationships::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ContactRelationships::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ContactRelationships::Table,
                                ContactRelationships::PersonContactId,
                            )
                            .to(Contacts::Table, Contacts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ContactRelationships::Table,
                                ContactRelationships::OrgContactId,
                            )
                            .to(Contacts::Table, Contacts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique index on (person_contact_id, org_contact_id)
        manager
            .create_index(
                Index::create()
                    .name("idx_contact_relationships_person_org")
                    .table(ContactRelationships::Table)
                    .col(ContactRelationships::PersonContactId)
                    .col(ContactRelationships::OrgContactId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ContactRelationships::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ContactRelationships {
    Table,
    Id,
    PersonContactId,
    OrgContactId,
    Role,
    Position,
    Department,
    IsPrimary,
    Notes,
    CreatedAt,
    UpdatedAt,
}
