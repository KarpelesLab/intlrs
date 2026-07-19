//! Date/time interval formatting (`formatRange` / `formatRangeToParts`).
//!
//! Expected values cross-checked against `Intl.DateTimeFormat.prototype.formatRange`
//! (V8 / Node) with the vendored CLDR 48 data. The `–` separators are the CLDR
//! thin-space + en-dash (`\u{2009}\u{2013}\u{2009}`).
#![cfg(feature = "datetime")]

use intl::datetime::{
    DateTime, DateTimeFormatOptions, DateTimePartType, MonthStyle, Numeric2Digit, RangeSource,
    format_range, format_range_to_parts,
};

const EPOCH: DateTime = DateTime {
    year: 1970,
    month: 1,
    day: 1,
    hour: 0,
    minute: 0,
    second: 0,
    millisecond: 0,
};

fn ymd(y: i32, m: u8, d: u8) -> DateTime {
    DateTime {
        year: y,
        month: m,
        day: d,
        ..EPOCH
    }
}

/// `{year:'numeric', month:'short', day:'numeric'}` → skeleton `yMMMd`.
fn ymmmd() -> DateTimeFormatOptions {
    let mut o = DateTimeFormatOptions::default();
    o.year = Some(Numeric2Digit::Numeric);
    o.month = Some(MonthStyle::Short);
    o.day = Some(Numeric2Digit::Numeric);
    o
}

const DASH: &str = "\u{2009}\u{2013}\u{2009}"; // thin space + en dash + thin space

#[test]
fn same_month_different_day() {
    let out = format_range("en", &ymd(2024, 1, 1), &ymd(2024, 1, 5), &ymmmd()).unwrap();
    assert_eq!(out, format!("Jan 1{DASH}5, 2024"));
}

#[test]
fn different_month() {
    let out = format_range("en", &ymd(2024, 1, 1), &ymd(2024, 2, 5), &ymmmd()).unwrap();
    assert_eq!(out, format!("Jan 1{DASH}Feb 5, 2024"));
}

#[test]
fn different_year() {
    let out = format_range("en", &ymd(2024, 1, 1), &ymd(2025, 2, 5), &ymmmd()).unwrap();
    assert_eq!(out, format!("Jan 1, 2024{DASH}Feb 5, 2025"));
}

#[test]
fn identical_is_single() {
    let d = ymd(2024, 1, 1);
    let out = format_range("en", &d, &d, &ymmmd()).unwrap();
    assert_eq!(out, "Jan 1, 2024");
    // No range separator for a single value.
    assert!(!out.contains('\u{2013}'));
}

#[test]
fn differ_only_in_omitted_field_is_single() {
    // The skeleton has no time fields, so an hour-only difference collapses to a
    // single formatted date.
    let a = DateTime {
        hour: 9,
        ..ymd(2024, 1, 1)
    };
    let b = DateTime {
        hour: 17,
        ..ymd(2024, 1, 1)
    };
    assert_eq!(format_range("en", &a, &b, &ymmmd()).unwrap(), "Jan 1, 2024");
}

#[test]
fn localized_fr() {
    // fr yMMMd 'd' pattern = "d–d MMM y" → "1–5 janv. 2024".
    let out = format_range("fr", &ymd(2024, 1, 1), &ymd(2024, 1, 5), &ymmmd()).unwrap();
    assert_eq!(out, "1\u{2013}5 janv. 2024");
}

#[test]
fn localized_ja() {
    // ja uses the '～' fallback-style separator embedded in its interval patterns.
    let out = format_range("ja", &ymd(2024, 1, 1), &ymd(2024, 1, 5), &ymmmd()).unwrap();
    assert_eq!(out, "2024年1月1日～5日");
}

#[test]
fn parts_sources_same_month_different_day() {
    let parts = format_range_to_parts("en", &ymd(2024, 1, 1), &ymd(2024, 1, 5), &ymmmd()).unwrap();
    // Joined value matches format_range.
    let joined: String = parts.iter().map(|p| p.value.as_str()).collect();
    assert_eq!(joined, format!("Jan 1{DASH}5, 2024"));

    use DateTimePartType::*;
    use RangeSource::*;
    let got: Vec<(DateTimePartType, &str, RangeSource)> = parts
        .iter()
        .map(|p| (p.kind, p.value.as_str(), p.source))
        .collect();
    assert_eq!(
        got,
        vec![
            (Month, "Jan", Shared),
            (Literal, " ", Shared),
            (Day, "1", StartRange),
            (Literal, DASH, Shared),
            (Day, "5", EndRange),
            (Literal, ", ", Shared),
            (Year, "2024", Shared),
        ]
    );
}

#[test]
fn parts_sources_different_month() {
    let parts = format_range_to_parts("en", &ymd(2024, 1, 1), &ymd(2024, 2, 5), &ymmmd()).unwrap();
    use DateTimePartType::*;
    use RangeSource::*;
    // The greatest difference is the month: month/day of each side are start/end,
    // the shared year and the boundary separator are shared.
    let sources: Vec<RangeSource> = parts.iter().map(|p| p.source).collect();
    assert_eq!(
        sources,
        vec![
            StartRange, // month Jan
            StartRange, // " "
            StartRange, // day 1
            Shared,     // " – "
            EndRange,   // month Feb
            EndRange,   // " "
            EndRange,   // day 5
            Shared,     // ", "
            Shared,     // year 2024
        ]
    );
    // Sanity: last part is the shared year.
    let last = parts.last().unwrap();
    assert_eq!((last.kind, last.source), (Year, Shared));
}
