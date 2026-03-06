pub const SUPPORTED_LANGUAGES: [&str; 4] = ["en", "de", "fr", "it"];

pub fn normalize_language(input: Option<&str>) -> Option<String> {
    let raw = input?.trim();
    if raw.is_empty() {
        return None;
    }

    let base = raw
        .split(['-', '_'])
        .next()
        .unwrap_or(raw)
        .trim()
        .to_ascii_lowercase();

    let normalized = match base.as_str() {
        "en" | "english" => "en",
        "de" | "deutsch" | "german" => "de",
        "fr" | "french" | "francais" | "français" => "fr",
        "it" | "italian" | "italiano" => "it",
        _ => return None,
    };

    Some(normalized.to_string())
}

pub fn normalize_or_default(input: Option<&str>, default_language: &str) -> String {
    normalize_language(input)
        .or_else(|| normalize_language(Some(default_language)))
        .unwrap_or_else(|| "en".to_string())
}
