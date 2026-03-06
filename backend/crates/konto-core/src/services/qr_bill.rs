use konto_common::error::AppError;

/// Creditor info for QR-bill
pub struct QrCreditor {
    pub iban: String,
    pub name: String,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
}

/// Debtor info for QR-bill
pub struct QrDebtor {
    pub name: String,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
}

/// Generate ISO 11649 Creditor Reference (SCOR) from invoice number.
/// Format: RF + 2-digit check + reference (alphanumeric, max 21 chars).
/// Check digits calculated via ISO 7064 Mod 97-10.
pub fn generate_creditor_reference(invoice_number: &str) -> String {
    // Strip non-alphanumeric characters and uppercase
    let reference: String = invoice_number
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .collect();
    // Truncate to 21 chars max (spec: reference part max 21)
    let reference = if reference.len() > 21 {
        &reference[..21]
    } else {
        &reference
    };
    // ISO 11649: append "RF00" to reference, convert letters→numbers, mod 97
    let check_string = format!("{reference}RF00");
    let numeric = letters_to_digits(&check_string);
    let remainder = mod97(&numeric);
    let check_digits = 98 - remainder;
    format!("RF{check_digits:02}{reference}")
}

/// Verify an ISO 11649 Creditor Reference check digit.
/// Returns true if the reference is valid.
pub fn verify_creditor_reference(full_ref: &str) -> bool {
    if !full_ref.starts_with("RF") || full_ref.len() < 5 || full_ref.len() > 25 {
        return false;
    }
    // Move first 4 chars (RF + check digits) to end
    let rearranged = format!("{}{}", &full_ref[4..], &full_ref[..4]);
    let numeric = letters_to_digits(&rearranged);
    mod97(&numeric) == 1
}

/// Format SPC amount per §4.2.2: no leading zeros, dot decimal, 2 decimal places,
/// no thousand separators. Range: 0.01–999999999.99.
pub fn format_spc_amount(amount: &str) -> String {
    // Strip any thousand separators (spaces, apostrophes, commas used as grouping)
    let clean: String = amount.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
    // Parse and reformat to ensure exactly 2 decimal places
    if let Ok(val) = clean.parse::<f64>() {
        format!("{:.2}", val)
    } else {
        clean
    }
}

/// Generate Swiss QR-bill SPC payload (v0200) per SIX IG QR-bill v2.3 §4.2.2.
/// All fields are truncated to their spec max lengths.
pub fn generate_spc_payload(
    creditor: &QrCreditor,
    debtor: &QrDebtor,
    amount: &str,
    currency: &str,
    reference_type: &str,
    reference: &str,
    additional_info: &str,
) -> String {
    // Truncate fields per SIX spec Table 8 max lengths
    let cr_iban = truncate(&creditor.iban, 21);
    let cr_name = truncate(&creditor.name, 70);
    let cr_street = truncate(&creditor.street, 70);
    let cr_postal = truncate(&creditor.postal_code, 16);
    let cr_city = truncate(&creditor.city, 35);
    let cr_country = truncate(&creditor.country, 2);
    let db_name = truncate(&debtor.name, 70);
    let db_street = truncate(&debtor.street, 70);
    let db_postal = truncate(&debtor.postal_code, 16);
    let db_city = truncate(&debtor.city, 35);
    let db_country = truncate(&debtor.country, 2);
    // QRR: exactly 27 digits, SCOR: max 25 chars
    let ref_max = if reference_type == "QRR" { 27 } else { 25 };
    let ref_val = truncate(reference, ref_max);
    let add_info = truncate(additional_info, 140);
    let amt = format_spc_amount(amount);

    let mut lines: Vec<&str> = Vec::with_capacity(32);
    lines.push("SPC");            // QRType: fixed "SPC" (§4.2.2)
    lines.push("0200");           // Version: fixed "0200" (§4.2.2)
    lines.push("1");              // Coding: fixed "1" = UTF-8 (§4.2.2)
    lines.push(&cr_iban);         // IBAN: 21 chars, no spaces, CH/LI only
    lines.push("S");              // Creditor address type: Structured
    lines.push(&cr_name);         // Creditor name: max 70
    lines.push(&cr_street);       // Creditor street: max 70
    lines.push("");               // Creditor building number: empty for Type S
    lines.push(&cr_postal);       // Creditor postal code: max 16
    lines.push(&cr_city);         // Creditor city: max 35
    lines.push(&cr_country);      // Creditor country: 2-char ISO 3166-1
    // Ultimate creditor: 7 empty fields (Status X = must not be filled)
    lines.extend_from_slice(&["", "", "", "", "", "", ""]);
    lines.push(&amt);             // Amount: decimal, 2 places, no separators
    lines.push(currency);         // Currency: CHF or EUR
    // Ultimate debtor (optional group, we always fill it for invoices)
    lines.push("S");              // Debtor address type: Structured
    lines.push(&db_name);         // Debtor name: max 70
    lines.push(&db_street);       // Debtor street: max 70
    lines.push("");               // Debtor building number: empty for Type S
    lines.push(&db_postal);       // Debtor postal code: max 16
    lines.push(&db_city);         // Debtor city: max 35
    lines.push(&db_country);      // Debtor country: 2-char ISO 3166-1
    lines.push(reference_type);    // Reference type: QRR, SCOR, or NON
    lines.push(&ref_val);         // Reference: SCOR creditor reference
    lines.push(&add_info);        // Unstructured message (Ustrd): max 140
    lines.push("EPD");            // Trailer: fixed "EPD"
    // No trailing newline after last element (§4.1.4)
    lines.join("\n")
}

