#![cfg(feature = "normalization")]
//! Readable normalization spot-checks. The exhaustive UAX #15 conformance run
//! lives in `normalization_conformance.rs` (full tier).

use intl::unicode::canonical_combining_class as ccc;
#[cfg(feature = "bmp")]
use intl::unicode::{nfc, nfd, nfkc, nfkd};

#[cfg(feature = "bmp")]
fn s(it: impl Iterator<Item = char>) -> String {
    it.collect()
}

#[test]
fn ccc_values() {
    assert_eq!(ccc('a'), 0);
    assert_eq!(ccc('A'), 0);
}

#[cfg(feature = "bmp")]
#[test]
fn ccc_combining() {
    assert_eq!(ccc('\u{0301}'), 230); // combining acute accent
    assert_eq!(ccc('\u{0323}'), 220); // combining dot below
}

#[cfg(feature = "bmp")]
#[test]
fn canonical_round_trip() {
    assert_eq!(s(nfd("é".chars())), "e\u{0301}");
    assert_eq!(s(nfc("e\u{0301}".chars())), "é");
    // Idempotence.
    assert_eq!(s(nfc(nfc("é".chars()))), "é");
}

#[cfg(feature = "bmp")]
#[test]
fn canonical_ordering() {
    // q + dot-below (ccc 220) + acute (ccc 230) is already ordered; the reverse
    // input must reorder to the same NFD.
    let ordered = "q\u{0323}\u{0301}";
    assert_eq!(s(nfd("q\u{0301}\u{0323}".chars())), ordered);
    assert_eq!(s(nfc("q\u{0301}\u{0323}".chars())), ordered);
}

#[cfg(feature = "bmp")]
#[test]
fn hangul() {
    // 각 U+AC01 = ᄀ U+1100 + ᅡ U+1161 + ᆨ U+11A8
    assert_eq!(s(nfd("\u{AC01}".chars())), "\u{1100}\u{1161}\u{11A8}");
    assert_eq!(s(nfc("\u{1100}\u{1161}\u{11A8}".chars())), "\u{AC01}");
}

#[cfg(feature = "bmp")]
#[test]
fn quick_check() {
    use intl::unicode::{IsNormalized, is_nfc, is_nfd, quick_check_nfc};
    assert!(is_nfc("é".chars())); // precomposed
    assert!(!is_nfc("e\u{0301}".chars())); // decomposed -> not NFC
    assert!(is_nfd("e\u{0301}".chars()));
    assert!(!is_nfd("é".chars()));
    // A combining mark makes NFC inconclusive without a full check.
    assert_eq!(quick_check_nfc("e\u{0301}".chars()), IsNormalized::Maybe);
    assert_eq!(quick_check_nfc("abc".chars()), IsNormalized::Yes);
}

#[cfg(feature = "bmp")]
#[test]
fn compatibility() {
    assert_eq!(s(nfkd("\u{FB01}".chars())), "fi"); // ﬁ ligature
    assert_eq!(s(nfkc("\u{FB01}".chars())), "fi");
    assert_eq!(s(nfkc("\u{00B2}".chars())), "2"); // superscript two
    // NFC leaves the ligature intact (compatibility-only decomposition).
    assert_eq!(s(nfc("\u{FB01}".chars())), "\u{FB01}");
}

#[cfg(feature = "bmp")]
#[test]
fn raw_decomposition() {
    use intl::unicode::NormalizationForm::{Nfc, Nfd, Nfkc, Nfkd};
    use intl::unicode::raw_decomposition as raw;
    let get = |c, f| raw(c, f).map(|d| d.to_vec());

    // php-src normalizer_get_raw_decomposition.phpt: <narrow> 3161, one step
    // (the full decomposition is U+1173).
    assert_eq!(get('\u{FFDA}', Nfkc), Some(vec!['\u{3161}']));
    assert_eq!(get('\u{FFDA}', Nfkd), Some(vec!['\u{3161}']));
    assert_eq!(get('\u{FFDA}', Nfc), None);
    assert_eq!(get('\u{FFDA}', Nfd), None);

    // Canonical mappings count for every form.
    for f in [Nfc, Nfd, Nfkc, Nfkd] {
        assert_eq!(get('\u{00C5}', f), Some(vec!['A', '\u{030A}']));
        // ANGSTROM SIGN: singleton to U+00C5, not A + ring.
        assert_eq!(get('\u{212B}', f), Some(vec!['\u{00C5}']));
        // LATIN SMALL LETTER LONG S WITH DOT ABOVE: the canonical raw mapping,
        // even under NFKC whose full decomposition is s + dot above.
        assert_eq!(get('\u{1E9B}', f), Some(vec!['\u{017F}', '\u{0307}']));
        // Hangul (algorithmic, as ICU): LVT -> LV + T, LV -> L + V.
        assert_eq!(get('\u{D4DB}', f), Some(vec!['\u{D4CC}', '\u{11B6}']));
        assert_eq!(get('\u{AC00}', f), Some(vec!['\u{1100}', '\u{1161}']));
        // No mapping.
        assert_eq!(get('a', f), None);
        assert_eq!(get('\u{4E2D}', f), None);
    }

    // Compatibility mapping that decomposes further: U+01C4 DŽ -> D + U+017D
    // in one step, not D + Z + caron.
    assert_eq!(get('\u{01C4}', Nfkd), Some(vec!['D', '\u{017D}']));
    assert_eq!(get('\u{01C4}', Nfd), None);
    // ﬃ ligature: <compat> f f i.
    assert_eq!(get('\u{FB03}', Nfkc), Some(vec!['f', 'f', 'i']));
    // Longest single-step mapping (18 code points).
    assert_eq!(raw('\u{FDFA}', Nfkc).unwrap().len(), 18);
}
