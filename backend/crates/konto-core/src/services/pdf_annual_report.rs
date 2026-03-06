use konto_common::error::AppError;
use sea_orm::DatabaseConnection;

use super::annual_report_service::AnnualReportService;
use super::pdf_ar_balance_sheet;
use super::pdf_ar_cover;
use super::pdf_ar_income_statement;
use super::pdf_ar_notes;
use super::pdf_ar_proposal;
use super::report_types::AnnualReportData;

pub struct PdfAnnualReportService;

impl PdfAnnualReportService {
    pub async fn generate(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
    ) -> Result<Vec<u8>, AppError> {
        let data = AnnualReportService::build_data(db, fiscal_year_id).await?;
        render_typst_pdf(&data)
    }
}

fn render_typst_pdf(data: &AnnualReportData) -> Result<Vec<u8>, AppError> {
    use typst::layout::PagedDocument;
    use typst_as_lib::typst_kit_options::TypstKitFontOptions;
    use typst_as_lib::TypstEngine;

    let typst_source = build_typst_source(data);

    let mut binaries: Vec<(&str, Vec<u8>)> = Vec::new();
    if let Some(logo) = load_logo() {
        binaries.push(("logo.png", logo));
    }

    let bin_refs: Vec<(&str, &[u8])> = binaries
        .iter()
        .map(|(name, data)| (*name, data.as_slice()))
        .collect();

    let font_opts = TypstKitFontOptions::new()
        .include_system_fonts(false)
        .include_embedded_fonts(true);

    let engine = TypstEngine::builder()
        .main_file(typst_source.as_str())
        .search_fonts_with(font_opts)
        .with_static_file_resolver(bin_refs)
        .build();

    let doc: PagedDocument = engine
        .compile()
        .output
        .map_err(|e| AppError::Internal(format!("Typst compile error: {e}")))?;

    let pdf = typst_pdf::pdf(&doc, &Default::default())
        .map_err(|e| AppError::Internal(format!("PDF generation failed: {e:?}")))?;
    Ok(pdf)
}

fn build_typst_source(data: &AnnualReportData) -> String {
    let cover = pdf_ar_cover::render(data);
    let balance_sheet = pdf_ar_balance_sheet::render(data);
    let income_statement = pdf_ar_income_statement::render(data);
    let notes = pdf_ar_notes::render(data);
    let proposal = pdf_ar_proposal::render(data);

    format!(
        r##"#set text(size: 9pt, font: "Noto Sans")
{cover}
{balance_sheet}
{income_statement}
{notes}
{proposal}"##
    )
}

fn load_logo() -> Option<Vec<u8>> {
    std::fs::read("uploads/logo.png")
        .or_else(|_| std::fs::read("plan/maravilla-logo.png"))
        .ok()
}

/// Escape text for Typst markup.
pub fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('@', "\\@")
        .replace('<', "\\<")
        .replace('>', "\\>")
        .replace('$', "\\$")
}

/// Format money with Swiss apostrophe separator.
pub fn fmt_money(d: rust_decimal::Decimal) -> String {
    let s = format!("{:.2}", d);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let dec_part = parts.get(1).unwrap_or(&"00");
    let negative = int_part.starts_with('-');
    let digits: String = int_part.chars().filter(|c| c.is_ascii_digit()).collect();
    let formatted: String = digits
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("'")
        .chars()
        .rev()
        .collect();
    if negative {
        format!("-{formatted}.{dec_part}")
    } else {
        format!("{formatted}.{dec_part}")
    }
}
