use konto_common::error::AppError;
use konto_db::entities::{company_setting, contact, document};
use konto_db::repository::contact_repo::ContactRepo;
use konto_db::repository::document_repo::DocumentRepo;
use konto_db::repository::settings_repo::SettingsRepo;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use super::language::normalize_or_default;

pub struct PdfDocumentService;

impl PdfDocumentService {
    pub async fn generate(db: &DatabaseConnection, id: &str) -> Result<Vec<u8>, AppError> {
        let data = fetch_data(db, id).await?;
        render_typst_pdf(&data)
    }
}

struct PdfData {
    document: document::Model,
    contact: contact::Model,
    settings: company_setting::Model,
    doc_model: DocumentContent,
}

async fn fetch_data(db: &DatabaseConnection, id: &str) -> Result<PdfData, AppError> {
    let doc = DocumentRepo::find_by_id(db, id)
        .await.map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Document not found".into()))?;
    let contact = ContactRepo::find_by_id(db, &doc.contact_id)
        .await.map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Contact not found".into()))?;
    let settings = SettingsRepo::find(db)
        .await.map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Company settings not configured".into()))?;
    let doc_model: DocumentContent = serde_json::from_str(&doc.content_json)
        .unwrap_or_else(|_| DocumentContent { blocks: vec![], header: vec![], footer: vec![] });
    Ok(PdfData { document: doc, contact, settings, doc_model })
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
    let has_logo = load_logo(s).is_some();
    let lang = normalize_or_default(
        data.document
            .language
            .as_deref()
            .or(data.contact.language.as_deref()),
        &s.ui_language,
    );
    let labels = doc_labels(&lang);

    let logo_block = if has_logo {
        r#"#image("logo.png", width: 40mm)"#.to_string()
    } else {
        let trade = s.trade_name.as_deref().unwrap_or(&s.legal_name);
        format!(r#"#text(size: 18pt, weight: "bold")[{}]"#, esc(trade))
    };

    let header_markup = render_header(data, &logo_block, &labels);
    let body_markup = render_blocks(&data.doc_model.blocks, &labels);
    let footer_markup = render_footer(data, &labels);

    format!(
        r##"#set page(paper: "a4", margin: (top: 25mm, bottom: 25mm, left: 20mm, right: 20mm))
#set text(size: 9pt, font: "Noto Sans")

{header_markup}
#v(6mm)
{body_markup}
#v(1fr)
{footer_markup}"##
    )
}

fn render_header(data: &PdfData, logo_block: &str, labels: &DocLabels) -> String {
    let c = &data.contact;
    let doc = &data.document;
    let doc_number = doc.doc_number.as_deref().unwrap_or("DRAFT");
    let doc_type_label = doc_type_title(doc.doc_type.as_str(), labels);

    let contact_name = &c.name1;
    let name2_line = match c.name2.as_deref() {
        Some(n) if !n.is_empty() => format!("{}\\\n", esc(n)),
        _ => String::new(),
    };
    let addr = c.address.as_deref().unwrap_or("");
    let postal = c.postal_code.as_deref().unwrap_or("");
    let city = c.city.as_deref().unwrap_or("");
    let country = c.country.as_deref().unwrap_or("");
    let issue_date = doc.issued_at.map(|d| d.to_string()).unwrap_or_default();
    let valid_until = doc.valid_until.map(|d| d.to_string()).unwrap_or_default();

    format!(
        r##"// Header
#grid(columns: (1fr, auto), [], align(right)[{logo_block}])
#v(12mm)
// Recipient
{name1}\
{name2}{addr_line}{postal} {city}\
{country}
#v(10mm)
#text(size: 14pt, weight: "bold")[{doc_type} {doc_nr}]
#v(3mm)
#set text(size: 8pt)
#grid(
  columns: (55pt, auto, 1fr, 55pt, auto),
  column-gutter: 5pt, row-gutter: 3pt,
  [{date_label}:], [*{issue_date}*], [], [{valid_until_label}:], [*{valid_until}*],
)
#set text(size: 9pt)
"##,
        logo_block = logo_block,
        name1 = esc(contact_name),
        name2 = name2_line,
        addr_line = if addr.is_empty() { String::new() } else { format!("{}\\\n", esc(addr)) },
        postal = esc(postal),
        city = esc(city),
        country = esc(country),
        doc_type = doc_type_label,
        doc_nr = esc(doc_number),
        date_label = labels.date,
        valid_until_label = labels.valid_until,
        issue_date = esc(&issue_date),
        valid_until = esc(&valid_until),
    )
}

