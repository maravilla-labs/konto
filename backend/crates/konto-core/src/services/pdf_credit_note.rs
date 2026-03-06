use konto_common::error::AppError;
use konto_db::entities::{company_setting, contact, credit_note, credit_note_line};
use konto_db::repository::contact_repo::ContactRepo;
use konto_db::repository::credit_note_repo::CreditNoteRepo;
use konto_db::repository::invoice_repo::InvoiceRepo;
use konto_db::repository::settings_repo::SettingsRepo;
use rust_decimal::Decimal;
use sea_orm::DatabaseConnection;

use super::language::normalize_or_default;

pub struct PdfCreditNoteService;

impl PdfCreditNoteService {
    pub async fn generate(db: &DatabaseConnection, id: &str) -> Result<Vec<u8>, AppError> {
        let data = fetch_data(db, id).await?;
        render_typst_pdf(&data)
    }
}

struct PdfData {
    credit_note: credit_note::Model,
    lines: Vec<credit_note_line::Model>,
    contact: contact::Model,
    settings: company_setting::Model,
    language: String,
    currency: String,
}

async fn fetch_data(db: &DatabaseConnection, id: &str) -> Result<PdfData, AppError> {
    let credit_note = CreditNoteRepo::find_by_id(db, id)
        .await.map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Credit note not found".into()))?;
    let lines = CreditNoteRepo::find_lines_by_credit_note(db, id)
        .await.map_err(|e| AppError::Database(e.to_string()))?;
    let contact = ContactRepo::find_by_id(db, &credit_note.contact_id)
        .await.map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Contact not found".into()))?;
    let settings = SettingsRepo::find(db)
        .await.map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Company settings not configured".into()))?;
    let invoice_language = if let Some(ref invoice_id) = credit_note.invoice_id {
        InvoiceRepo::find_by_id(db, invoice_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .and_then(|i| i.language)
    } else {
        None
    };
    let language = normalize_or_default(
        invoice_language.as_deref().or(contact.language.as_deref()),
        &settings.ui_language,
    );
    Ok(PdfData { credit_note, lines, contact, settings, language, currency: "CHF".into() })
}

fn render_typst_pdf(data: &PdfData) -> Result<Vec<u8>, AppError> {
    use typst::layout::PagedDocument;
    use typst_as_lib::TypstEngine;
    use typst_as_lib::typst_kit_options::TypstKitFontOptions;

    let typst_source = build_typst_source(data);

    let mut binaries: Vec<(&str, Vec<u8>)> = Vec::new();
    if let Some(logo) = load_logo(&data.settings) {
        binaries.push(("logo.png", logo));
    }

    let bin_refs: Vec<(&str, &[u8])> = binaries.iter()
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

    let doc: PagedDocument = engine.compile()
        .output
        .map_err(|e| AppError::Internal(format!("Typst compile error: {e}")))?;

    let pdf = typst_pdf::pdf(&doc, &Default::default())
        .map_err(|e| AppError::Internal(format!("PDF generation failed: {e:?}")))?;
    Ok(pdf)
}

fn build_typst_source(data: &PdfData) -> String {
    let s = &data.settings;
    let c = &data.contact;
    let cn = &data.credit_note;
    let cn_number = cn.credit_note_number.as_deref().unwrap_or("DRAFT");
    let labels = credit_note_labels(&data.language);

    let contact_name2_line = match c.name2.as_deref() {
        Some(n) if !n.is_empty() => format!("{}\\\n", esc(n)),
        _ => String::new(),
    };
    let contact_addr = c.address.as_deref().unwrap_or("");
    let contact_postal = c.postal_code.as_deref().unwrap_or("");
    let contact_city_val = c.city.as_deref().unwrap_or("");
    let contact_country = c.country.as_deref().unwrap_or("");

    let mut lines_markup = String::new();
    for line in &data.lines {
        lines_markup.push_str(&format!(
            "  [{}], [{}], [#align(right)[{}]], [#align(right)[{}]], [#align(right)[{}]], [#align(right)[{}]],\n",
            line.sort_order,
            esc(&line.description),
            fmt_qty(line.quantity),
            fmt_dec(line.unit_price),
            fmt_pct(line.vat_amount, cn.subtotal),
            fmt_money(line.line_total),
        ));
    }

    let vat_number = esc(s.vat_number.as_deref().unwrap_or(""));
    let email = esc(s.email.as_deref().unwrap_or(""));
    let has_logo = load_logo(s).is_some();

    let logo_block = if has_logo {
        r#"#image("logo.png", width: 40mm)"#.to_string()
    } else {
        let trade = s.trade_name.as_deref().unwrap_or(&s.legal_name);
        format!(r#"#text(size: 18pt, weight: "bold")[{}]"#, esc(trade))
    };

    format!(
        r##"#set page(paper: "a4", margin: (top: 20mm, bottom: 20mm, left: 20mm, right: 20mm))
#set text(size: 9pt, font: "Noto Sans")

// Header with logo
#grid(
  columns: (1fr, auto),
  [],
  align(right)[{logo_block}],
)

#v(12mm)

// Recipient address
{contact_name2_line}{contact_addr}\
{contact_postal} {contact_city}\
{contact_country}

#v(10mm)

// Credit Note title
#text(size: 14pt, weight: "bold")[{credit_note_title} {cn_number}]

#v(3mm)

// Metadata grid
#set text(size: 8pt)
#grid(
  columns: (55pt, auto, 1fr, 65pt, auto),
  column-gutter: 5pt,
  row-gutter: 3pt,
  [{date_label}:], [*{issue_date}*], [], [{contact_person_label}:], [*{contact_person}*],
  [{vat_number_label}:], [*{vat_number}*], [], [], [],
)
#set text(size: 9pt)

#v(6mm)

// Line items table
#table(
  columns: (28pt, 1fr, 55pt, 60pt, 40pt, 65pt),
  stroke: none,
  inset: (x: 4pt, y: 5pt),
  table.header(
    table.hline(stroke: 0.6pt),
    [*{position_header}*], [*{description_header}*], [#align(right)[*{quantity_header}*]], [#align(right)[*{unit_price_header}*]], [#align(right)[*{vat_header}*]], [#align(right)[*{total_header} {currency}*]],
    table.hline(stroke: 0.4pt),
  ),
{lines_markup}  table.hline(stroke: 0.4pt),
)

#v(4mm)

// Totals
#align(right)[
  #grid(
    columns: (auto, 80pt),
    column-gutter: 10pt,
    row-gutter: 3pt,
    align: (right, right),
    [*{total_caption}*], [*{subtotal}*],
    [{plus_vat_caption}], [{vat_amount}],
  )
  #v(2mm)
  #line(length: 130pt, stroke: 0.6pt)
  #v(2mm)
  #text(size: 11pt, weight: "bold")[{total_header}: {currency} {total}]
]

