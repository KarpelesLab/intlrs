//! Date/time formatting.
#![cfg(feature = "alloc")]
use intl::datetime::{
    DateStyle::*, DateTime, format_date as fd, format_datetime as fdt, format_time as ft,
};

const DT: DateTime = DateTime {
    year: 2026,
    month: 6,
    day: 4,
    hour: 14,
    minute: 30,
    second: 5,
    millisecond: 0,
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
        millisecond: 0,
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
        millisecond: 0,
    };
    assert_eq!(DateTime::parse_iso8601("2026-06-04"), Some(midnight));
    assert_eq!(DateTime::parse_iso8601("2026-06-04T00:00"), Some(midnight));
    // Round-trip.
    assert_eq!(DateTime::parse_iso8601(&DT.to_iso8601()), Some(DT));
    // Malformed.
    assert_eq!(DateTime::parse_iso8601("not-a-date"), None);
    assert_eq!(DateTime::parse_iso8601("2026-13-01"), None); // bad month
    // Fractional seconds round-trip through millisecond precision.
    let ms = DateTime {
        millisecond: 250,
        ..DT
    };
    assert_eq!(ms.to_iso8601(), "2026-06-04T14:30:05.250");
    assert_eq!(DateTime::parse_iso8601("2026-06-04T14:30:05.250"), Some(ms));
    // Fewer fractional digits scale to milliseconds; extra digits truncate.
    assert_eq!(
        DateTime::parse_iso8601("2026-06-04T14:30:05.5")
            .unwrap()
            .millisecond,
        500
    );
    assert_eq!(
        DateTime::parse_iso8601("2026-06-04T14:30:05.05")
            .unwrap()
            .millisecond,
        50
    );
    assert_eq!(
        DateTime::parse_iso8601("2026-06-04T14:30:05.123456")
            .unwrap()
            .millisecond,
        123
    );
    // A zero millisecond omits the fraction (byte-identical to before).
    assert_eq!(DT.to_iso8601(), "2026-06-04T14:30:05");
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
fn gmt_offset_extremes() {
    use intl::datetime::format_gmt_offset as g;
    // i32::MIN previously panicked via `.abs()` overflow; must now return a string.
    let s = g("en", i32::MIN);
    assert!(s.starts_with("GMT-"), "got {s:?}");
    assert!(!s.is_empty());
    // i32::MAX must also be handled without panic.
    assert!(g("en", i32::MAX).starts_with("GMT+"));
    // Normal inputs remain byte-for-byte identical.
    assert_eq!(g("en", 0), "GMT");
    assert_eq!(g("en", 330), "GMT+05:30");
    assert_eq!(g("en", -480), "GMT-08:00");
}

#[test]
fn islamic_dates() {
    use intl::datetime::{DateStyle::*, format_islamic_date as fi};
    // 9 Ramadan 1445 AH (Ramadan = month 9).
    assert_eq!(fi("en", 1445, 9, 1, Long), "Ramadan 1, 1445 AH");
    assert_eq!(fi("en", 1445, 1, 10, Medium), "Muh. 10, 1445 AH");
    // The corresponding Gregorian date for the year/era are localized.
    assert!(fi("en", 1446, 1, 1, Full).contains("Muharram"));
    assert!(fi("fr", 1445, 9, 1, Long).contains("1445"));
}

#[test]
fn arithmetic() {
    // Weekday (2026-06-04 is a Thursday = 4).
    assert_eq!(DT.weekday(), 4);
    // Add across a year boundary.
    let nye = DateTime {
        year: 2026,
        month: 12,
        day: 31,
        hour: 23,
        minute: 59,
        second: 30,
        millisecond: 0,
    };
    assert_eq!(
        nye.add_seconds(90),
        DateTime {
            year: 2027,
            month: 1,
            day: 1,
            hour: 0,
            minute: 1,
            second: 0,
            millisecond: 0,
        }
    );
    // Subtract a day, leap-year aware (2024 is leap, so day before Mar 1 is Feb 29).
    let mar1 = DateTime {
        year: 2024,
        month: 3,
        day: 1,
        hour: 12,
        minute: 0,
        second: 0,
        millisecond: 0,
    };
    assert_eq!(mar1.add_days(-1).day, 29);
    assert_eq!(mar1.add_days(-1).month, 2);
    // Round-trip.
    assert_eq!(DT.add_seconds(12345).add_seconds(-12345), DT);
}

