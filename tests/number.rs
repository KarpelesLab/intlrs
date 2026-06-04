//! Locale-aware number formatting.
#![cfg(feature = "alloc")]

use intl::number::{format_decimal as dec, format_percent as pct};

#[test]
fn decimal_grouping_and_separators() {
    assert_eq!(dec("en", 1234567.0), "1,234,567");
    assert_eq!(dec("de", 1234567.0), "1.234.567");
    assert_eq!(dec("fr", 1234567.0), "1\u{202f}234\u{202f}567"); // narrow no-break space
    assert_eq!(dec("hi", 1234567.0), "12,34,567"); // Indian grouping
    assert_eq!(dec("en", 1234.5), "1,234.5");
    assert_eq!(dec("de", 1234.5), "1.234,5");
}

#[test]
fn fraction_and_sign() {
    // Default max 3 fraction digits, trailing zeros trimmed.
    assert_eq!(dec("en", 0.5), "0.5");
    assert_eq!(dec("en", 1.25), "1.25");
    assert_eq!(dec("en", 1.0), "1");
    assert_eq!(dec("en", -1234.5), "-1,234.5");
    // Rounding to 3 fraction digits.
    assert_eq!(dec("en", 1.23456), "1.235");
}

#[test]
fn percent_formatting() {
    assert_eq!(pct("en", 0.5), "50%");
    assert_eq!(pct("de", 0.5), "50\u{a0}%"); // NBSP before %
    assert_eq!(pct("en", 0.1234), "12%"); // 0 fraction digits in the percent pattern
}

#[test]
fn unknown_locale_falls_back() {
    assert_eq!(dec("xx", 1234.5), dec("en", 1234.5));
    assert_eq!(dec("en-US", 1234.5), "1,234.5"); // region falls back to language
}

#[test]
fn currency() {
    use intl::number::format_currency as fc;
    assert_eq!(fc("en", 1234.5, "USD"), "$1,234.50");
    assert_eq!(fc("de", 1234.5, "EUR"), "1.234,50\u{a0}€");
    assert_eq!(fc("ja", 1234.0, "JPY"), "￥1,234"); // 0 fraction digits
    assert_eq!(fc("en", -5.0, "USD"), "-$5.00");
    // Unknown locale falls back; unknown currency uses its code as the symbol.
    assert_eq!(fc("xx", 5.0, "USD"), "$5.00");
    assert!(fc("en", 5.0, "XYZ").contains("XYZ"));
}

#[test]
fn parsing() {
    use intl::number::{format_decimal as f, parse_decimal as p};
    assert_eq!(p("en", "1,234.5"), Some(1234.5));
    assert_eq!(p("de", "1.234,5"), Some(1234.5));
    assert_eq!(p("fr", "1\u{202f}234,5"), Some(1234.5));
    assert_eq!(p("en", "-42"), Some(-42.0));
    assert_eq!(p("hi", "12,34,567"), Some(1234567.0)); // Indian grouping
    assert_eq!(p("en", "abc"), None);
    assert_eq!(p("en", ""), None);
    // Round-trips: format then parse.
    for &(lang, v) in &[("en", 1234567.0_f64), ("de", -98765.43), ("fr", 1000.0)] {
        assert_eq!(p(lang, &f(lang, v)), Some(v));
    }
}
