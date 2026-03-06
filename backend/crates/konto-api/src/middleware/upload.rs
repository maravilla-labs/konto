use konto_common::error::AppError;

const MAX_ATTACHMENT_SIZE: usize = 10 * 1024 * 1024; // 10MB
const MAX_AVATAR_SIZE: usize = 2 * 1024 * 1024; // 2MB

const ALLOWED_DOCUMENT_TYPES: &[(&str, &[&str])] = &[
    ("application/pdf", &["pdf"]),
    ("image/png", &["png"]),
    ("image/jpeg", &["jpg", "jpeg"]),
    ("image/gif", &["gif"]),
    ("image/webp", &["webp"]),
    (
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        &["xlsx"],
    ),
    ("text/csv", &["csv"]),
    ("text/plain", &["txt"]),
];

const ALLOWED_IMAGE_TYPES: &[(&str, &[&str])] = &[
    ("image/png", &["png"]),
    ("image/jpeg", &["jpg", "jpeg"]),
    ("image/webp", &["webp"]),
];

pub fn validate_document_upload(
    file_name: &str,
    mime_type: &str,
    data: &[u8],
) -> Result<(), AppError> {
    validate_upload(file_name, mime_type, data, MAX_ATTACHMENT_SIZE, ALLOWED_DOCUMENT_TYPES)
}

pub fn validate_image_upload(
    file_name: &str,
    mime_type: &str,
    data: &[u8],
) -> Result<(), AppError> {
    validate_upload(file_name, mime_type, data, MAX_AVATAR_SIZE, ALLOWED_IMAGE_TYPES)
}

fn validate_upload(
    file_name: &str,
    mime_type: &str,
    data: &[u8],
    max_size: usize,
    allowed_types: &[(&str, &[&str])],
) -> Result<(), AppError> {
    if data.len() > max_size {
        return Err(AppError::Validation(format!(
            "File too large. Maximum size is {} MB",
            max_size / (1024 * 1024)
        )));
    }

    if !allowed_types.iter().any(|(m, _)| *m == mime_type) {
        return Err(AppError::Validation(format!(
            "File type '{}' is not allowed",
            mime_type
        )));
    }

    let ext = file_name.rsplit('.').next().unwrap_or("").to_lowercase();
    let ext_matches = allowed_types
        .iter()
        .any(|(m, exts)| *m == mime_type && exts.iter().any(|e| *e == ext));
    if !ext_matches {
        return Err(AppError::Validation(
            "File extension does not match content type".to_string(),
        ));
    }

    validate_magic_bytes(mime_type, data)?;

    Ok(())
}

fn validate_magic_bytes(mime_type: &str, data: &[u8]) -> Result<(), AppError> {
    let valid = match mime_type {
        "image/png" => data.starts_with(b"\x89PNG\r\n\x1a\n"),
        "image/jpeg" => data.starts_with(b"\xFF\xD8\xFF"),
        "image/webp" => data.len() >= 12 && &data[..4] == b"RIFF" && &data[8..12] == b"WEBP",
        "image/gif" => data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a"),
        "application/pdf" => data.starts_with(b"%PDF"),
        // text/*, xlsx: no reliable magic check
        _ => return Ok(()),
    };
    if !valid {
        return Err(AppError::Validation(
            "File content does not match declared type".to_string(),
        ));
    }
    Ok(())
}
