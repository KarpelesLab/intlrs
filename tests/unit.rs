//! Measurement-unit formatting.
#![cfg(feature = "alloc")]
use intl::unit::{Unit::*, UnitWidth::*, format_unit as fu};

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

#[test]
fn durations() {
    use intl::unit::{UnitWidth::*, format_duration as fd};
    assert_eq!(fd("en", 3661, Long), "1 hour 1 minute 1 second");
    assert_eq!(fd("en", 90, Long), "1 minute 30 seconds");
    assert_eq!(fd("en", 90, Short), "1 min 30 sec");
    assert_eq!(fd("en", 0, Long), "0 seconds");
    assert_eq!(fd("en", 86400 + 3600, Long), "1 day 1 hour");
    assert_eq!(fd("en", -120, Long), "-2 minutes");
    // Localized: German wording + number.
    assert!(fd("de", 3661, Long).contains("Stunde"));
}
