use sea_orm::*;

use crate::entities::project::{self, ActiveModel, Entity as ProjectEntity};
use crate::entities::time_entry::{self, Entity as TimeEntryEntity};

pub struct ProjectRepo;

impl ProjectRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        search: Option<&str>,
    ) -> Result<(Vec<project::Model>, u64), DbErr> {
        let mut query = ProjectEntity::find().order_by_desc(project::Column::UpdatedAt);

        if let Some(search) = search {
            query = query.filter(
                Condition::any()
                    .add(project::Column::Name.contains(search))
                    .add(project::Column::Number.contains(search)),
            );
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> Result<Option<project::Model>, DbErr> {
        ProjectEntity::find_by_id(id).one(db).await
    }

    pub async fn create(db: &DatabaseConnection, model: ActiveModel) -> Result<project::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(db: &DatabaseConnection, model: ActiveModel) -> Result<project::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        ProjectEntity::delete_by_id(id).exec(db).await
    }
}

pub struct TimeEntryRepo;

impl TimeEntryRepo {
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        project_id: Option<&str>,
        billed: Option<bool>,
        status: Option<&str>,
        billable: Option<bool>,
    ) -> Result<(Vec<time_entry::Model>, u64), DbErr> {
        let mut query = TimeEntryEntity::find().order_by_desc(time_entry::Column::Date);

        if let Some(pid) = project_id {
            query = query.filter(time_entry::Column::ProjectId.eq(pid));
        }

        if let Some(is_billed) = billed {
            query = query.filter(time_entry::Column::Billed.eq(is_billed));
        }

        if let Some(s) = status {
            query = query.filter(time_entry::Column::Status.eq(s));
        }

        if let Some(b) = billable {
            query = query.filter(time_entry::Column::Billable.eq(b));
        }

        let paginator = query.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<time_entry::Model>, DbErr> {
        TimeEntryEntity::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        model: time_entry::ActiveModel,
    ) -> Result<time_entry::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: time_entry::ActiveModel,
    ) -> Result<time_entry::Model, DbErr> {
        model.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<DeleteResult, DbErr> {
        TimeEntryEntity::delete_by_id(id).exec(db).await
    }
}
