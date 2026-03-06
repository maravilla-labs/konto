use sea_orm::*;

use crate::entities::vat_rate::{self, Entity as VatRateEntity};

pub struct VatRateRepo;

impl VatRateRepo {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<vat_rate::Model>, DbErr> {
        VatRateEntity::find()
            .order_by_asc(vat_rate::Column::Code)
            .all(db)
            .await
    }

    pub async fn find_active(db: &DatabaseConnection) -> Result<Vec<vat_rate::Model>, DbErr> {
        VatRateEntity::find()
            .filter(vat_rate::Column::IsActive.eq(true))
            .order_by_asc(vat_rate::Column::Code)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<vat_rate::Model>, DbErr> {
        VatRateEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: vat_rate::ActiveModel,
    ) -> Result<vat_rate::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: vat_rate::ActiveModel,
    ) -> Result<vat_rate::Model, DbErr> {
        model.update(db).await
    }
}
