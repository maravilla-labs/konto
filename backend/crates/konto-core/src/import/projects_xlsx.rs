use calamine::{open_workbook_auto_from_rs, Reader, Data};
use konto_common::error::AppError;
use serde::Serialize;
use std::io::Cursor;

#[derive(Debug, Serialize, Clone)]
pub struct ProjectRow {
    pub bexio_id: Option<i32>,
    pub number: Option<String>,
    pub name: String,
    pub contact_name: Option<String>,
    pub contact_person: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub project_type: Option<String>,
}

fn cell_to_string(cell: &Data) -> Option<String> {
    match cell {
        Data::String(s) if !s.trim().is_empty() => Some(s.trim().to_string()),
        Data::Float(f) => Some(f.to_string()),
        Data::Int(i) => Some(i.to_string()),
        _ => None,
    }
}

pub fn parse_projects_xlsx(data: &[u8]) -> Result<Vec<ProjectRow>, AppError> {
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

        let name = match get("Projekt") {
            Some(n) => n,
            None => continue,
        };

        rows.push(ProjectRow {
            bexio_id: get("ID").and_then(|s| s.parse().ok()),
            number: get("Nr"),
            name,
            contact_name: get("Kontakt"),
            contact_person: get("Kontaktperson"),
            start_date: get("Start"),
            end_date: get("Ende"),
            status: get("Status"),
            description: get("Text"),
            project_type: get("Projekttyp"),
        });
    }

    Ok(rows)
}