#[test]
fn persian_dates() {
    use intl::datetime::{DateStyle::*, format_persian_date as fp};
    assert_eq!(fp("en", 1404, 1, 1, Long), "Farvardin 1, 1404 AP");
    assert_eq!(fp("en", 1403, 12, 30, Medium), "Esfand 30, 1403 AP");
    assert!(fp("fr", 1404, 1, 1, Long).contains("1404"));
}

#[test]
fn component_options() {
    use intl::datetime::{
        DateTimeFormatError, DateTimeFormatOptions, DateTimePartType, HourCycle, MonthStyle,
        NameStyle, Numeric2Digit, TimeZoneNameStyle, format_options as fo, format_to_parts as ftp,
    };
    let n = Some(Numeric2Digit::Numeric);
    let td = Some(Numeric2Digit::TwoDigit);

    // year/month/day numeric + short month.
    let o = DateTimeFormatOptions {
        year: n,
        month: Some(MonthStyle::Short),
        day: n,
        ..Default::default()
    };
    assert_eq!(fo("en", &DT, &o).unwrap(), "Jun 4, 2026");
    let kinds: Vec<_> = ftp("en", &DT, &o)
        .unwrap()
        .iter()
        .map(|p| p.kind.as_str().to_string())
        .collect();
    assert_eq!(kinds, ["month", "literal", "day", "literal", "year"]);

    // 24-hour time.
    let t = DateTimeFormatOptions {
        hour: n,
        minute: td,
        hour_cycle: Some(HourCycle::H23),
        ..Default::default()
    };
    assert_eq!(fo("en", &DT, &t).unwrap(), "14:30");

    // 12-hour time.
    let t12 = DateTimeFormatOptions {
        hour: n,
        minute: td,
        hour12: Some(true),
        ..Default::default()
    };
    assert_eq!(fo("en", &DT, &t12).unwrap(), "2:30\u{202f}PM");

    // dateStyle shortcut.
    let ds = DateTimeFormatOptions {
        date_style: Some(Long),
        ..Default::default()
    };
    assert_eq!(fo("en", &DT, &ds).unwrap(), "June 4, 2026");

    // Conflicting options.
    let bad = DateTimeFormatOptions {
        date_style: Some(Long),
        year: n,
        ..Default::default()
    };
    assert_eq!(
        fo("en", &DT, &bad),
        Err(DateTimeFormatError::ConflictingOptions)
    );

    // Narrow month (asserted on the part, robust to surrounding fields).
    let narrow = DateTimeFormatOptions {
        month: Some(MonthStyle::Narrow),
        day: n,
        ..Default::default()
    };
    let parts = ftp("en", &DT, &narrow).unwrap();
    let mon = parts
        .iter()
        .find(|p| p.kind == DateTimePartType::Month)
        .unwrap();
    assert_eq!(mon.value, "J");

    // Era + narrow weekday via skeleton/field wiring.
    assert_eq!(
        intl::datetime::format_skeleton("en", &DT, "GyMMMd"),
        "Jun 4, 2026 AD"
    );

    // Fractional seconds.
    let frac = DateTimeFormatOptions {
        hour: n,
        minute: td,
        second: td,
        fractional_second_digits: Some(3),
        hour_cycle: Some(HourCycle::H23),
        ..Default::default()
    };
    let ms = DateTime {
        millisecond: 50,
        ..DT
    };
    assert_eq!(fo("en", &ms, &frac).unwrap(), "14:30:05.050");

    // timeZoneName offset.
    let tz = DateTimeFormatOptions {
        hour: n,
        minute: td,
        hour_cycle: Some(HourCycle::H23),
        time_zone_name: Some(TimeZoneNameStyle::LongOffset),
        tz_offset_minutes: Some(-480),
        ..Default::default()
    };
    let parts = ftp("en", &DT, &tz).unwrap();
    assert_eq!(parts.last().unwrap().kind, DateTimePartType::TimeZoneName);
    assert_eq!(parts.last().unwrap().value, "GMT-08:00");

    // Default (no options) → numeric y/M/d.
    assert_eq!(
        fo("en", &DT, &DateTimeFormatOptions::default()).unwrap(),
        "6/4/2026"
    );

    // weekday:Narrow part value.
    let wd = DateTimeFormatOptions {
        weekday: Some(NameStyle::Narrow),
        ..Default::default()
    };
    let parts = ftp("en", &DT, &wd).unwrap();
    assert_eq!(parts[0].value, "T"); // Thursday narrow
}

