use axum::extract::{Path, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::Json;
use konto_common::error::AppError;
use konto_core::services::lohnausweis_service::{LohnausweisData, LohnausweisService};
use konto_core::services::pdf_lohnausweis::PdfLohnausweisService;

use crate::state::AppState;

/// List salary certificate data for all employees in a year.
#[utoipa::path(
    get, path = "/api/v1/salary-certificates/{year}",
    responses((status = 200, body = Vec<LohnausweisData>)),
    security(("bearer" = [])),
    tag = "salary-certificates"
)]
pub async fn list_salary_certificates(
    State(state): State<AppState>,
    Path(year): Path<i32>,
) -> Result<Json<Vec<LohnausweisData>>, AppError> {
    let data = LohnausweisService::list_for_year(&state.db, year).await?;
    Ok(Json(data))
}

/// Download salary certificate PDF for a single employee.
#[utoipa::path(
    get, path = "/api/v1/salary-certificates/{year}/{employee_id}/pdf",
    responses((status = 200, content_type = "application/pdf")),
    security(("bearer" = [])),
    tag = "salary-certificates"
)]
pub async fn download_salary_certificate_pdf(
    State(state): State<AppState>,
    Path((year, employee_id)): Path<(i32, String)>,
) -> Result<impl IntoResponse, AppError> {
    let pdf = PdfLohnausweisService::generate(&state.db, year, &employee_id).await?;
    let filename = format!("salary-certificate-{year}-{employee_id}.pdf");
    Ok((
        [
            (header::CONTENT_TYPE, "application/pdf".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        pdf,
    ))
}

/// Download all salary certificates as ZIP for a year.
#[utoipa::path(
    get, path = "/api/v1/salary-certificates/{year}/zip",
    responses((status = 200, content_type = "application/zip")),
    security(("bearer" = [])),
    tag = "salary-certificates"
)]
pub async fn download_salary_certificates_zip(
    State(state): State<AppState>,
    Path(year): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let all = PdfLohnausweisService::generate_all(&state.db, year).await?;
    let zip_bytes = create_zip(&all, "salary-certificate", year)?;
    let filename = format!("salary-certificates-{year}.zip");
    Ok((
        [
            (header::CONTENT_TYPE, "application/zip".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        zip_bytes,
    ))
}

fn create_zip(
    files: &[(String, Vec<u8>)],
    prefix: &str,
    year: i32,
) -> Result<Vec<u8>, AppError> {
    use std::io::{Cursor, Write};
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    let buf = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(buf);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    for (name, pdf) in files {
        let filename = format!("{prefix}-{year}-{name}.pdf");
        zip.start_file(filename, options)
            .map_err(|e| AppError::Internal(format!("ZIP error: {e}")))?;
        zip.write_all(pdf)
            .map_err(|e| AppError::Internal(format!("ZIP write error: {e}")))?;
    }

    let result = zip
        .finish()
        .map_err(|e| AppError::Internal(format!("ZIP finish error: {e}")))?;
    Ok(result.into_inner())
}
