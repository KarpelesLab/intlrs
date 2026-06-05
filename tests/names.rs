//! Full character Name database (the `names` feature).
#![cfg(feature = "names")]
use intl::unicode::name;

#[test]
fn tabulated_names() {
    assert_eq!(name('A').as_deref(), Some("LATIN CAPITAL LETTER A"));
    assert_eq!(name('a').as_deref(), Some("LATIN SMALL LETTER A"));
    assert_eq!(name('€').as_deref(), Some("EURO SIGN"));
    assert_eq!(
        name('ñ').as_deref(),
        Some("LATIN SMALL LETTER N WITH TILDE")
    );
    assert_eq!(name('!').as_deref(), Some("EXCLAMATION MARK"));
    assert_eq!(name('\u{05D0}').as_deref(), Some("HEBREW LETTER ALEF"));
    // Algorithmic names still work through name().
    assert_eq!(name('한').as_deref(), Some("HANGUL SYLLABLE HAN"));
    assert_eq!(name('一').as_deref(), Some("CJK UNIFIED IDEOGRAPH-4E00"));
    // Unnamed: control char, unassigned.
    assert_eq!(name('\u{0000}'), None);
    assert_eq!(name('\u{0378}'), None);
}
