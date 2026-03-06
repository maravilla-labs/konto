use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "expense_receipt")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub expense_id: String,
    pub line_id: Option<String>,
    pub file_name: String,
    pub storage_key: String,
    pub file_size: i64,
    pub mime_type: String,
    pub uploaded_at: DateTimeUtc,
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
