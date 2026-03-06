use sea_orm::*;

use crate::entities::payroll_setting::{self, Entity as PayrollSettingEntity};

pub struct PayrollSettingRepo;

impl PayrollSettingRepo {
    pub async fn find(db: &DatabaseConnection) -> Result<Option<payroll_setting::Model>, DbErr> {
        PayrollSettingEntity::find().one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: payroll_setting::ActiveModel,
    ) -> Result<payroll_setting::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: payroll_setting::ActiveModel,
    ) -> Result<payroll_setting::Model, DbErr> {
        model.update(db).await
    }
}