/// Official SIX Swiss cross PNG (7mm) per IG QR-bill v2.3 §6.4.2
const SWISS_CROSS_PNG: &[u8] = include_bytes!("../../assets/CH-Kreuz_7mm.png");

/// Generate QR code as PNG bytes from the SPC payload.
/// Uses error correction level M per SIX QR-bill spec §6.
/// Overlays the official Swiss cross PNG (7mm) per §6.4.2.
pub fn generate_qr_png(payload: &str) -> Result<Vec<u8>, AppError> {
    use image::{GenericImageView, ImageEncoder};
    use image::codecs::png::PngEncoder;
    use qrcode::{QrCode, EcLevel};

    let code = QrCode::with_error_correction_level(payload.as_bytes(), EcLevel::M)
        .map_err(|e| AppError::Internal(format!("QR code generation failed: {e}")))?;
    let modules = code.to_colors();
    let width = code.width() as u32;
    let scale = 10u32;
    let border = 4u32 * scale;
    let img_size = width * scale + 2 * border;

    let mut pixels = vec![255u8; (img_size * img_size) as usize];
    for (y, row) in modules.chunks(width as usize).enumerate() {
        for (x, &module) in row.iter().enumerate() {
            if module == qrcode::Color::Dark {
                let px = x as u32 * scale + border;
                let py = y as u32 * scale + border;
                for dy in 0..scale {
                    for dx in 0..scale {
                        let idx = ((py + dy) * img_size + (px + dx)) as usize;
                        pixels[idx] = 0;
                    }
                }
            }
        }
    }

    // Overlay official Swiss cross PNG (7×7mm on 46×46mm = 15.2%) per §6.4.2
    let cross_img = image::load_from_memory(SWISS_CROSS_PNG)
        .map_err(|e| AppError::Internal(format!("Swiss cross PNG load failed: {e}")))?;
    let (src_w, src_h) = cross_img.dimensions();
    let cross_size = img_size * 152 / 1000;
    let center = img_size / 2;
    let half_cross = cross_size / 2;

    // Clear white quiet zone slightly larger than cross
    let margin = cross_size / 8;
    for dy in 0..(cross_size + 2 * margin) {
        for dx in 0..(cross_size + 2 * margin) {
            let px = center - half_cross - margin + dx;
            let py = center - half_cross - margin + dy;
            if px < img_size && py < img_size {
                pixels[(py * img_size + px) as usize] = 255;
            }
        }
    }

    // Composite official Swiss cross using nearest-neighbor scaling
    for dy in 0..cross_size {
        for dx in 0..cross_size {
            let px = center - half_cross + dx;
            let py = center - half_cross + dy;
            if px >= img_size || py >= img_size { continue; }
            let sx = (dx as u64 * src_w as u64 / cross_size as u64) as u32;
            let sy = (dy as u64 * src_h as u64 / cross_size as u64) as u32;
            if sx < src_w && sy < src_h {
                let pixel = cross_img.get_pixel(sx, sy);
                // Source is grayscale/RGB — use luminance
                let luma = pixel[0];
                pixels[(py * img_size + px) as usize] = luma;
            }
        }
    }

    let mut png_bytes = Vec::new();
    let encoder = PngEncoder::new(&mut png_bytes);
    encoder
        .write_image(&pixels, img_size, img_size, image::ExtendedColorType::L8)
        .map_err(|e| AppError::Internal(format!("PNG encoding failed: {e}")))?;
    Ok(png_bytes)
}