fn render_blocks(blocks: &[ContentBlock], labels: &DocLabels) -> String {
    let mut out = String::new();
    for block in blocks {
        if block.meta.as_ref().is_some_and(|m| m.page_break_before.unwrap_or(false)) {
            out.push_str("#pagebreak()\n");
        }
        let align_prefix = block.meta.as_ref()
            .and_then(|m| m.align.as_deref())
            .and_then(|a| match a {
                "center" => Some("#align(center)["),
                "right" => Some("#align(right)["),
                _ => None,
            });
        if let Some(prefix) = align_prefix {
            out.push_str(prefix);
            out.push('\n');
        }
        out.push_str(&render_block(block, labels));
        out.push('\n');
        if align_prefix.is_some() {
            out.push_str("]\n");
        }
    }
    out
}

fn render_block(block: &ContentBlock, labels: &DocLabels) -> String {
    match block.block_type.as_str() {
        "h1" => render_heading(block, 1),
        "h2" => render_heading(block, 2),
        "h3" => render_heading(block, 3),
        "p" => render_paragraph(block),
        "blockquote" => render_blockquote(block),
        "table" => render_table(block),
        "divider" => "#line(length: 100%, stroke: 0.4pt + gray)\n".into(),
        "spacer" => render_spacer(block),
        "signature" => render_signature(block),
        "contact_info" => render_contact_info(block),
        "doc_meta" => render_doc_meta(block, labels),
        "placeholder" => render_placeholder(block),
        "image" => render_image(block),
        _ => String::new(),
    }
}

fn render_heading(block: &ContentBlock, level: u8) -> String {
    let text = extract_text(&block.data);
    let size = match level {
        1 => "14pt",
        2 => "12pt",
        _ => "10pt",
    };
    format!(r#"#text(size: {size}, weight: "bold")[{text}]"#, text = esc(&text))
}

fn render_paragraph(block: &ContentBlock) -> String {
    let text = extract_text(&block.data);
    if text.is_empty() { return "#v(4mm)\n".into(); }
    esc(&text) + "\n"
}

fn render_blockquote(block: &ContentBlock) -> String {
    let quote = block.data.get("quote").and_then(|v| v.as_str()).unwrap_or("");
    let author = block.data.get("author").and_then(|v| v.as_str()).unwrap_or("");
    let mut out = format!(
        "#block(inset: (left: 10pt), stroke: (left: 2pt + gray))[\n  {}\n]\n",
        esc(quote)
    );
    if !author.is_empty() {
        out.push_str(&format!("#text(size: 8pt, style: \"italic\")[— {}]\n", esc(author)));
    }
    out
}

fn render_table(block: &ContentBlock) -> String {
    let headers: Vec<String> = block.data.get("headers")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();
    let rows: Vec<Vec<String>> = block.data.get("rows")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().map(|row| {
            row.as_array()
                .map(|cells| cells.iter().filter_map(|c| c.as_str().map(String::from)).collect())
                .unwrap_or_default()
        }).collect())
        .unwrap_or_default();
    if headers.is_empty() { return String::new(); }

    let ncols = headers.len();
    let cols = vec!["1fr"; ncols].join(", ");
    let mut out = format!(
        "#table(\n  columns: ({cols}),\n  stroke: 0.4pt + gray,\n  inset: (x: 4pt, y: 5pt),\n"
    );
    // Header row
    for h in &headers {
        out.push_str(&format!("  [*{}*],\n", esc(h)));
    }
    // Data rows
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            if i < ncols {
                out.push_str(&format!("  [{}],\n", esc(cell)));
            }
        }
    }
    out.push_str(")\n");
    out
}

fn render_spacer(block: &ContentBlock) -> String {
    let height = block.data.get("height").and_then(|v| v.as_f64()).unwrap_or(10.0);
    format!("#v({}mm)\n", (height * 0.26).round().max(2.0)) // px to mm approx
}

