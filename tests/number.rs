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

#[test]
fn scientific() {
    use intl::number::format_scientific as sci;
    assert_eq!(sci("en", 12345.0, 6), "1.2345E4");
    assert_eq!(sci("en", 1.0, 6), "1E0");
    assert_eq!(sci("en", 1000.0, 6), "1E3");
    assert_eq!(sci("en", -250.0, 6), "-2.5E2");
    assert_eq!(sci("de", 0.00042, 6), "4,2E-4");
    assert_eq!(sci("en", 0.0, 6), "0");
    assert_eq!(sci("en", 6.022e23, 6), "6.022E23");
}

#[test]
fn compact() {
    use intl::number::format_compact as k;
    assert_eq!(k("en", 999.0), "999");
    assert_eq!(k("en", 1500.0), "1.5K");
    assert_eq!(k("en", 15000.0), "15K");
    assert_eq!(k("en", 150000.0), "150K");
    assert_eq!(k("en", 2_300_000.0), "2.3M");
    assert_eq!(k("en", 1_000_000_000.0), "1B");
    assert_eq!(k("de", 1500.0), "1.500"); // German doesn't abbreviate thousands
    assert_eq!(k("fr", 1500.0), "1,5\u{a0}k"); // French: NBSP + lowercase k
}

#[test]
fn native_digits() {
    use intl::number::{format_decimal_native as fdn, to_numbering_system as tns};
    assert_eq!(tns("2024", "arab"), "٢٠٢٤");
    assert_eq!(tns("3.14", "deva"), "३.१४");
    assert_eq!(tns("123", "latn"), "123");
    assert_eq!(tns("123", "unknown"), "123");
    // Persian defaults to Extended Arabic-Indic digits.
    assert_eq!(
        fdn("fa", 1234.0),
        tns(&intl::number::format_decimal("fa", 1234.0), "arabext")
    );
    // English stays Latin.
    assert_eq!(fdn("en", 1234.5), "1,234.5");
}

#[test]
fn ordinals() {
    use intl::number::format_ordinal as o;
    assert_eq!(o("en", 1), "1st");
    assert_eq!(o("en", 2), "2nd");
    assert_eq!(o("en", 3), "3rd");
    assert_eq!(o("en", 4), "4th");
    assert_eq!(o("en", 11), "11th");
    assert_eq!(o("en", 21), "21st");
    assert_eq!(o("en", 102), "102nd");
    assert_eq!(o("fr", 1), "1er");
    assert_eq!(o("fr", 2), "2e");
    assert_eq!(o("de", 2), "2."); // period convention
    assert_eq!(o("sv", 1), "1:a");
}

#[test]
fn compact_non_finite() {
    use intl::number::format_compact as k;
    // Non-finite values must not panic (NaN < 1000.0 is false).
    let _ = k("en", f64::NAN);
    let _ = k("en", f64::INFINITY);
    let _ = k("en", f64::NEG_INFINITY);
}

#[test]
fn unit_style() {
    use intl::number::{
        NumberFormatOptions, NumberPartType, NumberStyle, UnitDisplay, format, format_to_parts,
    };
    let mk = |unit, disp| NumberFormatOptions {
        style: NumberStyle::Unit,
        unit: Some(unit),
        unit_display: disp,
        ..Default::default()
    };
    assert_eq!(
        format("en", 5.0, &mk("kilometer", UnitDisplay::Long)),
        "5 kilometers"
    );
    assert_eq!(
        format("en", 1.0, &mk("kilometer", UnitDisplay::Long)),
        "1 kilometer"
    );
    assert_eq!(format("en", 3.0, &mk("hour", UnitDisplay::Short)), "3 hr");
    assert_eq!(
        format("en", 5.0, &mk("kilometer-per-hour", UnitDisplay::Short)),
        "5 km/h"
    );
    assert_eq!(
        format("de", 2.0, &mk("hour", UnitDisplay::Long)),
        "2 Stunden"
    );
    // Parts: number core, then a literal space and the unit.
    let parts = format_to_parts("en", 1.5, &mk("meter", UnitDisplay::Short));
    assert_eq!(parts.last().unwrap().kind, NumberPartType::Unit);
    assert_eq!(parts.last().unwrap().value, "m");
    // Unknown unit degrades to the bare number.
    assert_eq!(format("en", 5.0, &mk("furlong", UnitDisplay::Long)), "5");
}
