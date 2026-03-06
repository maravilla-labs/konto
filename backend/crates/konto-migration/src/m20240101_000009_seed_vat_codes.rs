use sea_orm_migration::prelude::*;

use crate::m20240101_000002_create_accounting::VatRates;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for (id, code, name, rate) in [
            ("vat-un77", "UN77", "Umsatzsteuer Nicht geschuldet 7.7%", 7.7),
            ("vat-vb77", "VB77", "Vorsteuer auf Betriebsaufwand 7.7%", 7.7),
            ("vat-vim", "VIM", "Vorsteuer auf Investitionen/Material", 7.7),
            ("vat-vm77", "VM77", "Vorsteuer auf Materialaufwand 7.7%", 7.7),
            ("vat-vm81", "VM81", "Vorsteuer auf Materialaufwand 8.1%", 8.1),
            ("vat-vsf", "VSF", "Vorsteuer pauschal", 0.0),
        ] {
            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(VatRates::Table)
                        .columns([
                            VatRates::Id,
                            VatRates::Code,
                            VatRates::Name,
                            VatRates::Rate,
                            VatRates::IsActive,
                        ])
                        .values_panic([
                            id.into(),
                            code.into(),
                            name.into(),
                            rate.into(),
                            true.into(),
                        ])
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for id in [
            "vat-un77", "vat-vb77", "vat-vim", "vat-vm77", "vat-vm81", "vat-vsf",
        ] {
            manager
                .exec_stmt(
                    Query::delete()
                        .from_table(VatRates::Table)
                        .and_where(Expr::col(VatRates::Id).eq(id))
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }
}
