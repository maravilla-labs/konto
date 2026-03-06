use sea_orm::*;

use crate::entities::payout_entry::{self, Entity as PayoutEntryEntity};

pub struct PayoutEntryRepo;

impl PayoutEntryRepo {
    pub async fn find_by_run_id(
        db: &DatabaseConnection,
        run_id: &str,
    ) -> Result<Vec<payout_entry::Model>, DbErr> {
        PayoutEntryEntity::find()
            .filter(payout_entry::Column::PayrollRunId.eq(run_id))
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<payout_entry::Model>, DbErr> {
        PayoutEntryEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: payout_entry::ActiveModel,
    ) -> Result<payout_entry::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: payout_entry::ActiveModel,
    ) -> Result<payout_entry::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete_by_run_id(
        db: &DatabaseConnection,
        run_id: &str,
    ) -> Result<DeleteResult, DbErr> {
        PayoutEntryEntity::delete_many()
            .filter(payout_entry::Column::PayrollRunId.eq(run_id))
            .exec(db)
            .await
    }
}
