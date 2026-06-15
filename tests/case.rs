#![cfg(feature = "case")]
//! Full, unconditional case mapping and folding.

use intl::unicode::{case_fold, to_lowercase, to_titlecase, to_uppercase};

fn s(it: impl Iterator<Item = char>) -> String {
    it.collect()
}

#[test]
fn ascii_case() {
    assert_eq!(s(to_uppercase('a')), "A");
    assert_eq!(s(to_lowercase('Z')), "z");
    assert_eq!(s(to_uppercase('A')), "A"); // identity
    assert_eq!(s(to_titlecase('a')), "A");
    assert_eq!(s(case_fold('A')), "a");
    assert_eq!(to_uppercase('a').len(), 1); // ExactSizeIterator
}

#[cfg(feature = "latin1")]
#[test]
fn latin1_case() {
    // ß uppercases to SS, titlecases to Ss, folds to ss.
    assert_eq!(s(to_uppercase('ß')), "SS");
    assert_eq!(s(to_titlecase('ß')), "Ss");
    assert_eq!(s(case_fold('ß')), "ss");
    assert_eq!(s(to_lowercase('É')), "é"); // U+00C9 -> U+00E9
    assert_eq!(to_uppercase('ß').len(), 2);
}

#[cfg(feature = "latin1")]
#[test]
fn stream_adaptors() {
    use intl::unicode::{fold, lowercase, uppercase};
    assert_eq!(s(uppercase("Weiß".chars())), "WEISS");
    assert_eq!(s(lowercase("HÉLLO".chars())), "héllo");
    // Caseless comparison via folding.
    assert!(fold("STRASSE".chars()).eq(fold("strasse".chars())));
    assert!(!fold("straße".chars()).eq(fold("strasches".chars())));
}

#[cfg(feature = "bmp")]
#[test]
fn bmp_case() {
    assert_eq!(s(to_uppercase('ﬀ')), "FF"); // U+FB00 ligature
    assert_eq!(s(to_lowercase('Σ')), "σ"); // U+03A3 -> U+03C3
    // U+212A KELVIN SIGN folds to plain ASCII 'k', enabling caseless match.
    assert_eq!(s(case_fold('\u{212A}')), "k");
    assert_eq!(s(case_fold('K')), s(case_fold('\u{212A}')));
}

#[cfg(feature = "alloc")]
#[test]
fn title_casing() {
    use intl::unicode::titlecase;
    assert_eq!(titlecase("loud HOUSE"), "Loud House");
    assert_eq!(titlecase("can't stop"), "Can't Stop");
    assert_eq!(titlecase("ﬂour"), "Flour"); // ﬂ ligature title-cases to "Fl"
}
