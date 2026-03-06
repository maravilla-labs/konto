use sea_orm::*;

use crate::entities::payroll_run_line::{self, Entity as PayrollRunLineEntity};

pub struct PayrollRunLineRepo;

impl PayrollRunLineRepo {
    pub async fn find_by_run(
        db: &DatabaseConnection,
        run_id: &str,
    ) -> Result<Vec<payroll_run_line::Model>, DbErr> {
        PayrollRunLineEntity::find()
            .filter(payroll_run_line::Column::PayrollRunId.eq(run_id))
            .all(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: payroll_run_line::ActiveModel,
    ) -> Result<payroll_run_line::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn delete_by_run(
        db: &DatabaseConnection,
        run_id: &str,
    ) -> Result<DeleteResult, DbErr> {
        PayrollRunLineEntity::delete_many()
            .filter(payroll_run_line::Column::PayrollRunId.eq(run_id))
            .exec(db)
            .await
    }
}