fn render_signature(block: &ContentBlock) -> String {
    let parties: Vec<SignatureParty> = block.data.get("parties")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    if parties.is_empty() { return String::new(); }

    let ncols = parties.len();
    let cols = vec!["1fr"; ncols].join(", ");
    let mut out = format!("#v(8mm)\n#grid(columns: ({cols}), column-gutter: 10pt,\n");
    for party in &parties {
        out.push_str("  [\n");
        if !party.role.is_empty() {
            out.push_str(&format!("    #text(size: 8pt, weight: \"bold\")[{}]\\\n", esc(&party.role)));
        }
        if !party.company.is_empty() {
            out.push_str(&format!("    #text(size: 8pt)[{}]\\\n", esc(&party.company)));
        }
        out.push_str("    #v(4mm)\n");
        if !party.location.is_empty() || !party.date.is_empty() {
            out.push_str(&format!("    #text(size: 8pt)[{}, {}]\\\n",
                esc(&party.location), esc(&party.date)));
        }
        out.push_str("    #v(12mm)\n    #line(length: 80%, stroke: 0.3pt)\n");
        for line in &party.lines {
            out.push_str(&format!("    #text(size: 7pt)[{}: {}]\\\n",
                esc(&line.label), esc(&line.value)));
        }
        out.push_str("  ],\n");
    }
    out.push_str(")\n");
    out
}

fn render_contact_info(block: &ContentBlock) -> String {
    let lines: Vec<String> = block.data.get("lines")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();
    if lines.is_empty() { return String::new(); }
    let mut out = String::new();
    for line in &lines {
        out.push_str(&format!("{}\\\n", esc(line)));
    }
    out
}

fn render_doc_meta(block: &ContentBlock, labels: &DocLabels) -> String {
    let doc_date = block.data.get("docDate").and_then(|v| v.as_str()).unwrap_or("");
    let valid = block.data.get("validUntil").and_then(|v| v.as_str()).unwrap_or("");
    let project = block.data.get("projectName").and_then(|v| v.as_str()).unwrap_or("");
    let number = block.data.get("docNumber").and_then(|v| v.as_str()).unwrap_or("");
    let mut out = String::from("#set text(size: 8pt)\n#grid(columns: (60pt, auto), column-gutter: 5pt, row-gutter: 3pt,\n");
    if !number.is_empty() {
        out.push_str(&format!("  [{}:], [*{}*],\n", labels.number, esc(number)));
    }
    if !doc_date.is_empty() {
        out.push_str(&format!("  [{}:], [*{}*],\n", labels.date, esc(doc_date)));
    }
    if !valid.is_empty() {
        out.push_str(&format!("  [{}:], [*{}*],\n", labels.valid_until, esc(valid)));
    }
    if !project.is_empty() {
        out.push_str(&format!("  [{}:], [*{}*],\n", labels.project, esc(project)));
    }
    out.push_str(")\n#set text(size: 9pt)\n");
    out
}

fn render_placeholder(block: &ContentBlock) -> String {
    let resolved = block.data.get("resolved").and_then(|v| v.as_str()).unwrap_or("");
    let variable = block.data.get("variable").and_then(|v| v.as_str()).unwrap_or("");
    let text = if resolved.is_empty() { variable } else { resolved };
    if text.is_empty() { return String::new(); }
    esc(text) + "\n"
}

fn render_image(_block: &ContentBlock) -> String {
    // Images from the editor are base64/URLs — skip in server-side PDF for now
    "#rect(width: 100%, height: 30mm, stroke: 0.3pt + gray, fill: luma(245))[\n  #align(center + horizon)[#text(size: 8pt, fill: gray)[Image]]\n]\n".into()
}

