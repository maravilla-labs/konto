use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let defaults = [
            ("Acquisition", 1, "#f59e0b"),
            ("Tender", 2, "#3b82f6"),
            ("Lost", 3, "#ef4444"),
            ("Preparation", 4, "#8b5cf6"),
            ("In Progress", 5, "#22c55e"),
            ("Final Stage", 6, "#06b6d4"),
            ("Documentation", 7, "#6366f1"),
            ("Delivery", 8, "#10b981"),
        ];

        let now = chrono::Utc::now().naive_utc().to_string();

        for (name, sort_order, color) in defaults {
            let id = uuid::Uuid::new_v4().to_string();
            db.execute_unprepared(&format!(
                "INSERT INTO project_sub_statuses (id, name, sort_order, color, is_active, created_at, updated_at) VALUES ('{}', '{}', {}, '{}', 1, '{}', '{}')",
                id, name, sort_order, color, now, now
            ))
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DELETE FROM project_sub_statuses").await?;
        Ok(())
    }
}
