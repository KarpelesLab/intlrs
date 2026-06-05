//! UTS #46 / Punycode round-trips.
#![cfg(feature = "idna")]

use intl::unicode::idna::{to_ascii, to_unicode};

#[test]
fn ascii_roundtrip() {
    assert_eq!(to_ascii("example.com").unwrap(), "example.com");
    assert_eq!(to_ascii("Bücher.example").unwrap(), "xn--bcher-kva.example");
    assert_eq!(to_ascii("日本語.jp").unwrap(), "xn--wgv71a119e.jp");
    assert_eq!(to_ascii("faß.de").unwrap(), "xn--fa-hia.de"); // nontransitional keeps ß
                                                              // Mapping: uppercase folds, fullwidth maps.
    assert_eq!(to_ascii("EXAMPLE.COM").unwrap(), "example.com");
}

#[test]
fn unicode_roundtrip() {
    assert_eq!(
        to_unicode("xn--bcher-kva.example").unwrap(),
        "bücher.example"
    );
    assert_eq!(to_unicode("xn--wgv71a119e.jp").unwrap(), "日本語.jp");
    assert_eq!(to_unicode("example.com").unwrap(), "example.com");
}

/// A single dot-free label of 100_000 distinct non-ASCII code points must be
/// rejected *before* the quadratic Punycode encoder runs. The encoder is
/// O(distinct × len), so without the early code-point cap this label would force
/// ~10^10 inner-loop iterations plus a large interim allocation. The cap makes
/// `to_ascii` reject it in linear time — the fact that this test completes
/// near-instantly is the assertion that the cap fires up front.
#[test]
fn encode_does_not_blow_up_on_huge_label() {
    // 100_000 distinct non-ASCII scalars starting past the surrogate range, so
    // every value is a valid scalar and the work lands on the encode path.
    let label: String = (0xE000u32..0xE000 + 100_000)
        .filter_map(char::from_u32)
        .collect();
    assert_eq!(label.chars().count(), 100_000);
    // Returns Err (label far exceeds the 63-octet A-label limit), quickly.
    assert!(to_ascii(&label).is_err());
}

/// The early cap must not reject any label that would have produced a valid
/// (≤63-octet) A-label. 63 code points is the correct permissive bound.
#[test]
fn long_but_valid_labels_still_succeed() {
    // Longest valid ASCII DNS label: 63 octets, passes through unchanged.
    let ascii63 = "a".repeat(63);
    assert_eq!(to_ascii(&ascii63).unwrap(), ascii63);

    // A realistic IDN that encodes to a valid A-label still round-trips.
    assert_eq!(to_ascii("日本語.jp").unwrap(), "xn--wgv71a119e.jp");

    // A non-ASCII label of exactly 63 code points is at the cap boundary; it may
    // legitimately fail the 63-octet A-label check, but must not panic or hang.
    let label63: String = (0x4E00u32..0x4E00 + 63)
        .filter_map(char::from_u32)
        .collect();
    assert_eq!(label63.chars().count(), 63);
    let _ = to_ascii(&label63);
}
