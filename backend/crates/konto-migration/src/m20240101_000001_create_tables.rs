use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Roles table
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Roles::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Roles::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Roles::Permissions).text().not_null())
                    .col(ColumnDef::new(Roles::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Roles::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Users::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Users::FullName).string().not_null())
                    .col(ColumnDef::new(Users::RoleId).string().not_null())
                    .col(ColumnDef::new(Users::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Users::Table, Users::RoleId)
                            .to(Roles::Table, Roles::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Audit log table
        manager
            .create_table(
                Table::create()
                    .table(AuditLog::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AuditLog::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(AuditLog::UserId).string().null())
                    .col(ColumnDef::new(AuditLog::Action).string().not_null())
                    .col(ColumnDef::new(AuditLog::EntityType).string().not_null())
                    .col(ColumnDef::new(AuditLog::EntityId).string().null())
                    .col(ColumnDef::new(AuditLog::OldValues).text().null())
                    .col(ColumnDef::new(AuditLog::NewValues).text().null())
                    .col(ColumnDef::new(AuditLog::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Currencies table
        manager
            .create_table(
                Table::create()
                    .table(Currencies::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Currencies::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Currencies::Code).string().not_null().unique_key())
                    .col(ColumnDef::new(Currencies::Name).string().not_null())
                    .col(ColumnDef::new(Currencies::Symbol).string().not_null())
                    .col(ColumnDef::new(Currencies::IsPrimary).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AuditLog::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Roles::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Currencies::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Roles {
    Table,
    Id,
    Name,
    Permissions,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Email,
    PasswordHash,
    FullName,
    RoleId,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum AuditLog {
    Table,
    Id,
    UserId,
    Action,
    EntityType,
    EntityId,
    OldValues,
    NewValues,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum Currencies {
    Table,
    Id,
    Code,
    Name,
    Symbol,
    IsPrimary,
}
