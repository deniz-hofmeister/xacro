//! Port of upstream `TestMatchXML`, which checks the comparison helper itself.
//! Not ported: `test_match_unordered_dicts`, `test_mismatch_different_dicts` (dicts are out of scope).
mod common;

use common::{text_matches, xml_matches};

#[test]
fn test_normalize_whitespace_text() {
    assert!(text_matches("", " \t\n\r"));
}

#[test]
fn test_normalize_whitespace_trim() {
    assert!(text_matches(" foo bar ", "foo \t\n\r bar"));
}

#[test]
fn test_match_similar_numbers() {
    assert!(text_matches("0.123456789", "0.123456788"));
}

#[test]
fn test_mismatch_different_numbers() {
    assert!(!text_matches("0.123456789", "0.1234567879"));
}

#[test]
fn test_empty_node_vs_whitespace() {
    assert!(xml_matches("<foo/>", "<foo> \t\n\r </foo>", false));
}

#[test]
fn test_whitespace_vs_empty_node() {
    assert!(xml_matches("<foo> \t\n\r </foo>", "<foo/>", false));
}

#[test]
fn test_normalize_whitespace_nested() {
    assert!(xml_matches("<a><b/></a>", "<a>\n<b> </b> </a>", false));
}

#[test]
fn test_ignore_comments() {
    assert!(xml_matches(
        "<a><b/><!-- foo --> <!-- bar --></a>",
        "<a><b/></a>",
        true
    ));
}
