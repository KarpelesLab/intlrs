//! Locale-aware relative time formatting.
#![cfg(feature = "alloc")]

use intl::relative::{format_relative as fr, RelativeUnit::*};

#[test]
fn english_relative() {
    assert_eq!(fr("en", -1, Day), "yesterday");
    assert_eq!(fr("en", 0, Day), "today");
    assert_eq!(fr("en", 1, Day), "tomorrow");
    assert_eq!(fr("en", 3, Day), "in 3 days");
    assert_eq!(fr("en", -2, Hour), "2 hours ago");
    assert_eq!(fr("en", 1, Minute), "in 1 minute");
    assert_eq!(fr("en", 5, Year), "in 5 years");
}

#[test]
fn other_locales() {
    assert_eq!(fr("de", -1, Day), "gestern");
    assert_eq!(fr("fr", 3, Day), "dans 3 jours");
    assert_eq!(fr("es", -2, Hour), "hace 2 horas");
    // Polish uses few/many plural forms for relative time.
    assert!(!fr("pl", 5, Day).is_empty());
}
