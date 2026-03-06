use konto_common::error::AppError;
use serde::Serialize;
use std::io::Cursor;

#[derive(Debug, Serialize, Clone)]
pub struct TimeEntryRow {
    pub contact_name: Option<String>,
    pub project_name: Option<String>,
    pub date: String,
    pub estimated_minutes: Option<i32>,
    pub actual_minutes: i32,
    pub flat_amount: Option<String>,
    pub activity: Option<String>,
    pub description: Option<String>,
    pub travel_minutes: Option<i32>,
    pub travel_flat_rate: Option<String>,
    pub travel_distance: Option<String>,
    pub status: Option<String>,
}

fn clean(s: &str) -> Option<String> {
    let trimmed = s.trim().trim_start_matches('\'').trim();
    if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
}

fn parse_duration_minutes(s: &str) -> Option<i32> {
    let trimmed = s.trim();
    if trimmed.is_empty() || trimmed == "00:00" {
        return None;
    }
    let parts: Vec<&str> = trimmed.split(':').collect();
    if parts.len() == 2 {
        let hours: i32 = parts[0].trim().parse().ok()?;
        let mins: i32 = parts[1].trim().parse().ok()?;
        Some(hours * 60 + mins)
    } else {
        None
    }
}

pub fn parse_time_entries_csv(data: &[u8]) -> Result<Vec<TimeEntryRow>, AppError> {
    let cursor = Cursor::new(data);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
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

        let actual = get("Effektiver Aufwand")
            .and_then(|s| parse_duration_minutes(&s))
            .unwrap_or(0);

        rows.push(TimeEntryRow {
            contact_name: get("Name 1"),
            project_name: get("Projekt"),
            date: get("Datum").unwrap_or_default(),
            estimated_minutes: get("Geschätzter Aufwand").and_then(|s| parse_duration_minutes(&s)),
            actual_minutes: actual,
            flat_amount: get("Pauschal-Betrag"),
            activity: get("Tätigkeit"),
            description: get("Text"),
            travel_minutes: get("Reisezeit").and_then(|s| parse_duration_minutes(&s)),
            travel_flat_rate: get("Reisepauschale"),
            travel_distance: get("Reisedistanz"),
            status: get("Status"),
        });
    }

    Ok(rows)
}
