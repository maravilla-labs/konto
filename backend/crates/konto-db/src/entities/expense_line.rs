use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "expense_line")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub expense_id: String,
    pub position: i32,
    pub expense_date: Date,
    pub description: String,
    pub air_transport: Decimal,
    pub lodging: Decimal,
    pub fuel_mileage: Decimal,
    pub phone: Decimal,
    pub meals_tips: Decimal,
    pub entertainment: Decimal,
    pub other: Decimal,
    pub line_total: Decimal,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::expense::Entity",
        from = "Column::ExpenseId",
        to = "super::expense::Column::Id"
    )]
    Expense,
}

impl Related<super::expense::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Expense.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
