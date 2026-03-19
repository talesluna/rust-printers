/// Maps a paper size name to a Windows DMPAPER_* constant value.
///
/// Accepts standard paper size names (case-insensitive) or numeric IDs.
/// Returns `None` if the value is not recognized.
pub fn parse_paper_size(value: &str) -> Option<i16> {
    Some(match value.to_ascii_uppercase().as_str() {
        "LETTER" => 1,
        "LETTERSMALL" => 2,
        "TABLOID" => 3,
        "LEDGER" => 4,
        "LEGAL" => 5,
        "EXECUTIVE" => 7,
        "A3" => 8,
        "A4" => 9,
        "A4SMALL" => 10,
        "A5" => 11,
        "B4" | "B4_JIS" => 12,
        "B5" | "B5_JIS" => 13,
        _ => return value.parse().ok(),
    })
}

/// Maps a media source / input slot name to a Windows DMBIN_* constant value.
///
/// Accepts standard tray names (case-insensitive) or numeric IDs.
/// Returns `None` if the value is not recognized.
pub fn parse_media_source(value: &str) -> Option<i16> {
    Some(match value.to_ascii_uppercase().as_str() {
        "AUTO" => 7,
        "UPPER" | "TRAY1" | "ONLYONE" => 1,
        "LOWER" | "TRAY2" => 2,
        "MIDDLE" | "TRAY3" => 3,
        "MANUAL" => 4,
        "ENVELOPE" => 5,
        "ENVMANUAL" => 6,
        "TRACTOR" => 8,
        "SMALLFMT" => 9,
        "LARGEFMT" => 10,
        "LARGECAPACITY" => 11,
        "CASSETTE" => 14,
        "FORMSOURCE" => 15,
        _ => return value.parse().ok(),
    })
}

/// Maps a color model name to a Windows DMCOLOR_* constant value.
///
/// Returns `None` if the value is not recognized.
pub fn parse_color_model(value: &str) -> Option<i16> {
    Some(match value.to_ascii_uppercase().as_str() {
        "GRAY" | "MONOCHROME" => 1,
        "COLOR" => 2,
        _ => return value.parse().ok(),
    })
}

/// Maps a duplex / sides name to a Windows DMDUP_* constant value.
///
/// Returns `None` if the value is not recognized.
pub fn parse_duplex(value: &str) -> Option<i16> {
    Some(match value.to_ascii_uppercase().as_str() {
        "ONE-SIDED" | "SIMPLEX" => 1,
        "TWO-SIDED-LONG-EDGE" => 2,
        "TWO-SIDED-SHORT-EDGE" => 3,
        _ => return value.parse().ok(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_paper_size_named() {
        assert_eq!(parse_paper_size("A4"), Some(9));
        assert_eq!(parse_paper_size("a4"), Some(9));
        assert_eq!(parse_paper_size("A3"), Some(8));
        assert_eq!(parse_paper_size("A5"), Some(11));
        assert_eq!(parse_paper_size("B4"), Some(12));
        assert_eq!(parse_paper_size("B5"), Some(13));
        assert_eq!(parse_paper_size("B4_JIS"), Some(12));
        assert_eq!(parse_paper_size("B5_JIS"), Some(13));
        assert_eq!(parse_paper_size("Letter"), Some(1));
        assert_eq!(parse_paper_size("LEGAL"), Some(5));
        assert_eq!(parse_paper_size("EXECUTIVE"), Some(7));
        assert_eq!(parse_paper_size("TABLOID"), Some(3));
        assert_eq!(parse_paper_size("LEDGER"), Some(4));
    }

    #[test]
    fn test_parse_paper_size_numeric() {
        assert_eq!(parse_paper_size("9"), Some(9));
        assert_eq!(parse_paper_size("13"), Some(13));
        assert_eq!(parse_paper_size("256"), Some(256));
    }

    #[test]
    fn test_parse_paper_size_invalid() {
        assert_eq!(parse_paper_size("UNKNOWN"), None);
        assert_eq!(parse_paper_size(""), None);
        assert_eq!(parse_paper_size("abc"), None);
    }

    #[test]
    fn test_parse_media_source_named() {
        assert_eq!(parse_media_source("Auto"), Some(7));
        assert_eq!(parse_media_source("AUTO"), Some(7));
        assert_eq!(parse_media_source("Tray1"), Some(1));
        assert_eq!(parse_media_source("Tray2"), Some(2));
        assert_eq!(parse_media_source("Tray3"), Some(3));
        assert_eq!(parse_media_source("UPPER"), Some(1));
        assert_eq!(parse_media_source("LOWER"), Some(2));
        assert_eq!(parse_media_source("MIDDLE"), Some(3));
        assert_eq!(parse_media_source("MANUAL"), Some(4));
        assert_eq!(parse_media_source("ENVELOPE"), Some(5));
        assert_eq!(parse_media_source("CASSETTE"), Some(14));
        assert_eq!(parse_media_source("ONLYONE"), Some(1));
    }

    #[test]
    fn test_parse_media_source_numeric() {
        assert_eq!(parse_media_source("7"), Some(7));
        assert_eq!(parse_media_source("1"), Some(1));
        assert_eq!(parse_media_source("256"), Some(256));
    }

    #[test]
    fn test_parse_media_source_invalid() {
        assert_eq!(parse_media_source("UNKNOWN"), None);
        assert_eq!(parse_media_source(""), None);
        assert_eq!(parse_media_source("xyz"), None);
    }

    #[test]
    fn test_parse_color_model() {
        assert_eq!(parse_color_model("Gray"), Some(1));
        assert_eq!(parse_color_model("gray"), Some(1));
        assert_eq!(parse_color_model("Monochrome"), Some(1));
        assert_eq!(parse_color_model("Color"), Some(2));
        assert_eq!(parse_color_model("color"), Some(2));
        assert_eq!(parse_color_model("1"), Some(1));
        assert_eq!(parse_color_model("2"), Some(2));
        assert_eq!(parse_color_model("UNKNOWN"), None);
        assert_eq!(parse_color_model(""), None);
    }

    #[test]
    fn test_parse_duplex() {
        assert_eq!(parse_duplex("one-sided"), Some(1));
        assert_eq!(parse_duplex("simplex"), Some(1));
        assert_eq!(parse_duplex("two-sided-long-edge"), Some(2));
        assert_eq!(parse_duplex("two-sided-short-edge"), Some(3));
        assert_eq!(parse_duplex("1"), Some(1));
        assert_eq!(parse_duplex("2"), Some(2));
        assert_eq!(parse_duplex("3"), Some(3));
        assert_eq!(parse_duplex("UNKNOWN"), None);
        assert_eq!(parse_duplex(""), None);
    }
}
