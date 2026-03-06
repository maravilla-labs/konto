use konto_common::error::AppError;
use konto_common::markdown::md_to_typst;
use konto_db::entities::{bank_account, company_setting, contact, invoice, invoice_line};
use konto_db::repository::bank_account_repo::BankAccountRepo;
use konto_db::repository::contact_repo::ContactRepo;
use konto_db::repository::invoice_repo::InvoiceRepo;
use konto_db::repository::settings_repo::SettingsRepo;
use rust_decimal::Decimal;
use sea_orm::DatabaseConnection;

use super::language::normalize_or_default;
use super::qr_bill;

/// Official SIX scissors perforation symbol PNG per IG QR-bill v2.3 §3.7
const SCISSORS_PNG: &[u8] = include_bytes!("../../assets/Scissors_symbol.png");

/// Crop the bottom portion of the scissors PNG (where the actual symbol is).
/// The source image is 4961×1109 with content at rows ~880-1010.
fn crop_scissors_png() -> Result<Vec<u8>, AppError> {
    use image::codecs::png::PngEncoder;
    use image::ImageEncoder;
    let img = image::load_from_memory(SCISSORS_PNG)
        .map_err(|e| AppError::Internal(format!("Scissors PNG load failed: {e}")))?;
    let cropped = img.crop_imm(0, 860, img.width(), 160);
    let mut buf = Vec::new();
    let encoder = PngEncoder::new(&mut buf);
    encoder.write_image(
        cropped.to_rgba8().as_raw(),
        cropped.width(),
        cropped.height(),
        image::ExtendedColorType::Rgba8,
    ).map_err(|e| AppError::Internal(format!("Scissors crop failed: {e}")))?;
    Ok(buf)
}


pub struct PdfInvoiceService;

impl PdfInvoiceService {
    pub async fn generate(db: &DatabaseConnection, invoice_id: &str) -> Result<Vec<u8>, AppError> {
        let data = fetch_data(db, invoice_id).await?;
        render_typst_pdf(&data)
    }
}

struct PdfData {
    invoice: invoice::Model,
    lines: Vec<invoice_line::Model>,
    contact: contact::Model,
    settings: company_setting::Model,
    bank: Option<bank_account::Model>,
    currency: String,
    contact_person_name: Option<String>,
}

