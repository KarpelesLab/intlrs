//! Readable grapheme segmentation checks. The exhaustive UAX #29 run lives in
//! `grapheme_conformance.rs`.

use intl::unicode::{graphemes, line_breaks, sentences, words};

fn g(s: &str) -> Vec<&str> {
    graphemes(s).collect()
}

#[test]
fn line_break_opportunities() {
    // Spaces and hyphens are break opportunities; "can't" stays whole.
    let parts: Vec<&str> = line_breaks("a big-ish can't").map(|b| b.text).collect();
    assert_eq!(parts, ["a ", "big-", "ish ", "can't"]);

    // A newline is a mandatory break.
    let mut it = line_breaks("a\nb");
    let first = it.next().unwrap();
    assert_eq!(first.text, "a\n");
    assert!(first.mandatory);
}

#[test]
fn sentence_boundaries() {
    let s: Vec<&str> = sentences("Hello world. How are you? I'm fine.").collect();
    assert_eq!(s, ["Hello world. ", "How are you? ", "I'm fine."]);
}

#[test]
fn word_boundaries() {
    let w: Vec<&str> = words("The (quick) fox").collect();
    assert_eq!(w, ["The", " ", "(", "quick", ")", " ", "fox"]);

    // Numbers with internal separators stay together; filter for word-like tokens.
    let toks: Vec<&str> = words("Hello, world! 3.14")
        .filter(|w| w.chars().next().is_some_and(char::is_alphanumeric))
        .collect();
    assert_eq!(toks, ["Hello", "world", "3.14"]);
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
