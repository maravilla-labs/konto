use sea_orm_migration::prelude::*;

use crate::m20240101_000004_create_projects::ActivityTypes;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ActivityTypes::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("unit_type"))
                            .string()
                            .not_null()
                            .default("hour"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(ActivityTypes::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("default_rate"))
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Seed default rates for existing hourly activity types
        let rates: Vec<(&str, &str)> = vec![
            ("act-admin", "120.00"),
            ("act-general", "120.00"),
            ("act-ba", "180.00"),
            ("act-meeting", "150.00"),
            ("act-pm", "180.00"),
            ("act-sa", "200.00"),
            ("act-dev", "180.00"),
            ("act-sysarch", "200.00"),
            ("act-doc", "150.00"),
            ("act-travel", "0.00"),
            ("act-impl", "180.00"),
        ];
        for (id, rate) in rates {
            manager
                .exec_stmt(
                    Query::update()
                        .table(ActivityTypes::Table)
                        .value(Alias::new("default_rate"), rate)
                        .and_where(Expr::col(ActivityTypes::Id).eq(id))
                        .to_owned(),
                )
                .await?;
        }

        // Seed non-hour unit type presets
        let presets: Vec<(&str, &str, &str, &str)> = vec![
            // (id, name, unit_type, default_rate)
            ("act-painting", "Malerarbeiten", "sqm", "45.00"),
            ("act-site-visit", "Baustellenbesuch", "fixed", "250.00"),
            ("act-transport", "Transport / Fahrt", "km", "0.70"),
            ("act-daily", "Tagessatz", "day", "1400.00"),
            ("act-piece", "Stückarbeit", "piece", "0.00"),
        ];
        for (id, name, unit_type, rate) in presets {
            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(ActivityTypes::Table)
                        .columns([
                            Alias::new("id"),
                            Alias::new("name"),
                            Alias::new("is_active"),
                            Alias::new("unit_type"),
                            Alias::new("default_rate"),
                        ])
                        .values_panic([
                            id.into(),
                            name.into(),
                            true.into(),
                            unit_type.into(),
                            rate.into(),
                        ])
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove seeded non-hour presets
        let preset_ids = vec!["act-painting", "act-site-visit", "act-transport", "act-daily", "act-piece"];
        for id in preset_ids {
            let _ = manager
                .exec_stmt(
                    Query::delete()
                        .from_table(ActivityTypes::Table)
                        .and_where(Expr::col(ActivityTypes::Id).eq(id))
                        .to_owned(),
                )
                .await;
        }

        let _ = manager
            .alter_table(
                Table::alter()
                    .table(ActivityTypes::Table)
                    .drop_column(Alias::new("default_rate"))
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(ActivityTypes::Table)
                    .drop_column(Alias::new("unit_type"))
                    .to_owned(),
            )
            .await;
        Ok(())
    }
}
