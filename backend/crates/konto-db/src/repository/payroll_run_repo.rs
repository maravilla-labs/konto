use sea_orm::*;

use crate::entities::payroll_run::{self, Entity as PayrollRunEntity};

pub struct PayrollRunRepo;

impl PayrollRunRepo {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<payroll_run::Model>, DbErr> {
        PayrollRunEntity::find()
            .order_by_desc(payroll_run::Column::PeriodYear)
            .order_by_desc(payroll_run::Column::PeriodMonth)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<payroll_run::Model>, DbErr> {
        PayrollRunEntity::find_by_id(id).one(db).await
    }

    pub async fn find_by_period(
        db: &DatabaseConnection,
        month: i32,
        year: i32,
    ) -> Result<Option<payroll_run::Model>, DbErr> {
        PayrollRunEntity::find()
            .filter(payroll_run::Column::PeriodMonth.eq(month))
            .filter(payroll_run::Column::PeriodYear.eq(year))
            .one(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: payroll_run::ActiveModel,
    ) -> Result<payroll_run::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: payroll_run::ActiveModel,
    ) -> Result<payroll_run::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        PayrollRunEntity::delete_by_id(id).exec(db).await
    }
}
