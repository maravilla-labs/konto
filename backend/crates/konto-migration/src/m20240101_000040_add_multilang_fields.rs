use sea_orm_migration::prelude::*;

use crate::m20240101_000004_create_projects::Projects;
use crate::m20240101_000010_create_invoices::Invoices;
use crate::m20240101_000011_create_settings::CompanySettings;
use crate::m20240101_000013_create_documents::Documents;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(ColumnDef::new(Alias::new("language")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .add_column(ColumnDef::new(Alias::new("language")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Documents::Table)
                    .add_column(ColumnDef::new(Alias::new("language")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(CompanySettings::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("ui_language"))
                            .string()
                            .not_null()
                            .default("en"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                UPDATE projects
                SET language = COALESCE(
                    (SELECT c.language FROM contacts c WHERE c.id = projects.contact_id),
                    'en'
                )
                WHERE language IS NULL;
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                UPDATE invoices
                SET language = COALESCE(
                    (SELECT c.language FROM contacts c WHERE c.id = invoices.contact_id),
                    'en'
                )
                WHERE language IS NULL;
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                UPDATE documents
                SET language = COALESCE(
                    (SELECT c.language FROM contacts c WHERE c.id = documents.contact_id),
                    'en'
                )
                WHERE language IS NULL;
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .drop_column(Alias::new("language"))
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .drop_column(Alias::new("language"))
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Documents::Table)
                    .drop_column(Alias::new("language"))
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(CompanySettings::Table)
                    .drop_column(Alias::new("ui_language"))
                    .to_owned(),
            )
            .await;
        Ok(())
    }
}
