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

#[cfg(feature = "calendars-extra")]
#[test]
fn japanese_dates() {
    use intl::datetime::{DateStyle::*, format_japanese_date as fj};

    // Every modern-era assertion below is the EXACT output of Node/V8
    // `new Intl.DateTimeFormat(loc,{calendar:'japanese',dateStyle}).format(date)`
    // (V8 uses the same CLDR `dateFormats` patterns this formatter renders). The
    // input is a Gregorian (year, month, day); the era/year-within-era come from
    // `calendar::japanese_era`.

    // ---- Reiwa (era starts 2019-05-01). ----
    // 2019-05-01 is Reiwa 1; in `en` the year is numeric ("1"), in `ja` it is 元
    // (gannen) for the full/long/medium styles (CLDR `jpanyear` numbering).
    assert_eq!(fj("en", 2019, 5, 1, Full), "Wednesday, May 1, 1 Reiwa");
    assert_eq!(fj("en", 2019, 5, 1, Long), "May 1, 1 Reiwa");
    assert_eq!(fj("en", 2019, 5, 1, Short), "5/1/1 R"); // narrow era (GGGGG)
    assert_eq!(fj("ja", 2019, 5, 1, Long), "令和元年5月1日"); // gannen
    assert_eq!(fj("ja", 2019, 5, 1, Full), "令和元年5月1日水曜日");
    // The `ja` short pattern (GGGGGy/M/d) has no `jpanyear`, so year 1 is "1".
    assert_eq!(fj("ja", 2019, 5, 1, Short), "R1/5/1");
    // 2024 is Reiwa 6 (year 6, no gannen).
    assert_eq!(fj("en", 2024, 3, 15, Long), "March 15, 6 Reiwa");
    assert_eq!(fj("en", 2024, 3, 15, Medium), "Mar 15, 6 Reiwa");
    assert_eq!(fj("ja", 2024, 3, 15, Long), "令和6年3月15日");

    // ---- Heisei: 2019-04-30 is the last Heisei day → Heisei 31. ----
    assert_eq!(fj("en", 2019, 4, 30, Long), "April 30, 31 Heisei");
    assert_eq!(fj("ja", 2019, 4, 30, Long), "平成31年4月30日");

    // ---- Shōwa. 1970-01-01 → Shōwa 45; 1926-12-25 (Shōwa era start) → Shōwa 1
    // (gannen in `ja`). Note the localized wide era name carries the macron. ----
    assert_eq!(fj("en", 1970, 1, 1, Long), "January 1, 45 Shōwa");
    assert_eq!(fj("ja", 1970, 1, 1, Long), "昭和45年1月1日");
    assert_eq!(fj("ja", 1926, 12, 25, Long), "昭和元年12月25日"); // gannen

    // ---- Taishō era start 1912-07-30 → Taishō 1 (gannen in `ja`). ----
    assert_eq!(fj("en", 1912, 7, 30, Long), "July 30, 1 Taishō");
    assert_eq!(fj("ja", 1912, 7, 30, Long), "大正元年7月30日"); // gannen

    // A non-en/ja locale (`fr`) localizes month names; the era names are the CLDR
    // Japanese era names (V8: "1 mai 1 Reiwa").
    assert_eq!(fj("fr", 2019, 5, 1, Long), "1 mai 1 Reiwa");

    // ---- Pre-Meiji historical nengō. Now rendered from ICU's Gregorian
    // era-start dates + the localized era names. Every assertion is the EXACT
    // output of Node/V8 `Intl.DateTimeFormat(loc,{calendar:'japanese',dateStyle})`.
    // Kaei era (1848-02-28 .. 1854-11-27): 1850 → Kaei 3. ----
    assert_eq!(fj("en", 1850, 3, 15, Long), "March 15, 3 Kaei (1848–1854)");
    assert_eq!(fj("en", 1850, 3, 15, Medium), "Mar 15, 3 Kaei (1848–1854)");
    assert_eq!(fj("en", 1850, 3, 15, Short), "3/15/3 Kaei (1848–1854)");
    assert_eq!(
        fj("en", 1850, 3, 15, Full),
        "Friday, March 15, 3 Kaei (1848–1854)"
    );
    assert_eq!(fj("ja", 1850, 3, 15, Long), "嘉永3年3月15日");
    assert_eq!(fj("ja", 1850, 3, 15, Full), "嘉永3年3月15日金曜日");
    assert_eq!(fj("ja", 1850, 3, 15, Short), "嘉永3/3/15"); // GGGGG narrow
    assert_eq!(fj("fr", 1850, 3, 15, Long), "15 mars 3 Kaei (1848–1854)");
    // Genroku era (1688-09-30 .. 1704-03-13): 1700 → Genroku 13.
    assert_eq!(
        fj("en", 1700, 1, 1, Long),
        "January 1, 13 Genroku (1688–1704)"
    );
    assert_eq!(fj("ja", 1700, 1, 1, Long), "元禄13年1月1日");
    // Genna era start 1615-07-13 (first post-1582-cutover era boundary).
    assert_eq!(fj("en", 1615, 7, 13, Long), "July 13, 1 Genna (1615–1624)");
    // The Meiji boundary matches ICU/V8 (1868-09-08, not the civil 1868-10-23):
    // 1868-09-10 is already Meiji 1 (gannen in `ja`).
    assert_eq!(fj("ja", 1868, 9, 10, Long), "明治元年9月10日");
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
    // DST-aware short specific name, from CLDR's `America_Eastern` metazone.
    #[cfg(feature = "tz-names-america")]
    assert!(
        fo(
            "en",
            &jul,
            &mk("America/New_York", TimeZoneNameStyle::Short)
        )
        .unwrap()
        .ends_with("EDT")
    );
    #[cfg(feature = "tz-names-america")]
    assert!(
        fo(
            "en",
            &jan,
            &mk("America/New_York", TimeZoneNameStyle::Short)
        )
        .unwrap()
        .ends_with("EST")
    );
    // CLDR has no short name for the `Japan` metazone, so `short` falls back to
    // the short localized GMT offset — not to the tz database's `JST`, which is
    // English-only and not what ECMA-402 asks for. Matches V8/ICU.
    assert!(
        fo("en", &jul, &mk("Asia/Tokyo", TimeZoneNameStyle::Short))
            .unwrap()
            .ends_with("GMT+9")
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

/// UTS #35's `y` is era-relative: on the BCE side it counts back from 1, so the
/// astronomical year 0 renders as `1 BC` and −1 as `2 BC`. A negative number
/// beside a `BC` era is never correct. The astronomical year stays reachable
/// through the `u` field.
#[test]
fn era_relative_year() {
    use intl::datetime::{NameStyle, Numeric2Digit};

    let at = |y: i32| DateTime { year: y, ..DT };
    let with_era = dtf(|o| {
        o.year = Some(Numeric2Digit::Numeric);
        o.era = Some(NameStyle::Short);
    });
    let fmt = |y: i32, o: &intl::datetime::DateTimeFormatOptions| {
        intl::datetime::format_options("en", &at(y), o).unwrap()
    };

    assert_eq!(fmt(1, &with_era), "1 AD");
    assert_eq!(fmt(0, &with_era), "1 BC");
    assert_eq!(fmt(-1, &with_era), "2 BC");
    assert_eq!(fmt(-99, &with_era), "100 BC");
    assert_eq!(fmt(2026, &with_era), "2026 AD");

    // The era-relative year is what `y` means whether or not an era is asked
    // for, so a bare year never renders negative either.
    let bare = dtf(|o| o.year = Some(Numeric2Digit::Numeric));
    assert_eq!(fmt(0, &bare), "1");
    assert_eq!(fmt(-1, &bare), "2");

    // Two-digit year takes the last two digits of the era-relative year.
    let two_digit = dtf(|o| {
        o.year = Some(Numeric2Digit::TwoDigit);
        o.era = Some(NameStyle::Short);
    });
    assert_eq!(fmt(0, &two_digit), "01 BC");
    assert_eq!(fmt(-99, &two_digit), "00 BC");

    // `formatToParts` agrees with the string path.
    let parts = intl::datetime::format_to_parts("en", &at(0), &with_era).unwrap();
    let year = parts
        .iter()
        .find(|p| p.kind == intl::datetime::DateTimePartType::Year)
        .expect("year part");
    assert_eq!(year.value, "1");

    // The skeleton path shares the same field renderer.
    assert_eq!(intl::datetime::format_skeleton("en", &at(0), "y"), "1");
    assert_eq!(intl::datetime::format_skeleton("en", &at(-1), "y"), "2");
}

/// A lone time field used to resolve to a date pattern that the field-keep pass
/// then emptied, returning `Ok("")`. CLDR only tabulates the `availableFormats`
/// combinations it expects to be asked for, so `m`, `s`, `B` and `S` have no
/// entry anywhere and the pattern is synthesized from the skeleton instead.
#[test]
fn lone_time_fields() {
    use intl::datetime::{NameStyle, Numeric2Digit};

    const T: DateTime = DateTime {
        year: 2024,
        month: 6,
        day: 15,
        hour: 9,
        minute: 5,
        second: 7,
        millisecond: 40,
    };
    let f = |o: &intl::datetime::DateTimeFormatOptions| {
        intl::datetime::format_options("en", &T, o).unwrap()
    };

    assert_eq!(
        f(&dtf(|o| o.day_period = Some(NameStyle::Long))),
        "in the morning"
    );
    // Both widths render unpadded: UTS #35 matches the requested field length
    // only for the hour, so a minute or second takes the width of the pattern it
    // landed in — here the synthesized `m`/`s`. ECMA-402 reports that back, which
    // is why V8's `resolvedOptions()` answers `numeric` to a `2-digit` request.
    assert_eq!(f(&dtf(|o| o.minute = Some(Numeric2Digit::Numeric))), "5");
    assert_eq!(f(&dtf(|o| o.minute = Some(Numeric2Digit::TwoDigit))), "5");
    assert_eq!(f(&dtf(|o| o.second = Some(Numeric2Digit::Numeric))), "7");
    assert_eq!(f(&dtf(|o| o.second = Some(Numeric2Digit::TwoDigit))), "7");
    assert_eq!(f(&dtf(|o| o.fractional_second_digits = Some(3))), "040");
    assert_eq!(f(&dtf(|o| o.fractional_second_digits = Some(2))), "04");

    // The parts carry the right tag, not just the right text.
    let parts = intl::datetime::format_to_parts(
        "en",
        &T,
        &dtf(|o| o.minute = Some(Numeric2Digit::Numeric)),
    )
    .unwrap();
    assert_eq!(parts.len(), 1);
    assert_eq!(parts[0].kind, intl::datetime::DateTimePartType::Minute);
    let parts =
        intl::datetime::format_to_parts("en", &T, &dtf(|o| o.fractional_second_digits = Some(3)))
            .unwrap();
    assert_eq!(
        parts[0].kind,
        intl::datetime::DateTimePartType::FractionalSecond
    );

    // Other locales pick their own day-period wording.
    let dp = dtf(|o| o.day_period = Some(NameStyle::Long));
    assert_eq!(
        intl::datetime::format_options("de", &T, &dp).unwrap(),
        "morgens"
    );
    assert_eq!(
        intl::datetime::format_options("fr", &T, &dp).unwrap(),
        "du matin"
    );
    assert_eq!(intl::datetime::format_options("ja", &T, &dp).unwrap(), "朝");

    // Combinations CLDR *does* tabulate keep resolving through availableFormats.
    let hm = dtf(|o| {
        o.hour = Some(Numeric2Digit::Numeric);
        o.minute = Some(Numeric2Digit::TwoDigit);
    });
    assert_eq!(f(&hm), "9:05\u{202f}AM");
    let ms = dtf(|o| {
        o.minute = Some(Numeric2Digit::TwoDigit);
        o.second = Some(Numeric2Digit::TwoDigit);
    });
    assert_eq!(f(&ms), "05:07");
    // Hour + day period still promotes the `a` field of the `h` entry to `B`.
    let hb = dtf(|o| {
        o.hour = Some(Numeric2Digit::Numeric);
        o.day_period = Some(NameStyle::Long);
    });
    assert_eq!(f(&hb), "9\u{202f}in the morning");
    // Seconds plus a fraction keeps the locale decimal separator between them.
    let sf = dtf(|o| {
        o.second = Some(Numeric2Digit::TwoDigit);
        o.fractional_second_digits = Some(3);
    });
    assert_eq!(f(&sf), "7.040");
    assert_eq!(
        intl::datetime::format_options("de", &T, &sf).unwrap(),
        "7,040"
    );
}

// ---------------------------------------------------------------------------
// Field-level calendar names (ECMA-402 `era` / `month`, UTS #35 §"Calendar
// Fields"). Expected values are the exact `Intl.DateTimeFormat(loc-u-ca-<cal>,
// {era|month: width}).formatToParts()` output of Node 22 / ICU 77 unless a
// comment says otherwise; where CLDR 48 and ICU 77 (CLDR 47) disagree the
// assertion follows the vendored CLDR 48 data, as in `tests/timezone.rs`.
// ---------------------------------------------------------------------------

#[test]
fn calendar_bcp47_keys_round_trip() {
    use intl::datetime::Calendar;
    for k in [
        "buddhist",
        "chinese",
        "coptic",
        "dangi",
        "ethiopic",
        "ethioaa",
        "gregory",
        "hebrew",
        "indian",
        "islamic",
        "islamic-civil",
        "islamic-rgsa",
        "islamic-tbla",
        "islamic-umalqura",
        "iso8601",
        "japanese",
        "persian",
        "roc",
    ] {
        let c = Calendar::from_bcp47(k).unwrap_or_else(|| panic!("{k}"));
        assert_eq!(c.as_bcp47(), k);
    }
    // The CLDR deprecated spellings older tags carry, and ASCII case folding.
    assert_eq!(Calendar::from_bcp47("gregorian"), Some(Calendar::Gregory));
    assert_eq!(
        Calendar::from_bcp47("islamicc"),
        Some(Calendar::IslamicCivil)
    );
    assert_eq!(
        Calendar::from_bcp47("ethiopic-amete-alem"),
        Some(Calendar::EthiopicAmeteAlem)
    );
    assert_eq!(Calendar::from_bcp47("GREGORY"), Some(Calendar::Gregory));
    // Not a BCP-47 calendar key.
    assert_eq!(Calendar::from_bcp47("julian"), None);
    assert_eq!(Calendar::from_bcp47(""), None);
}

#[test]
fn era_names_gregorian_need_no_extra_calendars() {
    use intl::datetime::{Calendar::*, NameStyle::*, era_name};
    // `gregory` and `iso8601` read `calendar.bin`, so they resolve with the
    // `datetime` feature alone.
    assert_eq!(era_name("en", Gregory, 0, Long), Some("Before Christ"));
    assert_eq!(era_name("en", Gregory, 1, Long), Some("Anno Domini"));
    assert_eq!(era_name("en", Gregory, 1, Short), Some("AD"));
    assert_eq!(era_name("en", Gregory, 1, Narrow), Some("A"));
    assert_eq!(era_name("en", Iso8601, 1, Long), Some("Anno Domini"));
    assert_eq!(era_name("fr", Gregory, 1, Long), Some("après Jésus-Christ"));
    assert_eq!(era_name("ja", Gregory, 1, Long), Some("西暦"));
    // Gregorian has exactly two eras.
    assert_eq!(era_name("en", Gregory, 2, Long), None);
}

#[cfg(feature = "calendars-extra")]
#[test]
fn era_names_across_calendars() {
    use intl::datetime::{Calendar::*, NameStyle::*, era_name};

    // Islamic: CLDR 48 gives `eraNames` a full spelling and a second era; ICU 77
    // (CLDR 47) has only "AH" at every width and no era 1.
    assert_eq!(era_name("en", Islamic, 0, Long), Some("Anno Hegirae"));
    assert_eq!(era_name("en", Islamic, 0, Short), Some("AH"));
    assert_eq!(era_name("en", Islamic, 1, Short), Some("BH"));
    assert_eq!(era_name("fr", Islamic, 0, Long), Some("ère de l’Hégire"));
    // The four Islamic variants are distinct calendars sharing one name set.
    for c in [IslamicCivil, IslamicRgsa, IslamicTbla, IslamicUmalqura] {
        assert_eq!(era_name("en", c, 0, Short), Some("AH"));
    }

    assert_eq!(era_name("en", Persian, 0, Long), Some("AP"));
    assert_eq!(era_name("en", Buddhist, 0, Long), Some("BE"));
    assert_eq!(era_name("fr", Buddhist, 0, Long), Some("ère bouddhique"));
    assert_eq!(era_name("th", Buddhist, 0, Long), Some("พุทธศักราช"));
    assert_eq!(era_name("en", Roc, 1, Long), Some("Minguo"));
    assert_eq!(era_name("en", Roc, 0, Long), Some("B.R.O.C."));
    assert_eq!(era_name("ja", Roc, 1, Long), Some("民国"));
    // CLDR 48 spells the Indian era with the diacritic; ICU 77 has "Saka".
    assert_eq!(era_name("en", Indian, 0, Long), Some("Śaka"));
    assert_eq!(era_name("ja", Indian, 0, Long), Some("サカ"));
    assert_eq!(era_name("en", Hebrew, 0, Long), Some("AM"));
    assert_eq!(era_name("fr", Hebrew, 0, Long), Some("Anno Mundi"));

    // Coptic's only era is CLDR index 1, so index 0 is a real gap. (CLDR 48
    // localized the name; ICU 77 still renders the "ERA1" placeholder in `en`.)
    assert_eq!(era_name("en", Coptic, 0, Long), None);
    assert_eq!(era_name("en", Coptic, 1, Long), Some("Anno Martyrum"));
    assert_eq!(era_name("en", Coptic, 1, Short), Some("AM"));
    assert_eq!(era_name("fr", Coptic, 1, Long), Some("après Dioclétien"));

    // Ethiopic has both; `ethioaa` is the same name set counted from era 0.
    // CLDR 48 replaced the localized Ethiopic era names with "AA"/"AM"
    // everywhere — ICU 77 still has e.g. `fr` "après l’Incarnation".
    assert_eq!(era_name("en", Ethiopic, 0, Long), Some("AA"));
    assert_eq!(era_name("en", Ethiopic, 1, Long), Some("AM"));
    assert_eq!(era_name("fr", Ethiopic, 1, Long), Some("AM"));
    assert_eq!(era_name("en", EthiopicAmeteAlem, 0, Long), Some("AA"));

    // The lunisolar calendars have no eras at all in CLDR; ICU emits no `era`
    // part for them either.
    for c in [Chinese, Dangi] {
        for e in 0..3 {
            assert_eq!(era_name("en", c, e, Long), None);
        }
    }
}

#[cfg(feature = "calendars-extra")]
#[test]
fn japanese_era_names_span_all_237_nengo() {
    use intl::datetime::{Calendar::Japanese, NameStyle::*, era_name};
    // Modern eras (CLDR 232 Meiji … 236 Reiwa).
    assert_eq!(era_name("en", Japanese, 236, Long), Some("Reiwa"));
    assert_eq!(era_name("en", Japanese, 236, Narrow), Some("R"));
    assert_eq!(era_name("ja", Japanese, 236, Long), Some("令和"));
    assert_eq!(era_name("ja", Japanese, 235, Long), Some("平成"));
    assert_eq!(era_name("ja", Japanese, 232, Long), Some("明治"));
    // Historical nengō, from the same call: 226 is Kaei (1848–1854).
    assert_eq!(
        era_name("en", Japanese, 226, Long),
        Some("Kaei (1848–1854)")
    );
    assert_eq!(era_name("ja", Japanese, 226, Long), Some("嘉永"));
    // CLDR gives the historical nengō the same string at all three widths in
    // `en` (the year range is part of the name), unlike the modern five.
    assert_eq!(
        era_name("en", Japanese, 226, Short),
        era_name("en", Japanese, 226, Long)
    );
    assert_eq!(era_name("en", Japanese, 0, Long), Some("Taika (645–650)"));
    // 0..=236, and nothing past it.
    assert!(
        (0..=236).all(|e| era_name("en", Japanese, e, Long).is_some()),
        "every CLDR Japanese era index resolves"
    );
    assert_eq!(era_name("en", Japanese, 237, Long), None);
}

#[cfg(feature = "calendars-extra")]
#[test]
fn month_names_across_calendars() {
    use intl::datetime::{Calendar::*, MonthStyle::*, month_name};
    let m = |lang, cal, n, w| month_name(lang, cal, n, false, w);

    // The `dateStyle: "long"` month a Temporal `PlainYearMonth` needs.
    assert_eq!(m("en", Islamic, 9, Long), Some("Ramadan".into()));
    assert_eq!(m("en", Islamic, 9, Short), Some("Ram.".into()));
    assert_eq!(m("ar", Islamic, 10, Long), Some("شوال".into()));
    assert_eq!(m("en", Persian, 5, Long), Some("Mordad".into()));
    assert_eq!(m("fa", Persian, 5, Long), Some("مرداد".into()));
    assert_eq!(m("en", Indian, 5, Long), Some("Sravana".into()));

    // 13-month calendars.
    assert_eq!(m("en", Coptic, 11, Long), Some("Epep".into()));
    assert_eq!(m("en", Coptic, 13, Long), Some("Nasie".into()));
    assert_eq!(m("en", Ethiopic, 11, Long), Some("Hamle".into()));
    assert_eq!(m("am", Ethiopic, 11, Long), Some("ሐምሌ".into()));
    assert_eq!(m("en", Hebrew, 12, Long), Some("Av".into()));
    assert_eq!(m("he", Hebrew, 12, Long), Some("אב".into()));
    assert_eq!(m("en", Hebrew, 13, Long), Some("Elul".into()));
    // …and the 12-month ones stop at 12.
    assert_eq!(m("en", Islamic, 13, Long), None);
    assert_eq!(m("en", Gregory, 13, Long), None);
    assert_eq!(m("en", Coptic, 14, Long), None);
    assert_eq!(m("en", Islamic, 0, Long), None);

    // Buddhist, ROC and Japanese reuse the locale's Gregorian month names, which
    // is why codegen stores none for them.
    for c in [Buddhist, Roc, Japanese, Gregory, Iso8601] {
        assert_eq!(m("en", c, 9, Long), Some("September".into()));
        assert_eq!(m("en", c, 9, Short), Some("Sep".into()));
        assert_eq!(m("en", c, 9, Narrow), Some("S".into()));
        assert_eq!(m("de", c, 9, Short), Some("Sept.".into()));
    }

    // The numeric widths render the number, for every calendar.
    assert_eq!(m("en", Islamic, 9, Numeric), Some("9".into()));
    assert_eq!(m("en", Islamic, 9, TwoDigit), Some("09".into()));
    // CLDR's narrow month for the non-Gregorian calendars is the month number —
    // but written in the locale's own digits, so it is stored rather than
    // synthesized from the index.
    assert_eq!(m("en", Islamic, 12, Narrow), Some("12".into()));
    assert_eq!(m("ar", Islamic, 12, Narrow), Some("١٢".into()));
}

#[cfg(feature = "calendars-extra")]
#[test]
fn leap_month_names() {
    use intl::datetime::{Calendar::*, MonthStyle::*, month_name};

    // UTS #35 `monthPatterns`: the lunisolar marker *wraps* the ordinary name.
    for c in [Chinese, Dangi] {
        assert_eq!(month_name("en", c, 5, true, Numeric), Some("5bis".into()));
        assert_eq!(month_name("en", c, 5, true, TwoDigit), Some("05bis".into()));
        assert_eq!(
            month_name("en", c, 5, true, Long),
            Some("Fifth Monthbis".into())
        );
        assert_eq!(month_name("en", c, 5, true, Short), Some("Mo5bis".into()));
        assert_eq!(month_name("en", c, 5, true, Narrow), Some("5b".into()));
        // …and is a no-op when the month is not intercalary.
        assert_eq!(
            month_name("en", c, 5, false, Long),
            Some("Fifth Month".into())
        );
    }
    assert_eq!(
        month_name("zh", Chinese, 2, true, Long),
        Some("闰二月".into())
    );
    assert_eq!(
        month_name("ja", Chinese, 2, true, Long),
        Some("閏二月".into())
    );
    assert_eq!(month_name("ko", Dangi, 2, true, Long), Some("윤2월".into()));

    // UTS #35 `yeartype="leap"`: the Hebrew variant *replaces* the name. Month 7
    // is Adar in a common year and Adar II in a leap year; month 6 (Adar I)
    // exists only in leap years and needs no variant.
    assert_eq!(
        month_name("en", Hebrew, 7, false, Long),
        Some("Adar".into())
    );
    assert_eq!(
        month_name("en", Hebrew, 7, true, Long),
        Some("Adar II".into())
    );
    assert_eq!(
        month_name("en", Hebrew, 6, true, Long),
        Some("Adar I".into())
    );
    // Only month 7 has one, and the numeric widths are unaffected.
    assert_eq!(
        month_name("en", Hebrew, 8, true, Long),
        Some("Nisan".into())
    );
    assert_eq!(month_name("en", Hebrew, 7, true, Numeric), Some("7".into()));

    // A solar calendar has no leap variant at all, so the flag is inert.
    assert_eq!(
        month_name("en", Islamic, 9, true, Long),
        month_name("en", Islamic, 9, false, Long)
    );
}

#[cfg(feature = "calendars-extra")]
#[test]
fn cyclic_year_names() {
    use intl::datetime::{Calendar::*, cyclic_year_name as cy};
    // 2024 is cycle position 41, 甲辰 / jia-chen.
    assert_eq!(cy("en", Chinese, 41), Some("jia-chen"));
    assert_eq!(cy("zh", Chinese, 41), Some("甲辰"));
    assert_eq!(cy("ja", Chinese, 41), Some("甲辰"));
    assert_eq!(cy("en", Chinese, 1), Some("jia-zi"));
    assert_eq!(cy("en", Chinese, 60), Some("gui-hai"));
    // Out of the 60-name cycle, and calendars that do not name years this way.
    assert_eq!(cy("en", Chinese, 0), None);
    assert_eq!(cy("en", Chinese, 61), None);
    assert_eq!(cy("en", Gregory, 41), None);
    assert_eq!(cy("en", Islamic, 41), None);
}

/// Without `calendars-extra` the non-Gregorian tables are not compiled in, and
/// the field lookups say so with `None` rather than an empty string.
#[cfg(not(feature = "calendars-extra"))]
#[test]
fn alternate_calendars_report_absent_data_as_none() {
    use intl::datetime::{Calendar::*, MonthStyle, NameStyle, era_name, month_name};
    assert_eq!(era_name("en", Islamic, 0, NameStyle::Long), None);
    assert_eq!(month_name("en", Islamic, 9, false, MonthStyle::Long), None);
    // Gregorian still works: it is `calendar.bin`, which `datetime` embeds.
    assert_eq!(
        month_name("en", Gregory, 9, false, MonthStyle::Long),
        Some("September".into())
    );
}

/// UTS #35 has two date+time combining patterns, and ICU picks between them by
/// what `{0}` holds: `dateTimeFormats-atTime` when it is a *time of day*, the
/// plain slot otherwise. They differ in 74 of the 101 locales, and the length
/// follows the date half's own width. Values match V8/ICU.
#[test]
fn date_time_combiner_uses_the_at_time_slot() {
    use intl::datetime::{DateTimeFormatOptions, MonthStyle, NameStyle, Numeric2Digit};

    const T: DateTime = DateTime {
        year: 2021,
        month: 8,
        day: 4,
        hour: 12,
        minute: 0,
        second: 0,
        millisecond: 0,
    };

    // Whole styles.
    assert_eq!(
        fdt("en", &T, Long, Short),
        "August 4, 2021 at 12:00\u{202f}PM"
    );
    assert_eq!(fdt("fi", &T, Long, Short), "4. elokuuta 2021 klo 12.00");
    assert_eq!(fdt("fr", &T, Full, Short), "mercredi 4 août 2021 à 12:00");
    // `en`'s short/medium `atTime` is the plain "{1}, {0}", so nothing changes.
    assert_eq!(fdt("en", &T, Short, Short), "8/4/21, 12:00\u{202f}PM");

    // Components: the slot's length follows the *requested* month width, not the
    // representative `MMM` the skeleton lookup uses.
    let with = |b: &dyn Fn(&mut DateTimeFormatOptions)| {
        let mut o = DateTimeFormatOptions::default();
        o.day = Some(Numeric2Digit::Numeric);
        o.year = Some(Numeric2Digit::Numeric);
        o.hour = Some(Numeric2Digit::Numeric);
        o.minute = Some(Numeric2Digit::TwoDigit);
        b(&mut o);
        o
    };
    let f = |o: &DateTimeFormatOptions| intl::datetime::format_options("en", &T, o).unwrap();
    assert_eq!(
        f(&with(&|o| o.month = Some(MonthStyle::Long))),
        "August 4, 2021 at 12:00\u{202f}PM"
    );
    assert_eq!(
        f(&with(&|o| o.month = Some(MonthStyle::Short))),
        "Aug 4, 2021, 12:00\u{202f}PM"
    );
    assert_eq!(
        f(&with(&|o| o.month = Some(MonthStyle::Numeric))),
        "8/4/2021, 12:00\u{202f}PM"
    );
    assert_eq!(
        f(&with(&|o| {
            o.month = Some(MonthStyle::Long);
            o.weekday = Some(NameStyle::Long);
        })),
        "Wednesday, August 4, 2021 at 12:00\u{202f}PM"
    );

    // A range's `{0}` is a time *range*, not a time of day, so it keeps the
    // plain slot even with a wide month — as ICU does.
    let a = DateTime {
        year: 2024,
        month: 6,
        day: 15,
        hour: 9,
        ..T
    };
    let b = DateTime { hour: 17, ..a };
    assert_eq!(
        intl::datetime::format_range("en", &a, &b, &with(&|o| o.month = Some(MonthStyle::Long)))
            .unwrap(),
        "June 15, 2024, 9:00\u{202f}AM\u{2009}–\u{2009}5:00\u{202f}PM"
    );
}