async fn fetch_data(db: &DatabaseConnection, id: &str) -> Result<PdfData, AppError> {
    let invoice = InvoiceRepo::find_by_id(db, id)
        .await.map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Invoice not found".into()))?;
    let lines = InvoiceRepo::find_lines_by_invoice(db, id)
        .await.map_err(|e| AppError::Database(e.to_string()))?;
    let contact = ContactRepo::find_by_id(db, &invoice.contact_id)
        .await.map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Contact not found".into()))?;
    let settings = SettingsRepo::find(db)
        .await.map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Company settings not configured".into()))?;
    // Use invoice-specific bank account if set, otherwise fall back to default
    let bank = if let Some(ref bank_id) = invoice.bank_account_id {
        BankAccountRepo::find_by_id(db, bank_id)
            .await.map_err(|e| AppError::Database(e.to_string()))?
    } else {
        BankAccountRepo::find_default(db)
            .await.map_err(|e| AppError::Database(e.to_string()))?
    };

    // Resolve contact person name (from contacts table, since migration 000081 moved persons to contacts)
    let contact_person_name = if let Some(ref cp_id) = invoice.contact_person_id {
        ContactRepo::find_by_id(db, cp_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .map(|cp| cp.name1)
    } else {
        None
    };

    Ok(PdfData { invoice, lines, contact, settings, bank, currency: "CHF".into(), contact_person_name })
}

fn render_typst_pdf(data: &PdfData) -> Result<Vec<u8>, AppError> {
    use typst::layout::PagedDocument;
    use typst_as_lib::TypstEngine;
    use typst_as_lib::typst_kit_options::TypstKitFontOptions;

    let typst_source = build_typst_source(data)?;

    // Collect static binary files (logo, QR code)
    let mut binaries: Vec<(&str, Vec<u8>)> = Vec::new();
    if let Some(logo) = load_logo(&data.settings) {
        binaries.push(("logo.png", logo));
    }
    if let Some(qr) = generate_qr_image(data) {
        binaries.push(("qrcode.png", qr));
    }
    // Cropped scissors symbol for QR-bill perforation per §3.7
    if let Ok(scissors) = crop_scissors_png() {
        binaries.push(("scissors.png", scissors));
    }

    let bin_refs: Vec<(&str, &[u8])> = binaries.iter()
        .map(|(name, data)| (*name, data.as_slice()))
        .collect();

    let font_opts = TypstKitFontOptions::new()
        .include_system_fonts(true)
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

fn build_typst_source(data: &PdfData) -> Result<String, AppError> {
    let s = &data.settings;
    let c = &data.contact;
    let inv = &data.invoice;
    let inv_number = inv.invoice_number.as_deref().unwrap_or("DRAFT");
    let lang = normalize_or_default(
        inv.language.as_deref().or(c.language.as_deref()),
        &s.ui_language,
    );
    let labels = invoice_labels(&lang);

    // Contact address lines
    let contact_name2_line = match c.name2.as_deref() {
        Some(n) if !n.is_empty() => format!("{}\\\n", esc(n)),
        _ => String::new(),
    };
    let contact_addr = c.address.as_deref().unwrap_or("");
    let contact_postal = c.postal_code.as_deref().unwrap_or("");
    let contact_city_val = c.city.as_deref().unwrap_or("");
    let contact_country = c.country.as_deref().unwrap_or("");

    // Check if any line has a discount
    let has_discount = data.lines.iter().any(|l| l.discount_percent.map(|d| !d.is_zero()).unwrap_or(false));

    // Build line items markup
    let mut lines_markup = String::new();
    for line in &data.lines {
        if has_discount {
            let disc_str = line.discount_percent
                .filter(|d| !d.is_zero())
                .map(|d| format!("{}%", d))
                .unwrap_or_default();
            lines_markup.push_str(&format!(
                "  [{}], [{}], [#align(right)[{}]], [#align(right)[{}]], [#align(right)[{}]], [#align(right)[{}]], [#align(right)[{}]],\n",
                line.position,
                md_to_typst(&line.description),
                fmt_qty(line.quantity),
                fmt_dec(line.unit_price),
                esc(&disc_str),
                fmt_pct(line.vat_amount, inv.subtotal),
                fmt_money(line.line_total),
            ));
        } else {
            lines_markup.push_str(&format!(
                "  [{}], [{}], [#align(right)[{}]], [#align(right)[{}]], [#align(right)[{}]], [#align(right)[{}]],\n",
                line.position,
                md_to_typst(&line.description),
                fmt_qty(line.quantity),
                fmt_dec(line.unit_price),
                fmt_pct(line.vat_amount, inv.subtotal),
                fmt_money(line.line_total),
            ));
        }
    }

    // Bank info
    let (bank_name, iban_fmt) = match &data.bank {
        Some(b) => (esc(&b.bank_name), format_iban(&b.iban)),
        None => (String::new(), String::new()),
    };
    let vat_number = esc(s.vat_number.as_deref().unwrap_or(""));
    let website = esc(s.website.as_deref().unwrap_or(""));
    let email = esc(s.email.as_deref().unwrap_or(""));
    let has_logo = load_logo(s).is_some();
    let customer_nr = c.customer_number.clone()
        .or_else(|| c.bexio_id.map(|id| format!("{:06}", id)))
        .unwrap_or_else(|| "-".into());

    // Logo or text fallback
    let logo_block = if has_logo {
        r#"#image("logo.png", width: 40mm)"#.to_string()
    } else {
        let trade = s.trade_name.as_deref().unwrap_or(&s.legal_name);
        format!(r#"#text(size: 18pt, weight: "bold")[{}]"#, esc(trade))
    };

    // QR-bill page (page 2)
    let qr_section = if data.bank.is_some() {
        build_qr_bill_section(data, &labels)
    } else {
        String::new()
    };

    // Contact person name — prefer resolved name, fallback to contact name
    let contact_person_display = data.contact_person_name.as_deref()
        .unwrap_or(&c.name1);

    // Header/footer text sections
    let header_text_section = match inv.header_text.as_deref() {
        Some(txt) if !txt.is_empty() => format!("{}\n\n#v(4mm)\n", md_to_typst(txt)),
        _ => String::new(),
    };
    let footer_text_section = match inv.footer_text.as_deref() {
        Some(txt) if !txt.is_empty() => format!("\n#v(4mm)\n{}\n", md_to_typst(txt)),
        _ => String::new(),
    };

    // Reverse charge / export exempt VAT notice
    let resolved_vat_mode = super::vat_resolution_service::VatResolutionService::resolve_vat_mode(c);
    let vat_notice_section = match resolved_vat_mode.as_str() {
        "reverse_charge" => {
            let notice = match lang.as_str() {
                "de" => "Reverse Charge — Die Steuerschuld geht auf den Leistungsempfänger über.",
                "fr" => "Autoliquidation — La TVA est due par le destinataire.",
                "it" => "Inversione contabile — L'obbligo IVA è trasferito al destinatario.",
                _ => "Reverse charge — VAT liability transferred to recipient.",
            };
            format!("\n#v(3mm)\n#text(size: 8pt, style: \"italic\")[{}]\n", esc(notice))
        }
        "export_exempt" => {
            let notice = match lang.as_str() {
                "de" => "Steuerbefreite Ausfuhrlieferung — Keine MWST erhoben.",
                "fr" => "Exportation exonérée — Pas de TVA appliquée.",
                "it" => "Esportazione esente — Nessuna IVA applicata.",
                _ => "Export exempt — No VAT applied.",
            };
            format!("\n#v(3mm)\n#text(size: 8pt, style: \"italic\")[{}]\n", esc(notice))
        }
        _ => String::new(),
    };

    // Dynamic table columns and headers based on discount presence
    let (table_columns, table_header) = if has_discount {
        (
            "(28pt, 1fr, 55pt, 60pt, 40pt, 40pt, 65pt)".to_string(),
            format!(
                "[*{pos}*], [*{desc}*], [#align(right)[*{qty}*]], [#align(right)[*{price}*]], [#align(right)[*{discount_h}*]], [#align(right)[*{vat}*]], [#align(right)[*{total} {cur}*]]",
                pos = labels.position_header,
                desc = labels.description_header,
                qty = labels.quantity_header,
                price = labels.unit_price_header,
                discount_h = labels.discount_header,
                vat = labels.vat_header,
                total = labels.total_header,
                cur = &data.currency,
            ),
        )
    } else {
        (
            "(28pt, 1fr, 55pt, 60pt, 40pt, 65pt)".to_string(),
            format!(
                "[*{pos}*], [*{desc}*], [#align(right)[*{qty}*]], [#align(right)[*{price}*]], [#align(right)[*{vat}*]], [#align(right)[*{total} {cur}*]]",
                pos = labels.position_header,
                desc = labels.description_header,
                qty = labels.quantity_header,
                price = labels.unit_price_header,
                vat = labels.vat_header,
                total = labels.total_header,
                cur = &data.currency,
            ),
        )
    };

    Ok(format!(
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

// Invoice title
#text(size: 14pt, weight: "bold")[{invoice_title} {inv_number}]

#v(3mm)

// Metadata grid
#set text(size: 8pt)
#grid(
  columns: (55pt, auto, 1fr, 65pt, auto),
  column-gutter: 5pt,
  row-gutter: 3pt,
  [{date_label}:], [*{issue_date}*], [], [{contact_person_label}:], [*{contact_person}*],
  [{valid_to_label}:], [*{due_date}*], [], [{customer_number_label}:], [*{customer_nr}*],
  [{vat_number_label}:], [*{vat_number}*], [], [], [],
)
#set text(size: 9pt)

#v(6mm)

{header_text_section}// Line items table
#table(
  columns: {table_columns},
  stroke: none,
  inset: (x: 4pt, y: 5pt),
  table.header(
    table.hline(stroke: 0.6pt),
    {table_header},
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
  #text(size: 11pt, weight: "bold")[{grand_total_caption}: {currency} {total}]
]
{vat_notice_section}{footer_text_section}
#v(1fr)

// Footer
#line(length: 100%, stroke: 0.3pt + gray)
#v(2mm)
#text(size: 7pt, fill: gray)[
  *{legal_name}* {company_addr} #h(4pt)
  *{email_label}:* {email} #h(4pt)
  *{bank_label}:* {bank_name}\
  *{iban_label}:* {iban} #h(4pt)
  *{ch_vat_label}:* {vat_number} #h(4pt)
  *{website_label}:* {website}
]
{qr_section}"##,
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
        invoice_title = labels.invoice_title,
        inv_number = esc(inv_number),
        date_label = labels.date,
        contact_person_label = labels.contact_person,
        valid_to_label = labels.valid_to,
        customer_number_label = labels.customer_number,
        vat_number_label = labels.vat_number,
        issue_date = inv.issue_date,
        due_date = inv.due_date,
        contact_person = esc(contact_person_display),
        customer_nr = esc(&customer_nr),
        table_columns = table_columns,
        table_header = table_header,
        header_text_section = header_text_section,
        footer_text_section = footer_text_section,
        vat_notice_section = vat_notice_section,
        vat_number = vat_number,
        lines_markup = lines_markup,
        currency = &data.currency,
        total_caption = labels.total_caption,
        plus_vat_caption = labels.plus_vat,
        grand_total_caption = labels.total_header,
        subtotal = fmt_money(inv.subtotal),
        vat_amount = fmt_money(inv.vat_amount),
        total = fmt_money(inv.total),
        legal_name = esc(&s.legal_name),
        company_addr = esc(&format!("{}, {} {}", s.street, s.postal_code, s.city)),
        email_label = labels.email,
        bank_label = labels.bank,
        iban_label = labels.iban,
        ch_vat_label = labels.ch_vat_number,
        website_label = labels.website,
        email = email,
        bank_name = bank_name,
        iban = esc(&iban_fmt),
        website = website,
        qr_section = qr_section,
    ))
}

#[allow(clippy::unwrap_used)]
fn build_qr_bill_section(data: &PdfData, labels: &InvoiceLabels) -> String {
    let bank = data.bank.as_ref().unwrap();
    let inv = &data.invoice;
    let s = &data.settings;
    let c = &data.contact;
    let inv_number = inv.invoice_number.as_deref().unwrap_or("DRAFT");
    let is_draft = inv.invoice_number.is_none();
    let spc_iban = effective_spc_iban(bank);
    let ref_info = qr_bill::build_reference(&spc_iban, inv_number, is_draft);
    // Display the QR-IBAN if present, otherwise regular IBAN
    let iban = format_iban(&spc_iban);
    let total = fmt_money_qr(inv.total);
    let debtor_street = c.address.as_deref().unwrap_or("");
    let debtor_postal_city = format!(
        "{} {}",
        c.postal_code.as_deref().unwrap_or(""),
        c.city.as_deref().unwrap_or("")
    );
    let add_info = inv_number;

    // SIX IG QR-bill v2.3 §3: Receipt 62×105mm + Payment Part 148×105mm = 210×105mm
    // §3.4: Only Helvetica, Arial, Frutiger, Liberation Sans
    // §3.7: PDF format — lines (not perforation) with scissors on BOTH horizontal AND vertical
    // #place(bottom) ensures the 105mm box is flush with page bottom (no spacing overflow)
    format!(r##"
// === QR-bill page — SIX IG QR-bill v2.3 ===
#set page(paper: "a4", margin: 0mm)
#set text(font: ("Helvetica", "Arial", "Frutiger", "Liberation Sans"))
// Place QR-bill at exact bottom of page to prevent spacing overflow
#place(bottom + left)[
  #set block(spacing: 0pt)
  #set par(spacing: 0pt)
  // Horizontal perforation with official scissors symbol (§3.7)
  #image("scissors.png", width: 100%)
  // QR-bill slip: 210 × 105mm (§3.3)
  #box(width: 210mm, height: 105mm)[
    // Vertical line between Receipt and Payment Part (§3.7)
    // Scissors at top of vertical line, rotated to point downward
    #place(dx: 60.5mm, dy: 0mm)[
      #rotate(-90deg, origin: center + horizon)[#text(size: 5pt, font: "Zapf Dingbats")[✂]]
    ]
    #place(dx: 62mm, dy: 3.5mm)[
      #line(start: (0pt, 0pt), end: (0pt, 101.5mm), stroke: (thickness: 0.5pt, paint: luma(120), dash: "dashed"))
    ]
    #grid(columns: (62mm, 148mm),
      // ═══ EMPFANGSSCHEIN (Receipt) — 62 × 105mm (§3.6) ═══
      // §3.4: Title 11pt bold, headings 6pt bold, values 8pt
      box(width: 62mm, height: 105mm, inset: (x: 5mm, y: 5mm))[
        #text(size: 11pt, weight: "bold")[{receipt_label}]
        #v(5mm)
        // Bereich Angaben — Receipt info area (§3.6.2)
        #text(size: 6pt, weight: "bold")[{account_payable_to_label}]
        #v(1mm)
        #text(size: 8pt)[
          {iban}\
          {creditor_name}\
          {creditor_street}\
          {creditor_postal} {creditor_city}
        ]
        #v(3mm)
        #text(size: 6pt, weight: "bold")[{reference_label}]
        #v(1mm)
        #text(size: 8pt)[{reference}]
        #v(3mm)
        #text(size: 6pt, weight: "bold")[{payable_by_label}]
        #v(1mm)
        #text(size: 8pt)[
          {debtor_name}\
          {debtor_street}\
          {debtor_postal_city}
        ]
        #v(1fr)
        // Bereich Betrag — Receipt amount area (§3.6.3)
        #grid(columns: (auto, 1fr), column-gutter: 3mm,
          [
            #text(size: 6pt, weight: "bold")[{currency_label}]
            #v(1mm)
            #text(size: 8pt)[{currency}]
          ],
          [
            #text(size: 6pt, weight: "bold")[{amount_label}]
            #v(1mm)
            #text(size: 8pt)[{total}]
          ],
        )
        #v(5mm)
        // Bereich Annahmestelle (§3.6.4)
        #align(right)[#text(size: 6pt, weight: "bold")[{acceptance_point_label}]]
      ],
      // ═══ ZAHLTEIL (Payment Part) — 148 × 105mm (§3.5) ═══
      // §3.4: Title 11pt bold, headings 8pt bold, values 10pt
      box(width: 148mm, height: 105mm, inset: (left: 5mm, top: 5mm, right: 5mm, bottom: 5mm))[
        #text(size: 11pt, weight: "bold")[{payment_part_label}]
        #v(5mm)
        #grid(columns: (51mm, 1fr), column-gutter: 5mm,
          // Bereich Swiss QR Code — 46×46mm with 5mm border (§3.5.2, §6.4)
          [#image("qrcode.png", width: 46mm)],
          // Bereich Angaben — Payment info area (§3.5.4)
          [
            #text(size: 8pt, weight: "bold")[{account_payable_to_label}]
            #v(1mm)
            #text(size: 10pt)[
              {iban}\
              {creditor_name}\
              {creditor_street}\
              {creditor_postal} {creditor_city}
            ]
            #v(2mm)
            #text(size: 8pt, weight: "bold")[{reference_label}]
            #v(1mm)
            #text(size: 10pt)[{reference}]
            #v(2mm)
            #text(size: 8pt, weight: "bold")[{additional_info_label}]
            #v(1mm)
            #text(size: 10pt)[{additional_info}]
            #v(2mm)
            #text(size: 8pt, weight: "bold")[{payable_by_label}]
            #v(1mm)
            #text(size: 10pt)[
              {debtor_name}\
              {debtor_street}\
              {debtor_postal_city}
            ]
          ],
        )
        #v(1fr)
        // Bereich Betrag — Payment amount area (§3.5.3)
        #grid(columns: (auto, 1fr), column-gutter: 5mm,
          [
            #text(size: 8pt, weight: "bold")[{currency_label}]
            #v(1mm)
            #text(size: 10pt)[{currency}]
          ],
          [
            #text(size: 8pt, weight: "bold")[{amount_label}]
            #v(1mm)
            #text(size: 10pt)[{total}]
          ],
        )
      ],
    )
  ]
]"##,
        receipt_label = labels.receipt,
        payment_part_label = labels.payment_part,
        account_payable_to_label = labels.account_payable_to,
        reference_label = labels.reference,
        additional_info_label = labels.additional_info,
        payable_by_label = labels.payable_by,
        currency_label = labels.currency,
        amount_label = labels.amount,
        acceptance_point_label = labels.acceptance_point,
        iban = esc(&iban),
        creditor_name = esc(&s.legal_name),
        creditor_street = esc(&s.street),
        creditor_postal = esc(&s.postal_code),
        creditor_city = esc(&s.city),
        reference = esc(&ref_info.display),
        additional_info = esc(add_info),
        debtor_name = esc(&c.name1),
        debtor_street = esc(debtor_street),
        debtor_postal_city = esc(&debtor_postal_city),
        currency = &data.currency,
        total = total,
    )
}