#v(1fr)

// Footer
#line(length: 100%, stroke: 0.3pt + gray)
#v(2mm)
#text(size: 7pt, fill: gray)[
  *{legal_name}* {company_addr} #h(4pt)
  *{email_label}:* {email} #h(4pt)
  *{ch_vat_label}:* {vat_number}
]"##,
        logo_block = logo_block,
        contact_name2_line = if contact_name2_line.is_empty() {
            format!("{}\\\n", esc(&c.name1))
        } else {
            format!("{}\\\n{}", esc(&c.name1), contact_name2_line)
        },
        contact_addr = if contact_addr.is_empty() { String::new() } else { format!("{}\\\n", esc(contact_addr)) },
        contact_postal = esc(contact_postal),
        contact_city = esc(contact_city_val),
        contact_country = esc(contact_country),
        credit_note_title = labels.credit_note_title,
        cn_number = esc(cn_number),
        date_label = labels.date,
        contact_person_label = labels.contact_person,
        vat_number_label = labels.vat_number,
        issue_date = cn.issue_date,
        contact_person = esc(&c.name1),
        position_header = labels.position_header,
        description_header = labels.description_header,
        quantity_header = labels.quantity_header,
        unit_price_header = labels.unit_price_header,
        vat_header = labels.vat_header,
        total_header = labels.total_header,
        total_caption = labels.total_caption,
        plus_vat_caption = labels.plus_vat,
        vat_number = vat_number,
        lines_markup = lines_markup,
        currency = &data.currency,
        subtotal = fmt_money(cn.subtotal),
        vat_amount = fmt_money(cn.vat_amount),
        total = fmt_money(cn.total),
        legal_name = esc(&s.legal_name),
        company_addr = esc(&format!("{}, {} {}", s.street, s.postal_code, s.city)),
        email_label = labels.email,
        ch_vat_label = labels.ch_vat_number,
        email = email,
    )
}

