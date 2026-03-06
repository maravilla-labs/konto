use sea_orm::*;
use konto_common::enums::EmployeeStatus;

use crate::entities::employee::{self, Entity as EmployeeEntity};

pub struct EmployeeRepo;

impl EmployeeRepo {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<employee::Model>, DbErr> {
        EmployeeEntity::find()
            .order_by_asc(employee::Column::LastName)
            .all(db)
            .await
    }

    pub async fn find_active(db: &DatabaseConnection) -> Result<Vec<employee::Model>, DbErr> {
        EmployeeEntity::find()
            .filter(employee::Column::Status.eq(EmployeeStatus::Active.as_str()))
            .order_by_asc(employee::Column::LastName)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<employee::Model>, DbErr> {
        EmployeeEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: employee::ActiveModel,
    ) -> Result<employee::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: employee::ActiveModel,
    ) -> Result<employee::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        EmployeeEntity::delete_by_id(id).exec(db).await
    }
}
