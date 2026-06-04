//! Readable grapheme segmentation checks. The exhaustive UAX #29 run lives in
//! `grapheme_conformance.rs`.

use intl::unicode::graphemes;

fn g(s: &str) -> Vec<&str> {
    graphemes(s).collect()
}

#[test]
fn ascii_and_crlf() {
    assert_eq!(g("abc"), ["a", "b", "c"]);
    assert_eq!(g("a\r\nb"), ["a", "\r\n", "b"]); // CRLF is one cluster
}

#[cfg(feature = "bmp")]
#[test]
fn combining_marks() {
    // base + combining acute is a single grapheme (the slice stays decomposed).
    assert_eq!(g("e\u{0301}x"), ["e\u{0301}", "x"]);
    assert_eq!(graphemes("e\u{0301}").count(), 1);
}

#[cfg(feature = "full")]
#[test]
fn emoji_and_flags() {
    // Each regional-indicator pair is one flag cluster.
    assert_eq!(g("🇫🇷🇩🇪"), ["🇫🇷", "🇩🇪"]);
    // A ZWJ family emoji is a single cluster.
    assert_eq!(graphemes("👨\u{200D}👩\u{200D}👧").count(), 1);
    // Skin-tone modifier joins its base.
    assert_eq!(graphemes("👍🏽").count(), 1);
}