#[derive(Clone, Copy)]
struct CreditNoteLabels {
    credit_note_title: &'static str,
    date: &'static str,
    contact_person: &'static str,
    vat_number: &'static str,
    position_header: &'static str,
    description_header: &'static str,
    quantity_header: &'static str,
    unit_price_header: &'static str,
    vat_header: &'static str,
    total_header: &'static str,
    total_caption: &'static str,
    plus_vat: &'static str,
    email: &'static str,
    ch_vat_number: &'static str,
}

fn credit_note_labels(language: &str) -> CreditNoteLabels {
    match language {
        "de" => CreditNoteLabels {
            credit_note_title: "Gutschrift",
            date: "Datum",
            contact_person: "Kontaktperson",
            vat_number: "MWST-Nummer",
            position_header: "Pos.",
            description_header: "Beschreibung",
            quantity_header: "Menge",
            unit_price_header: "Einzelpreis",
            vat_header: "MWST",
            total_header: "Total",
            total_caption: "Total",
            plus_vat: "zzgl. MWST",
            email: "E-Mail",
            ch_vat_number: "CH MWST-Nr.",
        },
        "fr" => CreditNoteLabels {
            credit_note_title: "Note de credit",
            date: "Date",
            contact_person: "Personne de contact",
            vat_number: "Numero TVA",
            position_header: "Pos.",
            description_header: "Description",
            quantity_header: "Quantite",
            unit_price_header: "Prix unitaire",
            vat_header: "TVA",
            total_header: "Total",
            total_caption: "Total",
            plus_vat: "plus TVA",
            email: "E-mail",
            ch_vat_number: "No TVA CH",
        },
        "it" => CreditNoteLabels {
            credit_note_title: "Nota di credito",
            date: "Data",
            contact_person: "Persona di contatto",
            vat_number: "Numero IVA",
            position_header: "Pos.",
            description_header: "Descrizione",
            quantity_header: "Quantita",
            unit_price_header: "Prezzo unitario",
            vat_header: "IVA",
            total_header: "Totale",
            total_caption: "Totale",
            plus_vat: "piu IVA",
            email: "E-mail",
            ch_vat_number: "No IVA CH",
        },
        _ => CreditNoteLabels {
            credit_note_title: "Credit Note",
            date: "Date",
            contact_person: "Contact Person",
            vat_number: "VAT Number",
            position_header: "Pos.",
            description_header: "Description",
            quantity_header: "Quantity",
            unit_price_header: "Unit Price",
            vat_header: "VAT",
            total_header: "Total",
            total_caption: "Total",
            plus_vat: "plus VAT",
            email: "E-mail",
            ch_vat_number: "CH VAT No.",
        },
    }
}

fn load_logo(settings: &company_setting::Model) -> Option<Vec<u8>> {
    settings
        .logo_url
        .as_ref()
        .and_then(|url| std::fs::read(url).ok())
        .or_else(|| std::fs::read("uploads/logo.png").ok())
}

fn fmt_money(d: Decimal) -> String {
    let s = format!("{:.2}", d);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let dec_part = parts.get(1).unwrap_or(&"00");
    let formatted: String = int_part.chars().rev().collect::<Vec<_>>()
        .chunks(3).map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>().join("'")
        .chars().rev().collect();
    format!("{formatted}.{dec_part}")
}

fn fmt_dec(d: Decimal) -> String { format!("{:.2}", d) }

fn fmt_qty(d: Decimal) -> String {
    let s = format!("{}", d);
    if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() }
    else { s }
}

fn fmt_pct(vat_amount: Decimal, subtotal: Decimal) -> String {
    if subtotal.is_zero() { return "0%".into(); }
    let pct = (vat_amount / subtotal) * Decimal::from(100);
    format!("{:.2}%", pct)
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('@', "\\@")
        .replace('<', "\\<")
        .replace('>', "\\>")
        .replace('$', "\\$")
}