#[test]
fn component_locale_defaults_and_field_keep() {
    use intl::datetime::{
        DateTimeFormatOptions, MonthStyle, NameStyle, Numeric2Digit, format_options as fo,
    };
    let n = Some(Numeric2Digit::Numeric);
    let td = Some(Numeric2Digit::TwoDigit);

    // Default hour cycle is derived from the locale's CLDR time pattern:
    // en-US is 12-hour, de is 24-hour (no explicit hourCycle/hour12).
    let hm = DateTimeFormatOptions {
        hour: n,
        minute: td,
        ..Default::default()
    };
    assert_eq!(fo("en", &DT, &hm).unwrap(), "2:30\u{202f}PM");
    assert_eq!(fo("de", &DT, &hm).unwrap(), "14:30");

    // Weekday must survive when combined with a wide month + day (the exact
    // skeleton MMMMEd is absent, but MMMEd matches and the width is patched).
    let wmd = DateTimeFormatOptions {
        weekday: Some(NameStyle::Long),
        month: Some(MonthStyle::Long),
        day: n,
        ..Default::default()
    };
    assert_eq!(fo("en", &DT, &wmd).unwrap(), "Thursday, June 4");
}

#[test]
fn hour_cycles() {
    use intl::datetime::{DateTimeFormatOptions, HourCycle, Numeric2Digit, format_options as fo};
    let n = Some(Numeric2Digit::Numeric);
    let td = Some(Numeric2Digit::TwoDigit);
    let at = |h: u8, c: HourCycle| {
        let dt = DateTime {
            hour: h,
            minute: 0,
            ..DT
        };
        fo(
            "en",
            &dt,
            &DateTimeFormatOptions {
                hour: n,
                minute: td,
                hour_cycle: Some(c),
                ..Default::default()
            },
        )
        .unwrap()
    };
    // Midnight (00:00): the four cycles diverge.
    assert_eq!(at(0, HourCycle::H11), "0:00\u{202f}AM");
    assert_eq!(at(0, HourCycle::H12), "12:00\u{202f}AM");
    assert_eq!(at(0, HourCycle::H23), "0:00");
    assert_eq!(at(0, HourCycle::H24), "24:00");
    // Noon (12:00).
    assert_eq!(at(12, HourCycle::H11), "0:00\u{202f}PM");
    assert_eq!(at(12, HourCycle::H12), "12:00\u{202f}PM");
    assert_eq!(at(12, HourCycle::H23), "12:00");
    assert_eq!(at(12, HourCycle::H24), "12:00");
    // Afternoon (13:00).
    assert_eq!(at(13, HourCycle::H11), "1:00\u{202f}PM");
    assert_eq!(at(13, HourCycle::H23), "13:00");
}
