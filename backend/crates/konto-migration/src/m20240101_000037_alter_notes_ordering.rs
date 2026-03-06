use sea_orm_migration::prelude::*;

use crate::m20240101_000034_create_annual_report_notes::AnnualReportNotes;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add sort_order column
        manager
            .alter_table(
                Table::alter()
                    .table(AnnualReportNotes::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("sort_order"))
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        // Add label column
        manager
            .alter_table(
                Table::alter()
                    .table(AnnualReportNotes::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("label"))
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        // Add section_type column
        manager
            .alter_table(
                Table::alter()
                    .table(AnnualReportNotes::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("section_type"))
                            .text()
                            .not_null()
                            .default("text"),
                    )
                    .to_owned(),
            )
            .await?;

        // Backfill existing rows
        let db = manager.get_connection();
        let backfill = vec![
            (1, "accounting_principles", "Grundsätze der Rechnungslegung", "text"),
            (2, "general_info", "Angaben zur Gesellschaft", "auto_company_info"),
            (3, "audit_optout", "Verzicht auf Revision (Opting-out)", "text"),
            (4, "employees", "Anzahl Arbeitsstellen im Jahresdurchschnitt", "employees"),
            (5, "guarantees", "Eventualverpflichtungen", "text"),
            (6, "fx_rates", "Verwendete Fremdwährungskurse", "auto_fx_rates"),
            (7, "extraordinary", "Ausserordentliche Positionen", "text"),
            (8, "post_balance_events", "Ereignisse nach dem Bilanzstichtag", "text"),
        ];

        for (order, key, label, stype) in backfill {
            db.execute_unprepared(&format!(
                "UPDATE annual_report_notes SET sort_order = {}, label = '{}', section_type = '{}' WHERE section_key = '{}'",
                order, label, stype, key,
            ))
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for col in ["sort_order", "label", "section_type"] {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(AnnualReportNotes::Table)
                        .drop_column(Alias::new(col))
                        .to_owned(),
                )
                .await;
        }
        Ok(())
    }
}
