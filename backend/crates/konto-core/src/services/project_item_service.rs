use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::TimeEntryStatus;
use konto_db::entities::project_item;
use konto_db::repository::project_item_repo::ProjectItemRepo;
use sea_orm::{DatabaseConnection, Set};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProjectItemTree {
    #[serde(flatten)]
    pub item: project_item::Model,
    pub children: Vec<ProjectItemTree>,
}

pub struct ProjectItemService;

impl ProjectItemService {
    /// Fetch all items for a project flat, then build an in-memory tree.
    pub async fn list_tree(
        db: &DatabaseConnection,
        project_id: &str,
    ) -> Result<Vec<ProjectItemTree>, AppError> {
        let items = ProjectItemRepo::find_by_project(db, project_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(Self::build_tree(items))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<project_item::Model, AppError> {
        ProjectItemRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Project item not found".into()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        project_id: &str,
        parent_id: Option<String>,
        item_type: &str,
        name: &str,
        description: Option<String>,
        assignee_id: Option<String>,
        start_date: Option<chrono::NaiveDate>,
        due_date: Option<chrono::NaiveDate>,
        estimated_hours: Option<rust_decimal::Decimal>,
        budget_hours: Option<rust_decimal::Decimal>,
        budget_amount: Option<rust_decimal::Decimal>,
        sort_order: i32,
        user_id: &str,
    ) -> Result<project_item::Model, AppError> {
        Self::validate_nesting(db, item_type, parent_id.as_deref()).await?;

        let now = Utc::now().naive_utc();
        let model = project_item::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            project_id: Set(project_id.to_string()),
            parent_id: Set(parent_id),
            item_type: Set(item_type.to_string()),
            name: Set(name.to_string()),
            description: Set(description),
            status: Set(TimeEntryStatus::Pending.to_string()),
            assignee_id: Set(assignee_id),
            start_date: Set(start_date),
            due_date: Set(due_date),
            estimated_hours: Set(estimated_hours),
            budget_hours: Set(budget_hours),
            budget_amount: Set(budget_amount),
            sort_order: Set(sort_order),
            created_by: Set(Some(user_id.to_string())),
            updated_by: Set(Some(user_id.to_string())),
            created_at: Set(now),
            updated_at: Set(now),
        };
        ProjectItemRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        name: Option<String>,
        description: Option<Option<String>>,
        status: Option<String>,
        assignee_id: Option<Option<String>>,
        start_date: Option<Option<chrono::NaiveDate>>,
        due_date: Option<Option<chrono::NaiveDate>>,
        estimated_hours: Option<Option<rust_decimal::Decimal>>,
        budget_hours: Option<Option<rust_decimal::Decimal>>,
        budget_amount: Option<Option<rust_decimal::Decimal>>,
        sort_order: Option<i32>,
        user_id: &str,
    ) -> Result<project_item::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let now = Utc::now().naive_utc();
        let mut model: project_item::ActiveModel = existing.into();

        if let Some(v) = name {
            model.name = Set(v);
        }
        if let Some(v) = description {
            model.description = Set(v);
        }
        if let Some(v) = status {
            model.status = Set(v);
        }
        if let Some(v) = assignee_id {
            model.assignee_id = Set(v);
        }
        if let Some(v) = start_date {
            model.start_date = Set(v);
        }
        if let Some(v) = due_date {
            model.due_date = Set(v);
        }
        if let Some(v) = estimated_hours {
            model.estimated_hours = Set(v);
        }
        if let Some(v) = budget_hours {
            model.budget_hours = Set(v);
        }
        if let Some(v) = budget_amount {
            model.budget_amount = Set(v);
        }
        if let Some(v) = sort_order {
            model.sort_order = Set(v);
        }
        model.updated_by = Set(Some(user_id.to_string()));
        model.updated_at = Set(now);

        ProjectItemRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        // Check no children
        let children = ProjectItemRepo::find_children(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        if !children.is_empty() {
            return Err(AppError::Validation(
                "Cannot delete item with children. Remove children first.".into(),
            ));
        }
        ProjectItemRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn reorder(
        db: &DatabaseConnection,
        id: &str,
        new_parent_id: Option<String>,
        new_sort_order: i32,
        user_id: &str,
    ) -> Result<project_item::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;

        // Validate nesting for the new parent
        Self::validate_nesting(db, &existing.item_type, new_parent_id.as_deref())
            .await?;

        let now = Utc::now().naive_utc();
        let mut model: project_item::ActiveModel = existing.into();
        model.parent_id = Set(new_parent_id);
        model.sort_order = Set(new_sort_order);
        model.updated_by = Set(Some(user_id.to_string()));
        model.updated_at = Set(now);

        ProjectItemRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    // --- Private helpers ---

    async fn validate_nesting(
        db: &DatabaseConnection,
        item_type: &str,
        parent_id: Option<&str>,
    ) -> Result<(), AppError> {
        match item_type {
            "phase" => {
                if parent_id.is_some() {
                    return Err(AppError::Validation(
                        "Phase must be top-level (no parent)".into(),
                    ));
                }
            }
            "work_package" => {
                let parent_id = parent_id.ok_or_else(|| {
                    AppError::Validation(
                        "Work package must have a parent phase".into(),
                    )
                })?;
                let parent = ProjectItemRepo::find_by_id(db, parent_id)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?
                    .ok_or_else(|| AppError::NotFound("Parent not found".into()))?;
                if parent.item_type != "phase" {
                    return Err(AppError::Validation(
                        "Work package parent must be a phase".into(),
                    ));
                }
            }
            "task" => {
                let parent_id = parent_id.ok_or_else(|| {
                    AppError::Validation(
                        "Task must have a parent (phase or work_package)".into(),
                    )
                })?;
                let parent = ProjectItemRepo::find_by_id(db, parent_id)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?
                    .ok_or_else(|| AppError::NotFound("Parent not found".into()))?;
                if parent.item_type != "phase" && parent.item_type != "work_package" {
                    return Err(AppError::Validation(
                        "Task parent must be a phase or work_package".into(),
                    ));
                }
            }
            _ => {
                return Err(AppError::Validation(format!(
                    "Invalid item_type: {}. Must be phase, work_package, or task",
                    item_type
                )));
            }
        }
        Ok(())
    }

    fn build_tree(items: Vec<project_item::Model>) -> Vec<ProjectItemTree> {
        use std::collections::HashMap;

        let mut children_map: HashMap<Option<String>, Vec<project_item::Model>> =
            HashMap::new();
        for item in items {
            children_map
                .entry(item.parent_id.clone())
                .or_default()
                .push(item);
        }

        Self::build_subtree(&children_map, None)
    }

    fn build_subtree(
        children_map: &std::collections::HashMap<
            Option<String>,
            Vec<project_item::Model>,
        >,
        parent_id: Option<String>,
    ) -> Vec<ProjectItemTree> {
        let Some(items) = children_map.get(&parent_id) else {
            return vec![];
        };
        items
            .iter()
            .map(|item| {
                let children = Self::build_subtree(
                    children_map,
                    Some(item.id.clone()),
                );
                ProjectItemTree {
                    item: item.clone(),
                    children,
                }
            })
            .collect()
    }
}
