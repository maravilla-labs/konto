use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Contacts::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Contacts::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Contacts::ContactType).string().not_null())
                    .col(ColumnDef::new(Contacts::Category).string().null())
                    .col(ColumnDef::new(Contacts::Industry).string().null())
                    .col(ColumnDef::new(Contacts::Name1).string().not_null())
                    .col(ColumnDef::new(Contacts::Name2).string().null())
                    .col(ColumnDef::new(Contacts::Salutation).string().null())
                    .col(ColumnDef::new(Contacts::Title).string().null())
                    .col(ColumnDef::new(Contacts::Address).text().null())
                    .col(ColumnDef::new(Contacts::PostalCode).string().null())
                    .col(ColumnDef::new(Contacts::City).string().null())
                    .col(ColumnDef::new(Contacts::Country).string().null())
                    .col(ColumnDef::new(Contacts::Email).string().null())
                    .col(ColumnDef::new(Contacts::Email2).string().null())
                    .col(ColumnDef::new(Contacts::Phone).string().null())
                    .col(ColumnDef::new(Contacts::Phone2).string().null())
                    .col(ColumnDef::new(Contacts::Mobile).string().null())
                    .col(ColumnDef::new(Contacts::Fax).string().null())
                    .col(ColumnDef::new(Contacts::Website).string().null())
                    .col(ColumnDef::new(Contacts::VatNumber).string().null())
                    .col(ColumnDef::new(Contacts::Language).string().null())
                    .col(ColumnDef::new(Contacts::Notes).text().null())
                    .col(ColumnDef::new(Contacts::BexioId).integer().null())
                    .col(ColumnDef::new(Contacts::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Contacts::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Contacts::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Contact persons (sub-contacts within a company)
        manager
            .create_table(
                Table::create()
                    .table(ContactPersons::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ContactPersons::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(ContactPersons::ContactId).string().not_null())
                    .col(ColumnDef::new(ContactPersons::FirstName).string().null())
                    .col(ColumnDef::new(ContactPersons::LastName).string().null())
                    .col(ColumnDef::new(ContactPersons::Email).string().null())
                    .col(ColumnDef::new(ContactPersons::Phone).string().null())
                    .col(ColumnDef::new(ContactPersons::Department).string().null())
                    .col(ColumnDef::new(ContactPersons::Position).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ContactPersons::Table, ContactPersons::ContactId)
                            .to(Contacts::Table, Contacts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ContactPersons::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Contacts::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Contacts {
    Table,
    Id,
    ContactType,
    Category,
    Industry,
    Name1,
    Name2,
    Salutation,
    Title,
    Address,
    PostalCode,
    City,
    Country,
    Email,
    Email2,
    Phone,
    Phone2,
    Mobile,
    Fax,
    Website,
    VatNumber,
    Language,
    Notes,
    BexioId,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum ContactPersons {
    Table,
    Id,
    ContactId,
    FirstName,
    LastName,
    Email,
    Phone,
    Department,
    Position,
}
