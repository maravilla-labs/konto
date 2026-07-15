use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "currencies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub code: String,
    pub name: String,
    pub symbol: String,
    pub is_primary: bool,
    pub default_bank_account_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bank_account::Entity",
        from = "Column::DefaultBankAccountId",
        to = "super::bank_account::Column::Id"
    )]
    BankAccount,
}

impl Related<super::bank_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BankAccount.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
