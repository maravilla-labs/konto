use konto_common::error::AppError;
use sea_orm::DatabaseConnection;

use super::contacts_csv::{parse_contacts_csv, ContactPersonRow};
use crate::services::contact_relationship_service::ContactRelationshipService;
use crate::services::contact_service::ContactService;

/// Execute contacts import: creates contacts, then person contacts
/// and contact_relationships for any Kontaktperson data found.
pub async fn execute_contacts_import(
    db: &DatabaseConnection,
    data: &[u8],
) -> Result<(i32, i32, Vec<String>), AppError> {
    let rows = parse_contacts_csv(data)?;
    let mut imported = 0i32;
    let mut errors = 0i32;
    let mut error_log = Vec::new();

    // Collect contact persons to process after all contacts are created.
    // Each entry: (company_contact_id, ContactPersonRow, is_primary)
    let mut pending_persons: Vec<(String, ContactPersonRow, bool)> = Vec::new();

    for row in &rows {
        let result = ContactService::create(
            db,
            &row.contact_type,
            &row.name1,
            row.name2.clone(),
            row.email.clone(),
            row.phone.clone(),
            row.address.clone(),
            row.postal_code.clone(),
            row.city.clone(),
            row.country.clone(),
            row.language.clone(),
            None, // notes
            None, // salutation
            None, // title
            None, // email2
            None, // phone2
            None, // mobile
            None, // fax
            None, // industry
            None, // birthday
            None, // employee_count
            None, // trade_register_number
            None, // salutation_form
            None, // website
            None, // vat_number
            // Derive category from contact_type: "company" → company, else person
            Some(if row.contact_type == "company" { "company".to_string() } else { "person".to_string() }),
            None, // vat_mode
            None, // customer_number
        )
        .await;

        match result {
            Ok(contact) => {
                imported += 1;
                // If this is a company contact, queue its Kontaktpersonen
                if row.contact_type == "company" {
                    for (idx, kp) in row.contact_persons.iter().enumerate() {
                        pending_persons.push((
                            contact.id.clone(),
                            kp.clone(),
                            idx == 0, // first contact person is primary
                        ));
                    }
                }
            }
            Err(e) => {
                error_log.push(format!("Contact '{}': {}", row.name1, e));
                errors += 1;
            }
        }
    }

    // Post-processing: create person contacts and relationships
    let mut relationships_created = 0i32;
    for (company_id, kp, is_primary) in &pending_persons {
        let full_name = kp.full_name();
        if full_name.is_empty() {
            continue;
        }

        // Create the person contact
        let person_id = match create_contact_person(db, kp).await {
            Ok(id) => id,
            Err(e) => {
                error_log.push(format!(
                    "Kontaktperson '{}': {}",
                    full_name, e
                ));
                continue;
            }
        };

        // Create the contact_relationship
        match ContactRelationshipService::create(
            db,
            &person_id,
            company_id,
            Some("Kontaktperson".to_string()),
            None,  // position
            None,  // department
            *is_primary,
            None,  // notes
        )
        .await
        {
            Ok(_) => {
                relationships_created += 1;
                imported += 1; // count relationship as imported row
            }
            Err(e) => {
                // If relationship already exists, skip silently
                let err_str = e.to_string();
                if err_str.contains("UNIQUE") || err_str.contains("duplicate") {
                    continue;
                }
                error_log.push(format!(
                    "Relationship '{}' <-> company: {}",
                    full_name, e
                ));
            }
        }
    }

    if relationships_created > 0 {
        error_log.insert(
            0,
            format!("Created {} contact relationships", relationships_created),
        );
    }

    Ok((imported, errors, error_log))
}

/// Create a person contact from Kontaktperson CSV data.
/// Uses nachname as name1 and vorname as name2 (matching CSV import convention).
async fn create_contact_person(
    db: &DatabaseConnection,
    kp: &ContactPersonRow,
) -> Result<String, AppError> {
    // For person contacts: name1 = nachname (last name), name2 = vorname (first name)
    let name1 = kp
        .nachname
        .clone()
        .or_else(|| kp.vorname.clone())
        .unwrap_or_default();
    let name2 = if kp.nachname.is_some() {
        kp.vorname.clone()
    } else {
        None
    };

    let contact = ContactService::create(
        db,
        "person",
        &name1,
        name2,
        kp.email.clone(),
        kp.phone.clone(),
        kp.address.clone(),
        kp.postal_code.clone(),
        kp.city.clone(),
        kp.country.clone(),
        kp.language.clone(),
        kp.notes.clone(),        // notes
        kp.anrede.clone(),       // salutation
        kp.titel.clone(),        // title
        kp.email2.clone(),       // email2
        kp.phone2.clone(),       // phone2
        kp.mobile.clone(),       // mobile
        kp.fax.clone(),          // fax
        None,                    // industry
        kp.birthday.clone(),     // birthday
        None,                    // employee_count
        None,                    // trade_register_number
        kp.anredeform.clone(),   // salutation_form
        kp.website.clone(),      // website
        None,                    // vat_number
        Some("person".to_string()), // category — Kontaktpersonen are always persons
        None,                    // vat_mode
        None,                    // customer_number
    )
    .await?;

    Ok(contact.id)
}