/// Truncate string to max chars (Unicode-aware).
fn truncate(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}

/// Replace letters with their numeric equivalents (A=10, B=11, ..., Z=35).
fn letters_to_digits(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            result.push(c);
        } else if c.is_ascii_alphabetic() {
            let val = c.to_ascii_uppercase() as u32 - b'A' as u32 + 10;
            result.push_str(&val.to_string());
        }
    }
    result
}

/// Calculate modulo 97 for a large numeric string (ISO 7064 Mod 97-10).
fn mod97(s: &str) -> u64 {
    let mut remainder: u64 = 0;
    for chunk in s.as_bytes().chunks(9) {
        let part = std::str::from_utf8(chunk).unwrap_or("0");
        let combined = format!("{remainder}{part}");
        remainder = combined.parse::<u64>().unwrap_or(0) % 97;
    }
    remainder
}

/// Check if an IBAN is a QR-IBAN (Swiss QR-IID range 30000-31999).
/// QR-IBANs require QRR reference type; regular IBANs require SCOR.
pub fn is_qr_iban(iban: &str) -> bool {
    let clean: String = iban.chars().filter(|c| !c.is_whitespace()).collect();
    if clean.len() < 9 {
        return false;
    }
    let upper = clean.to_ascii_uppercase();
    if !upper.starts_with("CH") && !upper.starts_with("LI") {
        return false;
    }
    if let Ok(iid) = upper[4..9].parse::<u32>() {
        (30000..=31999).contains(&iid)
    } else {
        false
    }
}

/// Mod10 recursive check digit per Swiss QRR/ISR standard.
fn mod10_recursive_check(digits: &str) -> u8 {
    const TABLE: [u8; 10] = [0, 9, 4, 6, 8, 2, 7, 1, 3, 5];
    let mut carry: u8 = 0;
    for ch in digits.chars() {
        if let Some(d) = ch.to_digit(10) {
            carry = TABLE[((carry as u32 + d) % 10) as usize];
        }
    }
    (10 - carry) % 10
}

/// Generate QRR (QR-Referenz) from invoice number.
/// Returns 27-digit numeric reference with Mod10 recursive check digit.
/// Digits are extracted from the invoice number and left-padded to 26 digits.
pub fn generate_qrr_reference(invoice_number: &str) -> String {
    let digits: String = invoice_number.chars().filter(|c| c.is_ascii_digit()).collect();
    let padded = format!("{:0>26}", digits);
    let base = if padded.len() > 26 {
        &padded[padded.len() - 26..]
    } else {
        &padded[..]
    };
    let check = mod10_recursive_check(base);
    format!("{}{}", base, check)
}

/// Verify a QRR reference (27 digits, Mod10 recursive check digit).
pub fn verify_qrr_reference(reference: &str) -> bool {
    if reference.len() != 27 || !reference.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let base = &reference[..26];
    let check = mod10_recursive_check(base);
    let expected = reference.as_bytes()[26] - b'0';
    check == expected
}

/// Format QRR reference in 2 + 5×5 grouping per SIX spec §3.5.4.
/// e.g. "21 00000 00003 13947 14300 09017"
pub fn format_qrr_display(reference: &str) -> String {
    if reference.len() != 27 {
        return reference.to_string();
    }
    format!(
        "{} {} {} {} {} {}",
        &reference[0..2],
        &reference[2..7],
        &reference[7..12],
        &reference[12..17],
        &reference[17..22],
        &reference[22..27],
    )
}

