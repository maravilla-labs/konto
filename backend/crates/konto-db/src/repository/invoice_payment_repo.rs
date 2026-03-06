use sea_orm::*;

use crate::entities::invoice_payment::{self, Entity as InvoicePaymentEntity};

pub struct InvoicePaymentRepo;

impl InvoicePaymentRepo {
    pub async fn find_by_invoice(
        db: &DatabaseConnection,
        invoice_id: &str,
    ) -> Result<Vec<invoice_payment::Model>, DbErr> {
        InvoicePaymentEntity::find()
            .filter(invoice_payment::Column::InvoiceId.eq(invoice_id))
            .order_by_asc(invoice_payment::Column::PaymentDate)
            .all(db)
            .await
    }

    pub async fn sum_by_invoice(
        db: &DatabaseConnection,
        invoice_id: &str,
    ) -> Result<rust_decimal::Decimal, DbErr> {
        use sea_orm::sea_query::Expr;
        let result: Option<rust_decimal::Decimal> = InvoicePaymentEntity::find()
            .filter(invoice_payment::Column::InvoiceId.eq(invoice_id))
            .select_only()
            .column_as(Expr::col(invoice_payment::Column::Amount).sum(), "total")
            .into_tuple()
            .one(db)
            .await?;
        Ok(result.unwrap_or(rust_decimal::Decimal::ZERO))
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: invoice_payment::ActiveModel,
    ) -> Result<invoice_payment::Model, DbErr> {
        model.insert(db).await
    }
}