fn render_footer(data: &PdfData, labels: &DocLabels) -> String {
    let s = &data.settings;
    let email = esc(s.email.as_deref().unwrap_or(""));
    let vat_number = esc(s.vat_number.as_deref().unwrap_or(""));
    let website = esc(s.website.as_deref().unwrap_or(""));
    format!(
        r##"#line(length: 100%, stroke: 0.3pt + gray)
#v(2mm)
#text(size: 7pt, fill: gray)[
  *{legal_name}* {addr} #h(4pt)
  *{email_label}:* {email} #h(4pt)
  *{vat_label}:* {vat_number} #h(4pt)
  *{website_label}:* {website}
]"##,
        legal_name = esc(&s.legal_name),
        addr = esc(&format!("{}, {} {}", s.street, s.postal_code, s.city)),
        email_label = labels.email,
        vat_label = labels.ch_vat_number,
        website_label = labels.website,
        email = email,
        vat_number = vat_number,
        website = website,
    )
}

#[derive(Clone, Copy)]
struct DocLabels {
    date: &'static str,
    valid_until: &'static str,
    number: &'static str,
    project: &'static str,
    email: &'static str,
    ch_vat_number: &'static str,
    website: &'static str,
    quote: &'static str,
    offer: &'static str,
    sow: &'static str,
    contract: &'static str,
    document: &'static str,
}

fn doc_labels(language: &str) -> DocLabels {
    match language {
        "de" => DocLabels {
            date: "Datum",
            valid_until: "Gueltig bis",
            number: "Nummer",
            project: "Projekt",
            email: "E-Mail",
            ch_vat_number: "CH MWST-Nr.",
            website: "Website",
            quote: "Offerte",
            offer: "Angebot",
            sow: "Leistungsvereinbarung",
            contract: "Vertrag",
            document: "Dokument",
        },
        "fr" => DocLabels {
            date: "Date",
            valid_until: "Valable jusqu'au",
            number: "Numero",
            project: "Projet",
            email: "E-mail",
            ch_vat_number: "No TVA CH",
            website: "Site web",
            quote: "Devis",
            offer: "Offre",
            sow: "Cahier des charges",
            contract: "Contrat",
            document: "Document",
        },
        "it" => DocLabels {
            date: "Data",
            valid_until: "Valido fino al",
            number: "Numero",
            project: "Progetto",
            email: "E-mail",
            ch_vat_number: "No IVA CH",
            website: "Sito web",
            quote: "Preventivo",
            offer: "Offerta",
            sow: "Dichiarazione di lavoro",
            contract: "Contratto",
            document: "Documento",
        },
        _ => DocLabels {
            date: "Date",
            valid_until: "Valid Until",
            number: "Number",
            project: "Project",
            email: "E-mail",
            ch_vat_number: "CH VAT No.",
            website: "Website",
            quote: "Quote",
            offer: "Offer",
            sow: "Statement of Work",
            contract: "Contract",
            document: "Document",
        },
    }
}

fn doc_type_title<'a>(doc_type: &str, labels: &'a DocLabels) -> &'a str {
    match doc_type {
        "quote" => labels.quote,
        "offer" => labels.offer,
        "sow" => labels.sow,
        "contract" => labels.contract,
        _ => labels.document,
    }
}

fn extract_text(data: &serde_json::Value) -> String {
    data.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string()
}

fn load_logo(settings: &company_setting::Model) -> Option<Vec<u8>> {
    settings
        .logo_url
        .as_ref()
        .and_then(|url| std::fs::read(url).ok())
        .or_else(|| std::fs::read("uploads/logo.png").ok())
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

// --- Deserialization types for content_json ---

#[derive(Deserialize)]
struct DocumentContent {
    blocks: Vec<ContentBlock>,
    #[serde(default)]
    #[allow(dead_code)]
    header: Vec<ContentBlock>,
    #[serde(default)]
    #[allow(dead_code)]
    footer: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    #[serde(default = "default_data")]
    data: serde_json::Value,
    meta: Option<BlockMeta>,
}

fn default_data() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

#[derive(Deserialize)]
struct BlockMeta {
    align: Option<String>,
    #[serde(rename = "pageBreakBefore")]
    page_break_before: Option<bool>,
}

#[derive(Deserialize, Default)]
struct SignatureParty {
    #[serde(default)]
    role: String,
    #[serde(default)]
    company: String,
    #[serde(default)]
    location: String,
    #[serde(default)]
    date: String,
    #[serde(default)]
    lines: Vec<SignatureLine>,
}

#[derive(Deserialize)]
struct SignatureLine {
    #[serde(default)]
    label: String,
    #[serde(default)]
    value: String,
}
