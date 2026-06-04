//! Readable normalization spot-checks. The exhaustive UAX #15 conformance run
//! lives in `normalization_conformance.rs` (full tier).

use intl::unicode::{canonical_combining_class as ccc, nfc, nfd, nfkc, nfkd};

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
fn compatibility() {
    assert_eq!(s(nfkd("\u{FB01}".chars())), "fi"); // ﬁ ligature
    assert_eq!(s(nfkc("\u{FB01}".chars())), "fi");
    assert_eq!(s(nfkc("\u{00B2}".chars())), "2"); // superscript two
                                                  // NFC leaves the ligature intact (compatibility-only decomposition).
    assert_eq!(s(nfc("\u{FB01}".chars())), "\u{FB01}");
}
