//! Date/time formatting.
#![cfg(feature = "datetime")]
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

/// Build options from `Default` (the struct is `#[non_exhaustive]`).
fn dtf(
    build: impl FnOnce(&mut intl::datetime::DateTimeFormatOptions),
) -> intl::datetime::DateTimeFormatOptions {
    let mut o = intl::datetime::DateTimeFormatOptions::default();
    build(&mut o);
    o
}

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

#[cfg(feature = "calendars-extra")]
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

#[cfg(feature = "calendars-extra")]
#[test]
fn islamic_umalqura_dates() {
    use intl::datetime::{DateStyle::*, format_islamic_umalqura_date as fu};
    // Same localized month names and era ("AH") as the civil formatter.
    let s = fu("en", 1445, 9, 1, Long);
    assert!(s.contains("Ramadan") && s.contains("1445"), "{s}");
    assert!(fu("en", 1446, 1, 1, Full).contains("Muharram"));
    assert!(fu("fr", 1445, 9, 1, Long).contains("1445"));
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

#[cfg(feature = "calendars-extra")]
#[test]
fn persian_dates() {
    use intl::datetime::{DateStyle::*, format_persian_date as fp};
    assert_eq!(fp("en", 1404, 1, 1, Long), "Farvardin 1, 1404 AP");
    assert_eq!(fp("en", 1403, 12, 30, Medium), "Esfand 30, 1403 AP");
    assert!(fp("fr", 1404, 1, 1, Long).contains("1404"));
}

#[cfg(feature = "calendars-extra")]
#[test]
fn chinese_dates() {
    use intl::datetime::{DateStyle::*, format_chinese_date as fc};

    // 2024-02-10 was Chinese new year: year 2024, month 1, day 1 — the year of
    // the dragon, sexagenary 甲辰 / jia-chen (the 41st stem-branch, i.e.
    // (2024 − 4) mod 60 + 1). Values verified against Node/V8
    // `new Intl.DateTimeFormat(loc,{calendar:'chinese',dateStyle}).format`.
    //
    // `U` = cyclic year NAME, `r` = related Gregorian year, month name is numeric.
    assert_eq!(
        fc("en", 2024, 1, 1, false, Full),
        "Saturday, First Month 1, 2024(jia-chen)"
    );
    assert_eq!(
        fc("en", 2024, 1, 1, false, Long),
        "First Month 1, 2024(jia-chen)"
    );
    // Medium carries the related year (`r`) but no cyclic name.
    assert_eq!(fc("en", 2024, 1, 1, false, Medium), "Mo1 1, 2024");
    assert_eq!(fc("en", 2024, 1, 1, false, Short), "1/1/2024");

    // Leap month: 2023 had a leap 2nd month; 2023-04-01 = leap month 2, day 11,
    // sexagenary 癸卯 / gui-mao (year 2023 → (2023 − 4) mod 60 + 1 = 40). The
    // leap marker wraps the month name (`"Second Monthbis"`), even numeric ones.
    assert_eq!(
        fc("en", 2023, 2, 11, true, Long),
        "Second Monthbis 11, 2023(gui-mao)"
    );
    assert_eq!(fc("en", 2023, 2, 11, true, Short), "2bis/11/2023");
    // `y` (German short pattern `dd.MM.yy`) renders the cyclic year NUMBER (40),
    // with the leap marker on the numeric month.
    assert_eq!(fc("de", 2023, 2, 11, true, Short), "11.02bis.40");

    // A non-Latin locale (`zh`): `rU年MMMd` → related year + cyclic name 甲辰 +
    // month name 正月. (V8 renders the day with the `hanidays` numbering — 初一 —
    // but, like the Islamic/Persian formatters, this crate uses ASCII digits.)
    assert_eq!(fc("zh", 2024, 1, 1, false, Long), "2024甲辰年正月1");
    assert!(fc("zh", 2024, 1, 1, false, Full).starts_with("2024甲辰年正月"));
    // zh leap month applies the 闰 marker.
    assert_eq!(fc("zh", 2023, 2, 11, true, Long), "2023癸卯年闰二月11");
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
    let o = dtf(|o| {
        o.year = n;
        o.month = Some(MonthStyle::Short);
        o.day = n;
    });
    assert_eq!(fo("en", &DT, &o).unwrap(), "Jun 4, 2026");
    let kinds: Vec<_> = ftp("en", &DT, &o)
        .unwrap()
        .iter()
        .map(|p| p.kind.as_str().to_string())
        .collect();
    assert_eq!(kinds, ["month", "literal", "day", "literal", "year"]);

    // 24-hour time.
    let t = dtf(|o| {
        o.hour = n;
        o.minute = td;
        o.hour_cycle = Some(HourCycle::H23);
    });
    assert_eq!(fo("en", &DT, &t).unwrap(), "14:30");

    // 12-hour time.
    let t12 = dtf(|o| {
        o.hour = n;
        o.minute = td;
        o.hour12 = Some(true);
    });
    assert_eq!(fo("en", &DT, &t12).unwrap(), "2:30\u{202f}PM");

    // dateStyle shortcut.
    let ds = dtf(|o| o.date_style = Some(Long));
    assert_eq!(fo("en", &DT, &ds).unwrap(), "June 4, 2026");

    // Conflicting options.
    let bad = dtf(|o| {
        o.date_style = Some(Long);
        o.year = n;
    });
    assert_eq!(
        fo("en", &DT, &bad),
        Err(DateTimeFormatError::ConflictingOptions)
    );

    // Narrow month (asserted on the part, robust to surrounding fields).
    let narrow = dtf(|o| {
        o.month = Some(MonthStyle::Narrow);
        o.day = n;
    });
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
    let frac = dtf(|o| {
        o.hour = n;
        o.minute = td;
        o.second = td;
        o.fractional_second_digits = Some(3);
        o.hour_cycle = Some(HourCycle::H23);
    });
    let ms = DateTime {
        millisecond: 50,
        ..DT
    };
    assert_eq!(fo("en", &ms, &frac).unwrap(), "14:30:05.050");

    // timeZoneName offset.
    let tz = dtf(|o| {
        o.hour = n;
        o.minute = td;
        o.hour_cycle = Some(HourCycle::H23);
        o.time_zone_name = Some(TimeZoneNameStyle::LongOffset);
        o.tz_offset_minutes = Some(-480);
    });
    let parts = ftp("en", &DT, &tz).unwrap();
    assert_eq!(parts.last().unwrap().kind, DateTimePartType::TimeZoneName);
    assert_eq!(parts.last().unwrap().value, "GMT-08:00");

    // Default (no options) → numeric y/M/d.
    assert_eq!(
        fo("en", &DT, &DateTimeFormatOptions::default()).unwrap(),
        "6/4/2026"
    );

    // weekday:Narrow part value.
    let wd = dtf(|o| o.weekday = Some(NameStyle::Narrow));
    let parts = ftp("en", &DT, &wd).unwrap();
    assert_eq!(parts[0].value, "T"); // Thursday narrow
}

#[test]
fn component_locale_defaults_and_field_keep() {
    use intl::datetime::{MonthStyle, NameStyle, Numeric2Digit, format_options as fo};
    let n = Some(Numeric2Digit::Numeric);
    let td = Some(Numeric2Digit::TwoDigit);

    // Default hour cycle is derived from the locale's CLDR time pattern:
    // en-US is 12-hour, de is 24-hour (no explicit hourCycle/hour12).
    let hm = dtf(|o| {
        o.hour = n;
        o.minute = td;
    });
    assert_eq!(fo("en", &DT, &hm).unwrap(), "2:30\u{202f}PM");
    assert_eq!(fo("de", &DT, &hm).unwrap(), "14:30");

    // Weekday must survive when combined with a wide month + day (the exact
    // skeleton MMMMEd is absent, but MMMEd matches and the width is patched).
    let wmd = dtf(|o| {
        o.weekday = Some(NameStyle::Long);
        o.month = Some(MonthStyle::Long);
        o.day = n;
    });
    assert_eq!(fo("en", &DT, &wmd).unwrap(), "Thursday, June 4");
}

#[test]
fn hour_cycles() {
    use intl::datetime::{HourCycle, Numeric2Digit, format_options as fo};
    let n = Some(Numeric2Digit::Numeric);
    let td = Some(Numeric2Digit::TwoDigit);
    let at = |h: u8, c: HourCycle| {
        let dt = DateTime {
            hour: h,
            minute: 0,
            ..DT
        };
        let opts = dtf(|o| {
            o.hour = n;
            o.minute = td;
            o.hour_cycle = Some(c);
        });
        fo("en", &dt, &opts).unwrap()
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

#[test]
fn flexible_day_period() {
    use intl::datetime::format_skeleton as fs;
    let at = |h, mi| DateTime {
        hour: h,
        minute: mi,
        second: 0,
        ..DT
    };
    // Range periods by hour (en: morning <12, afternoon <18, evening <21, night).
    assert_eq!(fs("en", &at(9, 30), "Bhm"), "9:30 in the morning");
    assert_eq!(fs("en", &at(15, 30), "Bhm"), "3:30 in the afternoon");
    assert_eq!(fs("en", &at(19, 30), "Bhm"), "7:30 in the evening");
    assert_eq!(fs("en", &at(22, 30), "Bhm"), "10:30 at night");
    // Midnight/noon only at the exact instant.
    assert_eq!(fs("en", &at(12, 0), "Bh"), "12 noon");
    assert_eq!(fs("en", &at(0, 0), "Bh"), "12 midnight");
    assert_eq!(fs("en", &at(12, 30), "Bh"), "12 in the afternoon");

    // dayPeriod option promotes am/pm to the flexible period.
    use intl::datetime::{NameStyle, Numeric2Digit, format_options as fo};
    let o = dtf(|o| {
        o.hour = Some(Numeric2Digit::Numeric);
        o.day_period = Some(NameStyle::Long);
        o.hour12 = Some(true);
    });
    assert_eq!(fo("en", &at(9, 0), &o).unwrap(), "9\u{202f}in the morning");
}

#[cfg(feature = "iana-tz")]
#[test]
fn named_time_zone() {
    use intl::datetime::{HourCycle, Numeric2Digit, TimeZoneNameStyle, format_options as fo};
    let mk = |zone, style| {
        dtf(move |o| {
            o.hour = Some(Numeric2Digit::Numeric);
            o.minute = Some(Numeric2Digit::TwoDigit);
            o.hour_cycle = Some(HourCycle::H23);
            o.time_zone = Some(zone);
            o.time_zone_name = Some(style);
        })
    };
    let jul = DateTime { month: 7, ..DT };
    let jan = DateTime { month: 1, ..DT };
    // DST-aware abbreviation from the tz database.
    assert!(
        fo(
            "en",
            &jul,
            &mk("America/New_York", TimeZoneNameStyle::Short)
        )
        .unwrap()
        .ends_with("EDT")
    );
    assert!(
        fo(
            "en",
            &jan,
            &mk("America/New_York", TimeZoneNameStyle::Short)
        )
        .unwrap()
        .ends_with("EST")
    );
    assert!(
        fo("en", &jul, &mk("Asia/Tokyo", TimeZoneNameStyle::Short))
            .unwrap()
            .ends_with("JST")
    );
    // Offset styles are zone-derived (DST-aware).
    assert!(
        fo(
            "en",
            &jul,
            &mk("America/New_York", TimeZoneNameStyle::LongOffset)
        )
        .unwrap()
        .ends_with("GMT-04:00")
    );
    assert!(
        fo(
            "en",
            &jan,
            &mk("America/New_York", TimeZoneNameStyle::LongOffset)
        )
        .unwrap()
        .ends_with("GMT-05:00")
    );
}
