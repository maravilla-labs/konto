use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::{Currencies, Users};
use crate::m20240101_000003_create_contacts::Contacts;
use crate::m20240101_000004_create_projects::Projects;
use crate::m20240101_000012_create_templates::DocumentTemplates;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Documents table
        manager
            .create_table(
                Table::create()
                    .table(Documents::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Documents::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Documents::DocType).string().not_null())
                    .col(ColumnDef::new(Documents::DocNumber).string().null().unique_key())
                    .col(ColumnDef::new(Documents::Title).string().not_null())
                    .col(
                        ColumnDef::new(Documents::Status)
                            .string()
                            .not_null()
                            .default("draft"),
                    )
                    .col(ColumnDef::new(Documents::ContactId).string().not_null())
                    .col(ColumnDef::new(Documents::ProjectId).string().null())
                    .col(ColumnDef::new(Documents::TemplateId).string().null())
                    .col(ColumnDef::new(Documents::ContentJson).text().not_null())
                    .col(ColumnDef::new(Documents::CurrencyId).string().null())
                    .col(ColumnDef::new(Documents::Subtotal).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(Documents::VatRate).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(Documents::VatAmount).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(Documents::Total).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(Documents::ValidUntil).date().null())
                    .col(ColumnDef::new(Documents::IssuedAt).date().null())
                    .col(ColumnDef::new(Documents::SignedAt).date().null())
                    .col(ColumnDef::new(Documents::ConvertedFrom).string().null())
                    .col(ColumnDef::new(Documents::CreatedBy).string().null())
                    .col(ColumnDef::new(Documents::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Documents::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Documents::Table, Documents::ContactId)
                            .to(Contacts::Table, Contacts::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Documents::Table, Documents::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Documents::Table, Documents::TemplateId)
                            .to(DocumentTemplates::Table, DocumentTemplates::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Documents::Table, Documents::CurrencyId)
                            .to(Currencies::Table, Currencies::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Documents::Table, Documents::ConvertedFrom)
                            .to(Documents::Table, Documents::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Documents::Table, Documents::CreatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Document Line Items
        manager
            .create_table(
                Table::create()
                    .table(DocumentLineItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DocumentLineItems::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DocumentLineItems::DocumentId).string().not_null())
                    .col(ColumnDef::new(DocumentLineItems::Position).integer().not_null())
                    .col(ColumnDef::new(DocumentLineItems::Description).text().not_null())
                    .col(ColumnDef::new(DocumentLineItems::Quantity).decimal().not_null())
                    .col(ColumnDef::new(DocumentLineItems::Unit).string().null())
                    .col(ColumnDef::new(DocumentLineItems::UnitPrice).decimal().not_null())
                    .col(
                        ColumnDef::new(DocumentLineItems::DiscountPct)
                            .decimal()
                            .not_null()
                            .default(0.0),
                    )
                    .col(ColumnDef::new(DocumentLineItems::Total).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(DocumentLineItems::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(DocumentLineItems::Table, DocumentLineItems::DocumentId)
                            .to(Documents::Table, Documents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DocumentLineItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Documents::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Documents {
    Table,
    Id,
    DocType,
    DocNumber,
    Title,
    Status,
    ContactId,
    ProjectId,
    TemplateId,
    ContentJson,
    CurrencyId,
    Subtotal,
    VatRate,
    VatAmount,
    Total,
    ValidUntil,
    IssuedAt,
    SignedAt,
    ConvertedFrom,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum DocumentLineItems {
    Table,
    Id,
    DocumentId,
    Position,
    Description,
    Quantity,
    Unit,
    UnitPrice,
    DiscountPct,
    Total,
    CreatedAt,
}
