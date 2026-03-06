use sea_orm_migration::prelude::*;

use crate::m20240101_000033_create_shareholders::Shareholders;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        seed_shareholders(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DELETE FROM shareholders").await?;
        Ok(())
    }
}

async fn seed_shareholders(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let shareholders = vec![
        (
            "Doe, John",
            "Zurich",
            "Gesellschafter und Geschäftsführer",
            "Einzelunterschrift",
            1,
        ),
        (
            "Doe, Jane",
            "Zurich",
            "Gesellschafterin",
            "",
            2,
        ),
    ];

    for (name, city, role, signing, sort) in shareholders {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().naive_utc().to_string();

        let signing_val: SimpleExpr = if signing.is_empty() {
            Keyword::Null.into()
        } else {
            signing.into()
        };

        let insert = Query::insert()
            .into_table(Shareholders::Table)
            .columns([
                Shareholders::Id,
                Shareholders::Name,
                Shareholders::City,
                Shareholders::Role,
                Shareholders::SigningRights,
                Shareholders::SortOrder,
                Shareholders::CreatedAt,
                Shareholders::UpdatedAt,
            ])
            .values_panic([
                id.into(),
                name.into(),
                city.into(),
                role.into(),
                signing_val,
                sort.into(),
                now.clone().into(),
                now.into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;
    }

    Ok(())
}