#[derive(Clone, Copy)]
struct InvoiceLabels {
    invoice_title: &'static str,
    date: &'static str,
    contact_person: &'static str,
    valid_to: &'static str,
    customer_number: &'static str,
    vat_number: &'static str,
    position_header: &'static str,
    description_header: &'static str,
    quantity_header: &'static str,
    unit_price_header: &'static str,
    vat_header: &'static str,
    discount_header: &'static str,
    total_header: &'static str,
    total_caption: &'static str,
    plus_vat: &'static str,
    email: &'static str,
    bank: &'static str,
    iban: &'static str,
    ch_vat_number: &'static str,
    website: &'static str,
    // QR-bill labels per SIX IG QR-bill v2.3 Appendix C
    receipt: &'static str,
    payment_part: &'static str,
    account_payable_to: &'static str,
    reference: &'static str,
    additional_info: &'static str,
    payable_by: &'static str,
    currency: &'static str,
    amount: &'static str,
    acceptance_point: &'static str,
}

fn invoice_labels(language: &str) -> InvoiceLabels {
    match language {
        "de" => InvoiceLabels {
            invoice_title: "Rechnung",
            date: "Datum",
            contact_person: "Kontaktperson",
            valid_to: "Gültig bis",
            customer_number: "Kundennummer",
            vat_number: "MWST-Nummer",
            position_header: "Pos.",
            description_header: "Beschreibung",
            quantity_header: "Menge",
            unit_price_header: "Einzelpreis",
            vat_header: "MWST",
            discount_header: "Rabatt",
            total_header: "Total",
            total_caption: "Total",
            plus_vat: "zzgl. MWST",
            email: "E-Mail",
            bank: "Bank",
            iban: "IBAN",
            ch_vat_number: "CH MWST-Nr.",
            website: "Website",
            receipt: "Empfangsschein",
            payment_part: "Zahlteil",
            account_payable_to: "Konto / Zahlbar an",
            reference: "Referenz",
            additional_info: "Zusätzliche Informationen",
            payable_by: "Zahlbar durch",
            currency: "Währung",
            amount: "Betrag",
            acceptance_point: "Annahmestelle",
        },
        "fr" => InvoiceLabels {
            invoice_title: "Facture",
            date: "Date",
            contact_person: "Personne de contact",
            valid_to: "Échéance",
            customer_number: "Numéro client",
            vat_number: "Numéro TVA",
            position_header: "Pos.",
            description_header: "Description",
            quantity_header: "Quantité",
            unit_price_header: "Prix unitaire",
            vat_header: "TVA",
            discount_header: "Remise",
            total_header: "Total",
            total_caption: "Total",
            plus_vat: "plus TVA",
            email: "E-mail",
            bank: "Banque",
            iban: "IBAN",
            ch_vat_number: "No TVA CH",
            website: "Site web",
            receipt: "Récépissé",
            payment_part: "Section paiement",
            account_payable_to: "Compte / Payable à",
            reference: "Référence",
            additional_info: "Informations supplémentaires",
            payable_by: "Payable par",
            currency: "Monnaie",
            amount: "Montant",
            acceptance_point: "Point de dépôt",
        },
        "it" => InvoiceLabels {
            invoice_title: "Fattura",
            date: "Data",
            contact_person: "Persona di contatto",
            valid_to: "Scadenza",
            customer_number: "Numero cliente",
            vat_number: "Numero IVA",
            position_header: "Pos.",
            description_header: "Descrizione",
            quantity_header: "Quantità",
            unit_price_header: "Prezzo unitario",
            vat_header: "IVA",
            discount_header: "Sconto",
            total_header: "Totale",
            total_caption: "Totale",
            plus_vat: "più IVA",
            email: "E-mail",
            bank: "Banca",
            iban: "IBAN",
            ch_vat_number: "No IVA CH",
            website: "Sito web",
            receipt: "Ricevuta",
            payment_part: "Sezione pagamento",
            account_payable_to: "Conto / Pagabile a",
            reference: "Riferimento",
            additional_info: "Informazioni supplementari",
            payable_by: "Pagabile da",
            currency: "Valuta",
            amount: "Importo",
            acceptance_point: "Punto di accettazione",
        },
        _ => InvoiceLabels {
            invoice_title: "Invoice",
            date: "Date",
            contact_person: "Contact Person",
            valid_to: "Due Date",
            customer_number: "Customer Number",
            vat_number: "VAT Number",
            position_header: "Pos.",
            description_header: "Description",
            quantity_header: "Quantity",
            unit_price_header: "Unit Price",
            vat_header: "VAT",
            discount_header: "Discount",
            total_header: "Total",
            total_caption: "Total",
            plus_vat: "plus VAT",
            email: "E-mail",
            bank: "Bank",
            iban: "IBAN",
            ch_vat_number: "CH VAT No.",
            website: "Website",
            receipt: "Receipt",
            payment_part: "Payment part",
            account_payable_to: "Account / Payable to",
            reference: "Reference",
            additional_info: "Additional information",
            payable_by: "Payable by",
            currency: "Currency",
            amount: "Amount",
            acceptance_point: "Acceptance point",
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

/// Get the effective IBAN for SPC payload: QR-IBAN if available, otherwise regular IBAN.
/// QR-IBAN triggers QRR reference type; regular IBAN triggers SCOR.
fn effective_spc_iban(bank: &bank_account::Model) -> String {
    bank.qr_iban.as_deref()
        .unwrap_or(&bank.iban)
        .replace(' ', "")
}

fn generate_qr_image(data: &PdfData) -> Option<Vec<u8>> {
    let bank = data.bank.as_ref()?;
    let inv = &data.invoice;
    let s = &data.settings;
    let c = &data.contact;
    let inv_number = inv.invoice_number.as_deref().unwrap_or("DRAFT");
    let is_draft = inv.invoice_number.is_none();
    let spc_iban = effective_spc_iban(bank);
    let ref_info = qr_bill::build_reference(&spc_iban, inv_number, is_draft);
    // SPC payload amount: plain decimal, no thousand separators, 2 decimal places (§4.2.2)
    let amount = format!("{:.2}", inv.total);
    let creditor = qr_bill::QrCreditor {
        iban: spc_iban.clone(),
        name: s.legal_name.clone(),
        street: s.street.clone(),
        postal_code: s.postal_code.clone(),
        city: s.city.clone(),
        country: s.country.clone(),
    };
    let debtor = qr_bill::QrDebtor {
        name: c.name1.clone(),
        street: c.address.clone().unwrap_or_default(),
        postal_code: c.postal_code.clone().unwrap_or_default(),
        city: c.city.clone().unwrap_or_default(),
        country: c.country.clone().unwrap_or("CH".into()),
    };
    let payload = qr_bill::generate_spc_payload(
        &creditor, &debtor, &amount, &data.currency, &ref_info.ref_type, &ref_info.reference, inv_number,
    );
    qr_bill::generate_qr_png(&payload).ok()
}

/// Format amount per SIX QR-bill spec §3.5.3: space as thousand separator, dot decimal, 2 places.
/// e.g. "1 590.00"
fn fmt_money_qr(d: Decimal) -> String {
    let s = format!("{:.2}", d);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let dec_part = parts.get(1).unwrap_or(&"00");
    let formatted: String = int_part.chars().rev().collect::<Vec<_>>()
        .chunks(3).map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>().join(" ")
        .chars().rev().collect();
    format!("{formatted}.{dec_part}")
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

fn format_iban(iban: &str) -> String {
    iban.chars().filter(|c| !c.is_whitespace())
        .collect::<Vec<_>>().chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>().join(" ")
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
