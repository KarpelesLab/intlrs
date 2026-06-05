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
