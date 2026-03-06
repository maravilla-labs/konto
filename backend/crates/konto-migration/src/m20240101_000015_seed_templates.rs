use sea_orm_migration::prelude::*;

mod template_data;

#[derive(DeriveMigrationName)]
pub struct Migration;

fn now_str() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let now = now_str();

        for (id, name, ttype, content, header, footer, page_setup) in
            template_data::default_templates()
        {
            let header_sql = match header {
                Some(h) => format!("'{}'", h.replace('\'', "''")),
                None => "NULL".to_string(),
            };
            let footer_sql = match footer {
                Some(f) => format!("'{}'", f.replace('\'', "''")),
                None => "NULL".to_string(),
            };

            let sql = format!(
                "INSERT INTO document_templates \
                 (id, name, template_type, content_json, header_json, \
                  footer_json, page_setup_json, is_default, created_at, \
                  updated_at) \
                 VALUES ('{}', '{}', '{}', '{}', {}, {}, '{}', 1, '{}', '{}')",
                id,
                name,
                ttype,
                content.replace('\'', "''"),
                header_sql,
                footer_sql,
                page_setup.replace('\'', "''"),
                now,
                now,
            );

            manager
                .get_connection()
                .execute_unprepared(&sql)
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for id in [
            "tmpl-letterhead",
            "tmpl-invoice",
            "tmpl-sow",
            "tmpl-quote",
            "tmpl-contract",
        ] {
            manager
                .get_connection()
                .execute_unprepared(&format!(
                    "DELETE FROM document_templates WHERE id = '{id}'"
                ))
                .await?;
        }
        Ok(())
    }
}
