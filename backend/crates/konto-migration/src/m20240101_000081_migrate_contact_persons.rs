use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // For each contact_person, create a person-type contact and a contact_relationship.
        // Uses SQLite's INSERT...SELECT to create contacts from contact_persons.
        // The new contact id is derived from the contact_person id prefixed with 'cp-'.
        db.execute_unprepared(
            "INSERT OR IGNORE INTO contacts (id, contact_type, category, name1, name2, email, phone, \
             address, postal_code, city, country, is_active, created_at, updated_at, vat_mode) \
             SELECT \
               'cp-' || cp.id, \
               COALESCE(c.contact_type, 'customer'), \
               'person', \
               CASE \
                 WHEN cp.first_name IS NOT NULL AND cp.last_name IS NOT NULL \
                   THEN cp.first_name || ' ' || cp.last_name \
                 WHEN cp.first_name IS NOT NULL THEN cp.first_name \
                 WHEN cp.last_name IS NOT NULL THEN cp.last_name \
                 ELSE 'Unknown' \
               END, \
               c.name1, \
               cp.email, \
               cp.phone, \
               c.address, \
               c.postal_code, \
               c.city, \
               c.country, \
               1, \
               CURRENT_TIMESTAMP, \
               CURRENT_TIMESTAMP, \
               'auto' \
             FROM contact_persons cp \
             JOIN contacts c ON c.id = cp.contact_id"
        ).await?;

        // Create contact_relationships linking the new person contacts to their org contacts
        db.execute_unprepared(
            "INSERT OR IGNORE INTO contact_relationships (id, person_contact_id, org_contact_id, \
             role, position, department, is_primary, created_at, updated_at) \
             SELECT \
               'rel-' || cp.id, \
               'cp-' || cp.id, \
               cp.contact_id, \
               NULL, \
               cp.position, \
               cp.department, \
               0, \
               CURRENT_TIMESTAMP, \
               CURRENT_TIMESTAMP \
             FROM contact_persons cp"
        ).await?;

        // Update invoices.contact_person_id to point to new contact IDs
        db.execute_unprepared(
            "UPDATE invoices SET contact_person_id = 'cp-' || contact_person_id \
             WHERE contact_person_id IS NOT NULL \
             AND contact_person_id NOT LIKE 'cp-%'"
        ).await?;

        // Update projects.contact_person_id to point to new contact IDs
        db.execute_unprepared(
            "UPDATE projects SET contact_person_id = 'cp-' || contact_person_id \
             WHERE contact_person_id IS NOT NULL \
             AND contact_person_id NOT LIKE 'cp-%'"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Revert invoices.contact_person_id
        db.execute_unprepared(
            "UPDATE invoices SET contact_person_id = SUBSTR(contact_person_id, 4) \
             WHERE contact_person_id LIKE 'cp-%'"
        ).await?;

        // Revert projects.contact_person_id
        db.execute_unprepared(
            "UPDATE projects SET contact_person_id = SUBSTR(contact_person_id, 4) \
             WHERE contact_person_id LIKE 'cp-%'"
        ).await?;

        // Delete migrated relationships
        db.execute_unprepared(
            "DELETE FROM contact_relationships WHERE id LIKE 'rel-%'"
        ).await?;

        // Delete migrated person contacts
        db.execute_unprepared(
            "DELETE FROM contacts WHERE id LIKE 'cp-%'"
        ).await?;

        Ok(())
    }
}
