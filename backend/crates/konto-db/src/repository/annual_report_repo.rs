use sea_orm::*;

use crate::entities::annual_report::{self, ActiveModel, Entity as ReportEntity};

pub struct AnnualReportRepo;

impl AnnualReportRepo {
    pub async fn find_by_fiscal_year(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
    ) -> Result<Option<annual_report::Model>, DbErr> {
        ReportEntity::find()
            .filter(annual_report::Column::FiscalYearId.eq(fiscal_year_id))
            .one(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<annual_report::Model>, DbErr> {
        ReportEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<annual_report::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: ActiveModel,
    ) -> Result<annual_report::Model, DbErr> {
        model.update(db).await
    }
}
