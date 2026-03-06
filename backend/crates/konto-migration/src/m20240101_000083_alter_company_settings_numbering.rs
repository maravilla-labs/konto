use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum CompanySettings {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    #[allow(clippy::unwrap_used)]
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let columns = vec![
            ("customer_number_auto", "boolean", "false"),
            ("customer_number_prefix", "string", "'K-'"),
            ("customer_number_restart_yearly", "boolean", "false"),
            ("customer_number_start", "integer", "1"),
            ("customer_number_min_length", "integer", "6"),
            ("employee_number_auto", "boolean", "false"),
            ("employee_number_prefix", "string", "'M-'"),
            ("employee_number_restart_yearly", "boolean", "false"),
            ("employee_number_start", "integer", "1"),
            ("employee_number_min_length", "integer", "3"),
        ];

        for (name, col_type, default) in &columns {
            let mut col = ColumnDef::new(Alias::new(*name));
            match *col_type {
                "boolean" => { col.boolean().not_null().default(Value::Bool(Some(*default == "true"))); }
                "integer" => { col.integer().not_null().default(default.parse::<i32>().unwrap()); }
                "string" => { col.string().not_null().default(default.trim_matches('\'')); }
                _ => {}
            }
            manager
                .alter_table(
                    Table::alter()
                        .table(CompanySettings::Table)
                        .add_column(col)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for col in [
            "employee_number_min_length",
            "employee_number_start",
            "employee_number_restart_yearly",
            "employee_number_prefix",
            "employee_number_auto",
            "customer_number_min_length",
            "customer_number_start",
            "customer_number_restart_yearly",
            "customer_number_prefix",
            "customer_number_auto",
        ] {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(CompanySettings::Table)
                        .drop_column(Alias::new(col))
                        .to_owned(),
                )
                .await;
        }
        Ok(())
    }
}
