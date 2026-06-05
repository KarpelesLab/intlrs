#![cfg(feature = "segmentation")]
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

/// Regression for the SB8 lookahead quadratic-DoS: an ATerm followed by a long
/// run of Sp (or Close) used to re-scan the whole remaining run per character
/// (O(n²); 16k chars ≈ 5s). With the memoized lookahead it is linear, so a 50k
/// input completes effectively instantly. Mere completion here proves linearity.
#[test]
fn sentence_sb8_lookahead_is_linear() {
    // ATerm "." then n spaces: each space keeps `term` in the ATerm sequence,
    // so the SB8 lookahead is invoked every iteration. There is no Lower/stopper
    // ahead, so the whole run is a single sentence (no break before eot).
    for filler in [' ', ')'] {
        let small = format!(".{}", String::from(filler).repeat(8));
        let small_out: Vec<&str> = sentences(&small).collect();
        assert_eq!(small_out, [small.as_str()]); // one sentence, no interior break

        let big = format!(".{}", String::from(filler).repeat(50_000));
        let big_out: Vec<&str> = sentences(&big).collect();
        assert_eq!(big_out.len(), 1);
        assert_eq!(big_out[0].len(), big.len());
    }
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
