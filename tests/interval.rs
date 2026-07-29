//! Date/time interval formatting (`formatRange` / `formatRangeToParts`).
//!
//! Expected values cross-checked against `Intl.DateTimeFormat.prototype.formatRange`
//! (V8 / Node) with the vendored CLDR 48 data. The `–` separators are the CLDR
//! thin-space + en-dash (`\u{2009}\u{2013}\u{2009}`).
#![cfg(feature = "datetime")]

use intl::datetime::{
    DateTime, DateTimeFormatOptions, DateTimePartType, DateTimeRangePart, MonthStyle,
    Numeric2Digit, RangeSource, format_range, format_range_to_parts,
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

/// `{year, month:'numeric', day, hour, minute:'2-digit'}` → skeleton `yMdhm`.
fn ymdhm() -> DateTimeFormatOptions {
    let mut o = DateTimeFormatOptions::default();
    o.year = Some(Numeric2Digit::Numeric);
    o.month = Some(MonthStyle::Numeric);
    o.day = Some(Numeric2Digit::Numeric);
    o.hour = Some(Numeric2Digit::Numeric);
    o.minute = Some(Numeric2Digit::TwoDigit);
    o
}

fn at(y: i32, m: u8, d: u8, hour: u8, minute: u8) -> DateTime {
    DateTime {
        year: y,
        month: m,
        day: d,
        hour,
        minute,
        ..EPOCH
    }
}

fn tagged(parts: &[DateTimeRangePart]) -> Vec<(DateTimePartType, &str, RangeSource)> {
    parts
        .iter()
        .map(|p| (p.kind, p.value.as_str(), p.source))
        .collect()
}

const DASH: &str = "\u{2009}\u{2013}\u{2009}"; // thin space + en dash + thin space
const NNBSP: &str = "\u{202f}"; // narrow no-break space, before en am/pm

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

#[test]
fn fallback_literals_belong_to_their_half() {
    // No `intervalFormats` key mixes date and time fields, and the day differs, so
    // both ends are formatted with the whole pattern and joined by
    // `intervalFormatFallback`. Everything a half produces is that half's,
    // literals included; only the fallback's own separator is shared.
    let mut o = ymmmd();
    o.hour = Some(Numeric2Digit::Numeric);
    o.minute = Some(Numeric2Digit::TwoDigit);
    let parts =
        format_range_to_parts("en", &at(2024, 6, 15, 9, 0), &at(2024, 6, 16, 17, 0), &o).unwrap();
    use DateTimePartType::*;
    use RangeSource::*;
    assert_eq!(
        tagged(&parts),
        vec![
            (Month, "Jun", StartRange),
            (Literal, " ", StartRange),
            (Day, "15", StartRange),
            (Literal, ", ", StartRange),
            (Year, "2024", StartRange),
            (Literal, ", ", StartRange),
            (Hour, "9", StartRange),
            (Literal, ":", StartRange),
            (Minute, "00", StartRange),
            (Literal, NNBSP, StartRange),
            (DayPeriod, "AM", StartRange),
            (Literal, DASH, Shared),
            (Month, "Jun", EndRange),
            (Literal, " ", EndRange),
            (Day, "16", EndRange),
            (Literal, ", ", EndRange),
            (Year, "2024", EndRange),
            (Literal, ", ", EndRange),
            (Hour, "5", EndRange),
            (Literal, ":", EndRange),
            (Minute, "00", EndRange),
            (Literal, NNBSP, EndRange),
            (DayPeriod, "PM", EndRange),
        ]
    );
}

#[test]
fn seconds_only_difference_is_a_range() {
    // `s` is a range field of its own: two instants a few seconds apart must not
    // collapse to a single time. CLDR keys no `hms` interval pattern, so the
    // whole time pattern is repeated through the fallback.
    let mut o = DateTimeFormatOptions::default();
    o.hour = Some(Numeric2Digit::Numeric);
    o.minute = Some(Numeric2Digit::TwoDigit);
    o.second = Some(Numeric2Digit::TwoDigit);
    let a = DateTime {
        second: 10,
        ..at(2024, 6, 15, 9, 0)
    };
    let b = DateTime { second: 45, ..a };
    assert_eq!(
        format_range("en", &a, &b, &o).unwrap(),
        format!("9:00:10{NNBSP}AM{DASH}9:00:45{NNBSP}AM")
    );
}

#[test]
fn fractional_second_only_difference_is_a_range() {
    // ECMA-402 groups the fractional second with `s` as one range field, so a
    // sub-second difference the pattern displays is a difference.
    let mut o = DateTimeFormatOptions::default();
    o.hour = Some(Numeric2Digit::Numeric);
    o.minute = Some(Numeric2Digit::TwoDigit);
    o.second = Some(Numeric2Digit::TwoDigit);
    o.fractional_second_digits = Some(3);
    let a = DateTime {
        second: 10,
        millisecond: 100,
        ..at(2024, 6, 15, 9, 0)
    };
    let b = DateTime {
        millisecond: 900,
        ..a
    };
    assert_eq!(
        format_range("en", &a, &b, &o).unwrap(),
        format!("9:00:10.100{NNBSP}AM{DASH}9:00:10.900{NNBSP}AM")
    );
    // A millisecond difference the pattern does not show stays a single value.
    let mut plain = o;
    plain.fractional_second_digits = None;
    assert_eq!(
        format_range("en", &a, &b, &plain).unwrap(),
        format!("9:00:10{NNBSP}AM")
    );
}

#[test]
fn date_time_composition_day_period_difference() {
    // UTS #35 §2.6.2: only the time differs, so the date is formatted once and the
    // time range is glued into it with `dateTimeFormats.medium` (`"{1}, {0}"`).
    // 9 AM → 5 PM crosses noon, so the greatest difference is `a`, whose `hm`
    // pattern repeats the day period on both ends.
    let parts = format_range_to_parts(
        "en",
        &at(2024, 6, 15, 9, 0),
        &at(2024, 6, 15, 17, 0),
        &ymdhm(),
    )
    .unwrap();
    use DateTimePartType::*;
    use RangeSource::*;
    assert_eq!(
        tagged(&parts),
        vec![
            (Month, "6", Shared),
            (Literal, "/", Shared),
            (Day, "15", Shared),
            (Literal, "/", Shared),
            (Year, "2024", Shared),
            (Literal, ", ", Shared),
            (Hour, "9", StartRange),
            (Literal, ":", StartRange),
            (Minute, "00", StartRange),
            (Literal, NNBSP, StartRange),
            (DayPeriod, "AM", StartRange),
            (Literal, DASH, Shared),
            (Hour, "5", EndRange),
            (Literal, ":", EndRange),
            (Minute, "00", EndRange),
            (Literal, NNBSP, EndRange),
            (DayPeriod, "PM", EndRange),
        ]
    );
}

#[test]
fn date_time_composition_hour_difference() {
    // Same half of the day: the greatest difference is `h`, whose `hm` pattern
    // (`"h:mm – h:mm a"`) names the day period once, making it shared.
    let out = format_range(
        "en",
        &at(2024, 6, 15, 10, 0),
        &at(2024, 6, 15, 11, 0),
        &ymdhm(),
    )
    .unwrap();
    assert_eq!(out, format!("6/15/2024, 10:00{DASH}11:00{NNBSP}AM"));
}

#[test]
fn date_time_composition_seconds_difference() {
    // No `hms` interval pattern exists, so the time range goes through the
    // fallback — but the date is still shown once, as ICU does for two instants
    // on the same day.
    let mut o = ymdhm();
    o.second = Some(Numeric2Digit::TwoDigit);
    let a = DateTime {
        second: 10,
        ..at(2024, 6, 15, 9, 0)
    };
    let b = DateTime { second: 45, ..a };
    assert_eq!(
        format_range("en", &a, &b, &o).unwrap(),
        format!("6/15/2024, 9:00:10{NNBSP}AM{DASH}9:00:45{NNBSP}AM")
    );
}

#[test]
fn date_time_composition_localized() {
    // ja glues with `"{1} {0}"` and ranges `Hm` as `"H時mm分～H時mm分"`.
    let out = format_range(
        "ja",
        &at(2024, 6, 15, 9, 0),
        &at(2024, 6, 15, 17, 0),
        &ymdhm(),
    )
    .unwrap();
    assert_eq!(out, "2024/6/15 9時00分～17時00分");
}

#[test]
fn date_difference_keeps_whole_pattern_on_both_sides() {
    // The greatest difference is a date field, so composition does not apply and
    // both ends carry the full date+time, as ICU does.
    let out = format_range(
        "en",
        &at(2024, 6, 15, 9, 0),
        &at(2024, 6, 16, 17, 0),
        &ymdhm(),
    )
    .unwrap();
    assert_eq!(
        out,
        format!("6/15/2024, 9:00{NNBSP}AM{DASH}6/16/2024, 5:00{NNBSP}PM")
    );
}

/// `date_style`/`time_style` name a whole pattern rather than a set of
/// components, so they left both component skeletons empty and every styled
/// range keyed as `yMd`: a time-only difference found no differing field at all
/// and silently formatted a single instant, and a date difference was rendered
/// with the `yMd` interval pattern, discarding the style. The skeleton is now
/// recovered from the pattern the style resolved to. Values match V8/ICU.
#[test]
fn style_shortcuts_range() {
    use intl::datetime::DateStyle;

    let styled = |d: Option<DateStyle>, t: Option<DateStyle>| {
        let mut o = DateTimeFormatOptions::default();
        o.date_style = d;
        o.time_style = t;
        o
    };
    let nine = at(2024, 6, 15, 9, 0);
    let five = at(2024, 6, 15, 17, 0);
    let next = at(2024, 6, 16, 17, 0);

    // Date + time, same day: the date is shown once and only the time ranges.
    let both = styled(Some(DateStyle::Medium), Some(DateStyle::Short));
    assert_eq!(
        format_range("en", &nine, &five, &both).unwrap(),
        format!("Jun 15, 2024, 9:00{NNBSP}AM{DASH}5:00{NNBSP}PM")
    );
    // Different days: the whole pattern on both ends, style intact.
    assert_eq!(
        format_range("en", &nine, &next, &both).unwrap(),
        format!("Jun 15, 2024, 9:00{NNBSP}AM{DASH}Jun 16, 2024, 5:00{NNBSP}PM")
    );
    // A time style alone over a time-only difference used to drop the end.
    let time_only = styled(None, Some(DateStyle::Short));
    assert_eq!(
        format_range("en", &nine, &five, &time_only).unwrap(),
        format!("9:00{NNBSP}AM{DASH}5:00{NNBSP}PM")
    );
    // A date style alone keeps its own width: `short` asks for a 2-digit year,
    // which CLDR keys as plain `y`, and `medium` collapses the shared month.
    assert_eq!(
        format_range("en", &nine, &next, &styled(Some(DateStyle::Short), None)).unwrap(),
        format!("6/15/24{DASH}6/16/24")
    );
    assert_eq!(
        format_range("en", &nine, &next, &styled(Some(DateStyle::Medium), None)).unwrap(),
        format!("Jun 15{DASH}16, 2024")
    );

    // Other locales compose with their own glue and separators.
    assert_eq!(
        format_range("ja", &nine, &five, &both).unwrap(),
        "2024/06/15 9\u{6642}00\u{5206}\u{ff5e}17\u{6642}00\u{5206}"
    );
    assert_eq!(
        format_range("de", &nine, &five, &both).unwrap(),
        "15.06.2024, 09:00\u{2013}17:00 Uhr"
    );

    // Identical instants still collapse to a single formatted value.
    assert_eq!(
        format_range("en", &nine, &nine, &both).unwrap(),
        format!("Jun 15, 2024, 9:00{NNBSP}AM")
    );
}
