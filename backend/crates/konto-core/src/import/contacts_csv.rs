use konto_common::error::AppError;
use serde::Serialize;
use std::io::Cursor;

/// Parsed contact person (Kontaktperson) data from CSV import.
/// Each company row can have up to 3 contact persons.
#[derive(Debug, Serialize, Clone)]
pub struct ContactPersonRow {
    pub vorname: Option<String>,
    pub nachname: Option<String>,
    pub anrede: Option<String>,
    pub titel: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub email2: Option<String>,
    pub phone: Option<String>,
    pub phone2: Option<String>,
    pub mobile: Option<String>,
    pub fax: Option<String>,
    pub website: Option<String>,
    pub anredeform: Option<String>,
    pub birthday: Option<String>,
    pub notes: Option<String>,
    pub language: Option<String>,
}

impl ContactPersonRow {
    /// Returns true if this contact person has at least a first or last name.
    pub fn has_name(&self) -> bool {
        self.vorname.is_some() || self.nachname.is_some()
    }

    /// Returns the full name as "Vorname Nachname", handling missing parts.
    pub fn full_name(&self) -> String {
        match (&self.vorname, &self.nachname) {
            (Some(v), Some(n)) => format!("{} {}", v, n),
            (Some(v), None) => v.clone(),
            (None, Some(n)) => n.clone(),
            (None, None) => String::new(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ContactRow {
    pub bexio_id: Option<i32>,
    pub contact_type: String,
    pub category: Option<String>,
    pub name1: String,
    pub name2: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub website: Option<String>,
    pub vat_number: Option<String>,
    pub language: Option<String>,
    pub notes: Option<String>,
    pub contact_persons: Vec<ContactPersonRow>,
}

fn clean(s: &str) -> Option<String> {
    let trimmed = s.trim().trim_start_matches('\'').trim();
    if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
}

pub fn parse_contacts_csv(data: &[u8]) -> Result<Vec<ContactRow>, AppError> {
    let cursor = Cursor::new(data);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .flexible(true)
        .from_reader(cursor);

    let headers = rdr.headers()
        .map_err(|e| AppError::BadRequest(format!("Invalid CSV: {e}")))?
        .clone();

    let mut rows = Vec::new();

    for result in rdr.records() {
        let record = result.map_err(|e| AppError::BadRequest(format!("CSV parse error: {e}")))?;

        let get = |name: &str| -> Option<String> {
            headers.iter().position(|h| h == name)
                .and_then(|i| record.get(i))
                .and_then(clean)
        };

        let bexio_id = get("Datensatz ID").and_then(|s| s.parse().ok());
        let contact_type = match get("Kontaktart").as_deref() {
            Some("Firma") => "company",
            _ => "person",
        };

        // Parse Kontaktperson 1-3
        let mut contact_persons = Vec::new();
        for n in 1..=3 {
            let prefix = format!("Kontaktperson {n}");
            let kp = ContactPersonRow {
                vorname: get(&format!("{prefix} Vorname")),
                nachname: get(&format!("{prefix} Nachname")),
                anrede: get(&format!("{prefix} Anrede")),
                titel: get(&format!("{prefix} Titel")),
                address: get(&format!("{prefix} Adresse")),
                postal_code: get(&format!("{prefix} PLZ")),
                city: get(&format!("{prefix} Ort")),
                country: get(&format!("{prefix} Land")),
                email: get(&format!("{prefix} E-Mail")),
                email2: get(&format!("{prefix} E-Mail 2")),
                phone: get(&format!("{prefix} Telefon")),
                phone2: get(&format!("{prefix} Telefon 2")),
                mobile: get(&format!("{prefix} Mobile")),
                fax: get(&format!("{prefix} Fax")),
                website: get(&format!("{prefix} Website")),
                anredeform: get(&format!("{prefix} Anredeform")),
                birthday: get(&format!("{prefix} Geburtstag")),
                notes: get(&format!("{prefix} Bemerkungen")),
                language: get(&format!("{prefix} Sprache")),
            };
            if kp.has_name() {
                contact_persons.push(kp);
            }
        }

        rows.push(ContactRow {
            bexio_id,
            contact_type: contact_type.to_string(),
            category: get("Kategorie"),
            name1: get("Name 1").unwrap_or_default(),
            name2: get("Name 2"),
            address: get("Adresse"),
            postal_code: get("PLZ"),
            city: get("Ort"),
            country: get("Land"),
            email: get("E-Mail"),
            phone: get("Telefon"),
            mobile: get("Mobile"),
            website: get("Website"),
            vat_number: get("MWST-Nummer"),
            language: get("Sprache"),
            notes: get("Bemerkungen"),
            contact_persons,
        });
    }

    Ok(rows)
}
