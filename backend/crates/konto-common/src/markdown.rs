/// Convert Markdown to Typst markup for PDF rendering.
/// Handles: bold, italic, bullet lists, ordered lists, line breaks.
/// Plain text passes through unchanged.
pub fn md_to_typst(input: &str) -> String {
    let mut result = Vec::new();

    for line in input.lines() {
        let trimmed = line.trim();

        // Bullet list items: "- text" or "* text"
        if let Some(rest) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")) {
            result.push(format!("- {}", inline_md_to_typst(rest)));
            continue;
        }

        // Ordered list items: "1. text", "2. text", etc.
        if let Some(dot_pos) = trimmed.find(". ") {
            let prefix = &trimmed[..dot_pos];
            if prefix.chars().all(|c| c.is_ascii_digit()) && !prefix.is_empty() {
                let rest = &trimmed[dot_pos + 2..];
                result.push(format!("+ {}", inline_md_to_typst(rest)));
                continue;
            }
        }

        // Regular line
        result.push(inline_md_to_typst(trimmed));
    }

    result.join("\\\n")
}

/// Convert inline Markdown formatting to Typst.
fn inline_md_to_typst(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Bold: **text**
        if i + 1 < len
            && chars[i] == '*'
            && chars[i + 1] == '*'
            && let Some(end) = find_closing(&chars, i + 2, &['*', '*'])
        {
            let inner: String = chars[i + 2..end].iter().collect();
            out.push('*');
            out.push_str(&esc_typst(&inner));
            out.push('*');
            i = end + 2;
            continue;
        }

        // Italic: *text*
        if chars[i] == '*'
            && let Some(end) = find_closing_single(&chars, i + 1, '*')
        {
            let inner: String = chars[i + 1..end].iter().collect();
            out.push('_');
            out.push_str(&esc_typst(&inner));
            out.push('_');
            i = end + 1;
            continue;
        }

        // Escape Typst special characters in regular text
        match chars[i] {
            '#' => out.push_str("\\#"),
            '@' => out.push_str("\\@"),
            '<' => out.push_str("\\<"),
            '>' => out.push_str("\\>"),
            '$' => out.push_str("\\$"),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
        i += 1;
    }

    out
}

fn find_closing(chars: &[char], start: usize, pattern: &[char; 2]) -> Option<usize> {
    let len = chars.len();
    let mut i = start;
    while i + 1 < len {
        if chars[i] == pattern[0] && chars[i + 1] == pattern[1] {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn find_closing_single(chars: &[char], start: usize, ch: char) -> Option<usize> {
    chars[start..]
        .iter()
        .position(|&c| c == ch)
        .map(|p| p + start)
}

/// Escape Typst special characters (subset, for use inside markup blocks)
fn esc_typst(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('@', "\\@")
        .replace('<', "\\<")
        .replace('>', "\\>")
        .replace('$', "\\$")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text() {
        assert_eq!(md_to_typst("hello world"), "hello world");
    }

    #[test]
    fn bold() {
        assert_eq!(md_to_typst("some **bold** text"), "some *bold* text");
    }

    #[test]
    fn italic() {
        assert_eq!(md_to_typst("some *italic* text"), "some _italic_ text");
    }

    #[test]
    fn bullet_list() {
        let input = "- item one\n- item two";
        let output = md_to_typst(input);
        assert!(output.contains("- item one"));
        assert!(output.contains("- item two"));
    }

    #[test]
    fn ordered_list() {
        let input = "1. first\n2. second";
        let output = md_to_typst(input);
        assert!(output.contains("+ first"));
        assert!(output.contains("+ second"));
    }

    #[test]
    fn special_chars_escaped() {
        assert_eq!(md_to_typst("price #1 <test>"), "price \\#1 \\<test\\>");
    }

    #[test]
    fn multiline() {
        let input = "line one\nline two";
        let output = md_to_typst(input);
        assert_eq!(output, "line one\\\nline two");
    }
}
