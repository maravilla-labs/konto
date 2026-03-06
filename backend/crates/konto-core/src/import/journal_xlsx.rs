use calamine::{open_workbook_auto_from_rs, Reader, Data};
use konto_common::error::AppError;
use serde::Serialize;
use std::io::Cursor;

#[derive(Debug, Serialize, Clone)]
pub struct JournalRow {
    pub bexio_id: Option<i32>,
    pub date: String,
    pub reference: Option<String>,
    pub debit_account: Option<i32>,
    pub credit_account: Option<i32>,
    pub description: Option<String>,
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub exchange_rate: Option<String>,
    pub base_amount: Option<String>,
    pub base_currency: Option<String>,
    pub vat_code: Option<String>,
}

fn cell_to_string(cell: &Data) -> Option<String> {
    match cell {
        Data::String(s) if !s.trim().is_empty() => Some(s.trim().to_string()),
        Data::Float(f) => Some(f.to_string()),
        Data::Int(i) => Some(i.to_string()),
        _ => None,
    }
}

/// Parse account number from strings like "1020 - Bank UBS (alt) CHF (UBS Switzerland AG)"
fn parse_account_number(s: &str) -> Option<i32> {
    s.split_whitespace().next()?.parse().ok()
}

pub fn parse_journal_xlsx(data: &[u8]) -> Result<Vec<JournalRow>, AppError> {
    let cursor = Cursor::new(data);
    let mut workbook = open_workbook_auto_from_rs(cursor)
        .map_err(|e| AppError::BadRequest(format!("Cannot open XLSX: {e}")))?;

    let sheet_name = workbook.sheet_names().first()
        .ok_or_else(|| AppError::BadRequest("No sheets in workbook".to_string()))?
        .clone();

    let range = workbook.worksheet_range(&sheet_name)
        .map_err(|e| AppError::BadRequest(format!("Cannot read sheet: {e}")))?;

    let mut rows_iter = range.rows();
    let header = rows_iter.next()
        .ok_or_else(|| AppError::BadRequest("Empty spreadsheet".to_string()))?;

    let col_index = |name: &str| -> Option<usize> {
        header.iter().position(|c| {
            if let Data::String(s) = c { s.trim() == name } else { false }
        })
    };

    let mut rows = Vec::new();
    for row in rows_iter {
        let get = |name: &str| -> Option<String> {
            col_index(name).and_then(|i| row.get(i)).and_then(cell_to_string)
        };

        let debit_str = get("Soll");
        let credit_str = get("Haben");

        rows.push(JournalRow {
            bexio_id: get("Id").and_then(|s| s.parse().ok()),
            date: get("Datum").unwrap_or_default(),
            reference: get("Referenz"),
            debit_account: debit_str.as_deref().and_then(parse_account_number),
            credit_account: credit_str.as_deref().and_then(parse_account_number),
            description: get("Beschreibung"),
            amount: get("Betrag"),
            currency: get("Buchungswährung"),
            exchange_rate: get("Umrechnungsfaktor"),
            base_amount: get("Betrag in Basiswährung"),
            base_currency: get("Währung in Basiswährung"),
            vat_code: get("MWST"),
        });
    }

    Ok(rows)
}
