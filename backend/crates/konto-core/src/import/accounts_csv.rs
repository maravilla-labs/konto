use konto_common::error::AppError;
use serde::Serialize;
use std::io::Cursor;

#[derive(Debug, Serialize, Clone)]
pub struct AccountRow {
    pub number: i32,
    pub name: String,
    pub account_type: String,
    pub description: Option<String>,
}

fn clean(s: &str) -> Option<String> {
    let trimmed = s.trim().trim_start_matches('\'').trim();
    if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
}

/// Infer Swiss KMU account type from account number.
fn infer_account_type(number: i32) -> &'static str {
    match number {
        1..=1999 => "asset",
        2000..=2799 => "liability",
        2800..=2999 => "equity",
        3000..=3999 => "revenue",
        _ => "expense",
    }
}

pub fn parse_accounts_csv(data: &[u8]) -> Result<Vec<AccountRow>, AppError> {
    let cursor = Cursor::new(data);
    // Try comma first, fall back to semicolon
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .flexible(true)
        .from_reader(cursor);

    let headers = rdr.headers()
        .map_err(|e| AppError::BadRequest(format!("Invalid CSV: {e}")))?
        .clone();

    // Check if we have the right columns — if not, try semicolon
    let has_number = headers.iter().any(|h| h == "number" || h == "Number" || h == "Konto" || h == "Nr");
    if !has_number {
        let cursor2 = Cursor::new(data);
        let mut rdr2 = csv::ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(true)
            .flexible(true)
            .from_reader(cursor2);

        let headers2 = rdr2.headers()
            .map_err(|e| AppError::BadRequest(format!("Invalid CSV: {e}")))?
            .clone();

        return parse_rows(&headers2, rdr2.records().collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::BadRequest(format!("CSV parse error: {e}")))?);
    }

    let records: Vec<_> = rdr.records().collect::<Result<Vec<_>, _>>()
        .map_err(|e| AppError::BadRequest(format!("CSV parse error: {e}")))?;

    parse_rows(&headers, records)
}

fn parse_rows(
    headers: &csv::StringRecord,
    records: Vec<csv::StringRecord>,
) -> Result<Vec<AccountRow>, AppError> {
    let mut rows = Vec::new();

    for record in records {
        let get = |names: &[&str]| -> Option<String> {
            for name in names {
                if let Some(val) = headers
                    .iter()
                    .position(|h| h == *name)
                    .and_then(|pos| record.get(pos))
                    .and_then(clean)
                {
                    return Some(val);
                }
            }
            None
        };

        let number_str = get(&["number", "Number", "Konto", "Nr", "Nr.", "Account"]);
        let number: i32 = match number_str.and_then(|s| {
            // Handle "1020 - Bank UBS" format
            s.split_whitespace().next()?.parse().ok()
        }) {
            Some(n) => n,
            None => continue,
        };

        let name = get(&["name", "Name", "Bezeichnung", "Description"])
            .unwrap_or_else(|| format!("Account {number}"));

        let account_type = get(&["type", "Type", "Typ", "account_type"])
            .unwrap_or_else(|| infer_account_type(number).to_string());

        let description = get(&["description", "Description", "Beschreibung"]);

        rows.push(AccountRow {
            number,
            name,
            account_type,
            description,
        });
    }

    Ok(rows)
}
