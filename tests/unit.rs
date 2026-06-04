//! Measurement-unit formatting.
#![cfg(feature = "alloc")]
use intl::unit::{format_unit as fu, Unit::*, UnitWidth::*};

#[test]
fn units() {
    assert_eq!(fu("en", 5.0, Kilometer, Long), "5 kilometers");
    assert_eq!(fu("en", 1.0, Kilometer, Long), "1 kilometer");
    assert_eq!(fu("en", 3.0, Hour, Short), "3 hr");
    assert_eq!(fu("en", 1.0, Hour, Long), "1 hour");
    assert_eq!(fu("de", 2.0, Hour, Long), "2 Stunden");
    assert_eq!(fu("fr", 5.0, Meter, Long), "5\u{a0}mètres"); // NBSP in French
    assert_eq!(fu("en", 2.5, Gigabyte, Long), "2.5 gigabytes");
    // Locale fallback to English for unknown locale.
    assert_eq!(fu("xx", 5.0, Kilometer, Long), "5 kilometers");
}
