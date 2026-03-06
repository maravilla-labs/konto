use sea_orm::*;

use crate::entities::annual_report_note::{self, ActiveModel, Entity as NoteEntity};

pub struct AnnualReportNoteRepo;

impl AnnualReportNoteRepo {
    pub async fn find_by_fiscal_year(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
    ) -> Result<Vec<annual_report_note::Model>, DbErr> {
        NoteEntity::find()
            .filter(annual_report_note::Column::FiscalYearId.eq(fiscal_year_id))
            .order_by_asc(annual_report_note::Column::SortOrder)
            .all(db)
            .await
    }

    pub async fn find_by_key(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
        section_key: &str,
    ) -> Result<Option<annual_report_note::Model>, DbErr> {
        NoteEntity::find()
            .filter(annual_report_note::Column::FiscalYearId.eq(fiscal_year_id))
            .filter(annual_report_note::Column::SectionKey.eq(section_key))
            .one(db)
            .await
    }

    pub async fn upsert(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<annual_report_note::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<annual_report_note::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
        section_key: &str,
    ) -> Result<(), DbErr> {
        NoteEntity::delete_many()
            .filter(annual_report_note::Column::FiscalYearId.eq(fiscal_year_id))
            .filter(annual_report_note::Column::SectionKey.eq(section_key))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn find_max_sort_order(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
    ) -> Result<i32, DbErr> {
        let notes = NoteEntity::find()
            .filter(annual_report_note::Column::FiscalYearId.eq(fiscal_year_id))
            .order_by_desc(annual_report_note::Column::SortOrder)
            .one(db)
            .await?;
        Ok(notes.map(|n| n.sort_order).unwrap_or(0))
    }
}
