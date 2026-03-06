use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::contact_relationship;
use konto_db::repository::contact_relationship_repo::ContactRelationshipRepo;
use konto_db::repository::contact_repo::ContactRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct ContactRelationshipService;

impl ContactRelationshipService {
    pub async fn list_for_contact(
        db: &DatabaseConnection,
        contact_id: &str,
    ) -> Result<Vec<contact_relationship::Model>, AppError> {
        ContactRelationshipRepo::find_by_contact(db, contact_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<contact_relationship::Model, AppError> {
        ContactRelationshipRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Contact relationship not found".into()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        person_contact_id: &str,
        org_contact_id: &str,
        role: Option<String>,
        position: Option<String>,
        department: Option<String>,
        is_primary: bool,
        notes: Option<String>,
    ) -> Result<contact_relationship::Model, AppError> {
        // Validate both contacts exist
        ContactRepo::find_by_id(db, person_contact_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| {
                AppError::Validation("Person contact not found".into())
            })?;

        ContactRepo::find_by_id(db, org_contact_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| {
                AppError::Validation("Organization contact not found".into())
            })?;

        let now = Utc::now().naive_utc();
        let model = contact_relationship::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            person_contact_id: Set(person_contact_id.to_string()),
            org_contact_id: Set(org_contact_id.to_string()),
            role: Set(role),
            position: Set(position),
            department: Set(department),
            is_primary: Set(is_primary),
            notes: Set(notes),
            created_at: Set(now),
            updated_at: Set(now),
        };

        ContactRelationshipRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        role: Option<Option<String>>,
        position: Option<Option<String>>,
        department: Option<Option<String>>,
        is_primary: Option<bool>,
        notes: Option<Option<String>>,
    ) -> Result<contact_relationship::Model, AppError> {
        let existing = Self::get_by_id(db, id).await?;
        let now = Utc::now().naive_utc();
        let mut model: contact_relationship::ActiveModel = existing.into();

        if let Some(r) = role {
            model.role = Set(r);
        }
        if let Some(p) = position {
            model.position = Set(p);
        }
        if let Some(d) = department {
            model.department = Set(d);
        }
        if let Some(p) = is_primary {
            model.is_primary = Set(p);
        }
        if let Some(n) = notes {
            model.notes = Set(n);
        }
        model.updated_at = Set(now);

        ContactRelationshipRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        ContactRelationshipRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
