//! Date/time formatting.
#![cfg(feature = "alloc")]
use intl::datetime::{
    format_date as fd, format_datetime as fdt, format_time as ft, DateStyle::*, DateTime,
};

const DT: DateTime = DateTime {
    year: 2026,
    month: 6,
    day: 4,
    hour: 14,
    minute: 30,
    second: 5,
};

#[test]
fn dates() {
    assert_eq!(fd("en", &DT, Full), "Thursday, June 4, 2026");
    assert_eq!(fd("en", &DT, Long), "June 4, 2026");
    assert_eq!(fd("en", &DT, Medium), "Jun 4, 2026");
    assert_eq!(fd("en", &DT, Short), "6/4/26");
    assert_eq!(fd("de", &DT, Long), "4. Juni 2026");
    assert_eq!(fd("fr", &DT, Long), "4 juin 2026");
}

#[test]
fn times_and_combined() {
    assert_eq!(ft("en", &DT, Short), "2:30\u{202f}PM");
    assert_eq!(ft("en", &DT, Medium), "2:30:05\u{202f}PM");
    assert_eq!(ft("de", &DT, Short), "14:30");
    // Combined date+time.
    let c = fdt("en", &DT, Medium, Short);
    assert!(c.contains("Jun 4, 2026") && c.contains("2:30"));
}

#[test]
fn weekday_correct() {
    // 2026-06-04 is a Thursday.
    assert!(fd("en", &DT, Full).starts_with("Thursday"));
    // 2000-01-01 was a Saturday.
    let y2k = DateTime {
        year: 2000,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
    };
    assert!(fd("en", &y2k, Full).starts_with("Saturday"));
}

#[test]
fn skeletons() {
    use intl::datetime::format_skeleton as fs;
    assert_eq!(fs("en", &DT, "yMMMd"), "Jun 4, 2026");
    assert_eq!(fs("en", &DT, "MMMMd"), "June 4");
    assert_eq!(fs("en", &DT, "Hm"), "14:30");
    assert_eq!(fs("en", &DT, "yM"), "6/2026");
    assert_eq!(fs("de", &DT, "yMMMd"), "4. Juni 2026");
    assert_eq!(fs("fr", &DT, "MMMd"), "4 juin");
}

#[test]
fn iso8601() {
    assert_eq!(DT.to_iso8601(), "2026-06-04T14:30:05");
    assert_eq!(DateTime::parse_iso8601("2026-06-04T14:30:05"), Some(DT));
    assert_eq!(DateTime::parse_iso8601("2026-06-04 14:30:05"), Some(DT)); // space
    assert_eq!(DateTime::parse_iso8601("2026-06-04T14:30:05Z"), Some(DT)); // Z
                                                                           // Omitted seconds / time default to zero.
    let midnight = DateTime {
        year: 2026,
        month: 6,
        day: 4,
        hour: 0,
        minute: 0,
        second: 0,
    };
    assert_eq!(DateTime::parse_iso8601("2026-06-04"), Some(midnight));
    assert_eq!(DateTime::parse_iso8601("2026-06-04T00:00"), Some(midnight));
    // Round-trip.
    assert_eq!(DateTime::parse_iso8601(&DT.to_iso8601()), Some(DT));
    // Malformed.
    assert_eq!(DateTime::parse_iso8601("not-a-date"), None);
    assert_eq!(DateTime::parse_iso8601("2026-13-01"), None); // bad month
}

#[test]
fn gmt_offset() {
    use intl::datetime::format_gmt_offset as g;
    assert_eq!(g("en", 0), "GMT");
    assert_eq!(g("en", 330), "GMT+05:30"); // India
    assert_eq!(g("en", -480), "GMT-08:00"); // US Pacific
    assert_eq!(g("fr", 0), "UTC");
    assert_eq!(g("fr", -480), "UTC\u{2212}08:00"); // French uses UTC + minus sign
    assert_eq!(g("en", 60), "GMT+01:00");
}

#[test]
fn islamic_dates() {
    use intl::datetime::{format_islamic_date as fi, DateStyle::*};
    // 9 Ramadan 1445 AH (Ramadan = month 9).
    assert_eq!(fi("en", 1445, 9, 1, Long), "Ramadan 1, 1445 AH");
    assert_eq!(fi("en", 1445, 1, 10, Medium), "Muh. 10, 1445 AH");
    // The corresponding Gregorian date for the year/era are localized.
    assert!(fi("en", 1446, 1, 1, Full).contains("Muharram"));
    assert!(fi("fr", 1445, 9, 1, Long).contains("1445"));
}