/// Format SCOR Creditor Reference in 4-char blocks per SIX spec §3.5.4.
/// e.g. "RF72 0191 2301 0040 5JSH 0438"
pub fn format_scor_display(reference: &str) -> String {
    reference
        .chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Reference info for QR-bill generation.
pub struct QrReferenceInfo {
    /// SPC reference type: "QRR" or "SCOR"
    pub ref_type: String,
    /// Raw reference value for SPC payload
    pub reference: String,
    /// Formatted display string for QR-bill visual
    pub display: String,
}

/// Build the appropriate reference based on IBAN type.
/// QR-IBAN → QRR (27-digit numeric), regular IBAN → SCOR (ISO 11649).
/// For drafts, display shows placeholder text while QR payload uses valid reference.
pub fn build_reference(iban: &str, invoice_number: &str, is_draft: bool) -> QrReferenceInfo {
    let clean_iban: String = iban.chars().filter(|c| !c.is_whitespace()).collect();
    if is_qr_iban(&clean_iban) {
        let reference = generate_qrr_reference(invoice_number);
        let display = if is_draft {
            "** ***** ***** ***** ***** *****".to_string()
        } else {
            format_qrr_display(&reference)
        };
        QrReferenceInfo { ref_type: "QRR".into(), reference, display }
    } else {
        let reference = generate_creditor_reference(invoice_number);
        let display = if is_draft {
            "DRAFT".to_string()
        } else {
            format_scor_display(&reference)
        };
        QrReferenceInfo { ref_type: "SCOR".into(), reference, display }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // --- ISO 11649 Creditor Reference tests ---

    #[test]
    fn test_creditor_reference_format() {
        let r = generate_creditor_reference("RE-2026-001");
        assert!(r.starts_with("RF"));
        assert!(r.len() >= 5);  // min: RF + 2 check + 1 char
        assert!(r.len() <= 25); // max: RF + 2 check + 21 chars
    }

    #[test]
    fn test_creditor_reference_check_digit_valid() {
        // Generate and verify — round-trip must pass
        let refs = [
            "RE-2026-001",
            "RE-2026-099",
            "RE-2025-123",
            "1",
            "ABC123DEF456",
            "LONGINVOICENUMBER2026",
        ];
        for inv_nr in &refs {
            let r = generate_creditor_reference(inv_nr);
            assert!(
                verify_creditor_reference(&r),
                "Failed check digit verification for input '{}' → '{}'",
                inv_nr, r
            );
        }
    }

    #[test]
    fn test_creditor_reference_strips_special_chars() {
        let r = generate_creditor_reference("RE-2026-001");
        // Should not contain dashes
        assert!(!r[4..].contains('-'));
        // Reference part should be uppercase alphanumeric
        assert!(r[4..].chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_creditor_reference_max_length() {
        // Very long invoice number — should truncate reference to 21 chars
        let r = generate_creditor_reference("ABCDEFGHIJKLMNOPQRSTUVWXYZ12345");
        assert!(r.len() <= 25); // RF + 2 + max 21
        assert!(verify_creditor_reference(&r));
    }

    #[test]
    fn test_verify_invalid_references() {
        assert!(!verify_creditor_reference("RF00ABC"));  // invalid check digits
        assert!(!verify_creditor_reference("XX12ABC"));  // wrong prefix
        assert!(!verify_creditor_reference("RF12"));     // too short (4 chars < 5)
        assert!(!verify_creditor_reference(""));         // empty
    }

    // --- Mod 97 tests ---

    #[test]
    fn test_mod97_known_values() {
        // 123456789 % 97 = 39
        assert_eq!(mod97("123456789"), 39);
        assert_eq!(mod97("0"), 0);
        assert_eq!(mod97("97"), 0);
        assert_eq!(mod97("98"), 1);
        assert_eq!(mod97("100"), 3);
    }

    #[test]
    fn test_mod97_multi_chunk() {
        // 1234567890 spans 2 chunks (9 + 1): result = 2
        assert_eq!(mod97("1234567890"), 2);
        // Verify single-chunk still works
        assert_eq!(mod97("999999999"), 999999999u64 % 97);
    }

    // --- Letters to digits tests ---

    #[test]
    fn test_letters_to_digits() {
        assert_eq!(letters_to_digits("A"), "10");
        assert_eq!(letters_to_digits("Z"), "35");
        assert_eq!(letters_to_digits("RF00"), "271500");
        assert_eq!(letters_to_digits("AB12"), "101112");
        assert_eq!(letters_to_digits("123"), "123");
    }

    // --- SPC Payload tests ---

    #[test]
    fn test_spc_payload_line_count() {
        let payload = make_test_payload("1234.50");
        let lines: Vec<&str> = payload.split('\n').collect();
        assert_eq!(lines.len(), 31, "SPC payload must have exactly 31 lines");
    }

    #[test]
    fn test_spc_payload_header() {
        let payload = make_test_payload("100.00");
        let lines: Vec<&str> = payload.split('\n').collect();
        assert_eq!(lines[0], "SPC");
        assert_eq!(lines[1], "0200");
        assert_eq!(lines[2], "1");
    }

    #[test]
    fn test_spc_payload_no_trailing_newline() {
        let payload = make_test_payload("100.00");
        assert!(!payload.ends_with('\n'), "SPC payload must not end with newline (§4.1.4)");
    }

    #[test]
    fn test_spc_payload_creditor_fields() {
        let payload = make_test_payload("100.00");
        let lines: Vec<&str> = payload.split('\n').collect();
        assert_eq!(lines[3], "CH4431999123000889012"); // IBAN, no spaces
        assert_eq!(lines[4], "S");                      // Address type
        assert_eq!(lines[5], "Acme GmbH");           // Name
        assert_eq!(lines[6], "Dorfstrasse 1");           // Street
        assert_eq!(lines[7], "");                        // Building nr (empty)
        assert_eq!(lines[8], "6300");                    // Postal code
        assert_eq!(lines[9], "Zug");                     // City
        assert_eq!(lines[10], "CH");                     // Country
    }

    #[test]
    fn test_spc_payload_ultimate_creditor_empty() {
        let payload = make_test_payload("100.00");
        let lines: Vec<&str> = payload.split('\n').collect();
        // Lines 11-17 (0-indexed) = Ultimate Creditor, all must be empty (Status X)
        for (i, line) in lines.iter().enumerate().skip(11).take(7) {
            assert_eq!(*line, "", "UltmtCdtr line {i} must be empty");
        }
    }

    #[test]
    fn test_spc_payload_amount_format() {
        // Amount must be plain decimal, 2 places, no thousand separators
        let payload = make_test_payload("1234.50");
        let lines: Vec<&str> = payload.split('\n').collect();
        assert_eq!(lines[18], "1234.50");
        assert_eq!(lines[19], "CHF");
    }

    #[test]
    fn test_spc_payload_amount_no_separators() {
        // Even if input has separators, they must be stripped
        let payload = make_test_payload("1'234.50");
        let lines: Vec<&str> = payload.split('\n').collect();
        assert_eq!(lines[18], "1234.50", "Amount must not contain thousand separators");
    }

    #[test]
    fn test_spc_payload_debtor_fields() {
        let payload = make_test_payload("100.00");
        let lines: Vec<&str> = payload.split('\n').collect();
        assert_eq!(lines[20], "S");                      // Address type
        assert_eq!(lines[21], "Max Muster AG");           // Name
        assert_eq!(lines[22], "Hauptstrasse 10");         // Street
        assert_eq!(lines[23], "");                        // Building nr
        assert_eq!(lines[24], "8000");                    // Postal code
        assert_eq!(lines[25], "Zürich");                  // City (UTF-8 ok)
        assert_eq!(lines[26], "CH");                      // Country
    }

    #[test]
    fn test_spc_payload_reference_and_trailer() {
        let payload = make_test_payload("100.00");
        let lines: Vec<&str> = payload.split('\n').collect();
        assert_eq!(lines[27], "SCOR");
        // Reference should be a valid SCOR ref
        assert!(lines[28].starts_with("RF"));
        assert_eq!(lines[30], "EPD");
    }

    #[test]
    fn test_spc_payload_field_truncation() {
        let long_name = "A".repeat(100); // > 70 char max
        let creditor = QrCreditor {
            iban: "CH4431999123000889012".into(),
            name: long_name,
            street: "B".repeat(100),
            postal_code: "C".repeat(20),
            city: "D".repeat(50),
            country: "CHE".into(), // > 2 chars
        };
        let debtor = QrDebtor {
            name: "E".repeat(100),
            street: "F".repeat(100),
            postal_code: "G".repeat(20),
            city: "H".repeat(50),
            country: "CH".into(),
        };
        let ref_str = generate_creditor_reference("RE-2026-001");
        let payload = generate_spc_payload(
            &creditor, &debtor, "100.00", "CHF", "SCOR", &ref_str, "test",
        );
        let lines: Vec<&str> = payload.split('\n').collect();
        assert!(lines[5].len() <= 70, "Creditor name must be ≤70 chars");
        assert!(lines[6].len() <= 70, "Creditor street must be ≤70 chars");
        assert!(lines[8].len() <= 16, "Creditor postal must be ≤16 chars");
        assert!(lines[9].len() <= 35, "Creditor city must be ≤35 chars");
        assert!(lines[10].len() <= 2, "Creditor country must be ≤2 chars");
        assert!(lines[21].len() <= 70, "Debtor name must be ≤70 chars");
        assert!(lines[22].len() <= 70, "Debtor street must be ≤70 chars");
        assert!(lines[24].len() <= 16, "Debtor postal must be ≤16 chars");
        assert!(lines[25].len() <= 35, "Debtor city must be ≤35 chars");
    }

    // --- Amount formatting tests ---

    #[test]
    fn test_format_spc_amount() {
        assert_eq!(format_spc_amount("1234.50"), "1234.50");
        assert_eq!(format_spc_amount("0.01"), "0.01");
        assert_eq!(format_spc_amount("999999999.99"), "999999999.99");
        assert_eq!(format_spc_amount("100"), "100.00");
    }

    #[test]
    fn test_format_spc_amount_strips_separators() {
        assert_eq!(format_spc_amount("1'234.50"), "1234.50");
        assert_eq!(format_spc_amount("1 234.50"), "1234.50");
        assert_eq!(format_spc_amount("1,234.50"), "1234.50"); // comma as grouping
    }

    // --- QR PNG generation test ---

    #[test]
    fn test_qr_png_generates_valid_png() {
        let payload = make_test_payload("100.00");
        let png = generate_qr_png(&payload).expect("QR PNG generation failed");
        // PNG magic bytes
        assert_eq!(&png[..4], &[0x89, 0x50, 0x4E, 0x47], "Must be valid PNG");
        assert!(png.len() > 100, "PNG must have reasonable size");
    }

    // --- QR-IBAN detection tests ---

    #[test]
    fn test_is_qr_iban() {
        // QR-IID range 30000-31999
        assert!(is_qr_iban("CH4431999123000889012"));  // IID 31999
        assert!(is_qr_iban("CH44 3000 0123 0008 8901 2")); // IID 30000 with spaces
        assert!(is_qr_iban("CH4430000123000889012"));  // IID 30000
    }

    #[test]
    fn test_is_not_qr_iban() {
        assert!(!is_qr_iban("CH9300762011623852957")); // IID 00762 (regular)
        assert!(!is_qr_iban("CH4400762011623852957")); // IID 00762
        assert!(!is_qr_iban("DE89370400440532013000")); // German IBAN
        assert!(!is_qr_iban(""));
        assert!(!is_qr_iban("CH12"));
    }

    // --- QRR reference tests ---

    #[test]
    fn test_qrr_reference_length() {
        let r = generate_qrr_reference("RE-2026-001");
        assert_eq!(r.len(), 27, "QRR must be exactly 27 digits");
        assert!(r.chars().all(|c| c.is_ascii_digit()), "QRR must be all digits");
    }

    #[test]
    fn test_qrr_reference_round_trip() {
        let refs = ["RE-2026-001", "RE-2026-099", "1", "123456789", "DRAFT"];
        for inv_nr in &refs {
            let r = generate_qrr_reference(inv_nr);
            assert!(
                verify_qrr_reference(&r),
                "QRR verification failed for input '{}' → '{}'", inv_nr, r
            );
        }
    }

    #[test]
    fn test_qrr_reference_contains_digits() {
        // "RE-2026-001" → digits "2026001" → padded to 26 + check
        let r = generate_qrr_reference("RE-2026-001");
        assert!(r.contains("2026001"), "QRR should contain invoice digits: {}", r);
    }

    #[test]
    fn test_verify_qrr_invalid() {
        assert!(!verify_qrr_reference("123")); // too short
        assert!(!verify_qrr_reference("ABCDEFGHIJKLMNOPQRSTUVWXYZ1")); // not digits
        // Wrong check digit (flip last digit)
        let r = generate_qrr_reference("RE-2026-001");
        let mut bad = r.clone();
        let last = bad.pop().unwrap();
        let flipped = if last == '0' { '1' } else { '0' };
        bad.push(flipped);
        assert!(!verify_qrr_reference(&bad));
    }

    #[test]
    fn test_qrr_display_format() {
        let r = generate_qrr_reference("RE-2026-001");
        let display = format_qrr_display(&r);
        // Must be "XX XXXXX XXXXX XXXXX XXXXX XXXXX" format (2 + 5*5 with spaces)
        let parts: Vec<&str> = display.split(' ').collect();
        assert_eq!(parts.len(), 6);
        assert_eq!(parts[0].len(), 2);
        for p in &parts[1..] {
            assert_eq!(p.len(), 5);
        }
    }

    #[test]
    fn test_mod10_recursive_known_values() {
        // Known ISR test: "00000000000000000002026001" → check digit
        assert_eq!(mod10_recursive_check("0"), 0);
        // Verify against known reference: all zeros → check digit 0
        assert_eq!(mod10_recursive_check("00000000000000000000000000"), 0);
    }

    // --- build_reference auto-detection tests ---

    #[test]
    fn test_build_reference_qr_iban_uses_qrr() {
        let info = build_reference("CH4431999123000889012", "RE-2026-001", false);
        assert_eq!(info.ref_type, "QRR");
        assert_eq!(info.reference.len(), 27);
        assert!(info.reference.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_build_reference_regular_iban_uses_scor() {
        let info = build_reference("CH9300762011623852957", "RE-2026-001", false);
        assert_eq!(info.ref_type, "SCOR");
        assert!(info.reference.starts_with("RF"));
    }

    #[test]
    fn test_build_reference_draft_display() {
        let qrr = build_reference("CH4431999123000889012", "DRAFT", true);
        assert_eq!(qrr.display, "** ***** ***** ***** ***** *****");
        let scor = build_reference("CH9300762011623852957", "DRAFT", true);
        assert_eq!(scor.display, "DRAFT");
    }

    // --- SPC payload with QRR ---

    #[test]
    fn test_spc_payload_with_qrr() {
        let creditor = QrCreditor {
            iban: "CH4431999123000889012".into(),
            name: "Acme GmbH".into(),
            street: "Dorfstrasse 1".into(),
            postal_code: "6300".into(),
            city: "Zug".into(),
            country: "CH".into(),
        };
        let debtor = QrDebtor {
            name: "Max Muster AG".into(),
            street: "Hauptstrasse 10".into(),
            postal_code: "8000".into(),
            city: "Zürich".into(),
            country: "CH".into(),
        };
        let qrr_ref = generate_qrr_reference("RE-2026-001");
        let payload = generate_spc_payload(
            &creditor, &debtor, "100.00", "CHF", "QRR", &qrr_ref, "RE-2026-001",
        );
        let lines: Vec<&str> = payload.split('\n').collect();
        assert_eq!(lines[27], "QRR");
        assert_eq!(lines[28].len(), 27); // QRR is 27 digits
    }

    // --- Helper ---

    fn make_test_payload(amount: &str) -> String {
        let creditor = QrCreditor {
            iban: "CH4431999123000889012".into(),
            name: "Acme GmbH".into(),
            street: "Dorfstrasse 1".into(),
            postal_code: "6300".into(),
            city: "Zug".into(),
            country: "CH".into(),
        };
        let debtor = QrDebtor {
            name: "Max Muster AG".into(),
            street: "Hauptstrasse 10".into(),
            postal_code: "8000".into(),
            city: "Zürich".into(),
            country: "CH".into(),
        };
        let reference = generate_creditor_reference("RE-2026-001");
        generate_spc_payload(
            &creditor, &debtor, amount, "CHF", "SCOR", &reference, "RE-2026-001",
        )
    }
}
