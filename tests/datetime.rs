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
