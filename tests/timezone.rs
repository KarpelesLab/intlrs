//! POSIX TZ zones and CLDR localized time-zone names (UTS #35 §4.8).
#![cfg(feature = "datetime")]
use intl::datetime::DateTime;
use intl::timezone::PosixTz;

fn dt(m: u8, d: u8, h: u8) -> DateTime {
    DateTime {
        year: 2026,
        month: m,
        day: d,
        hour: h,
        minute: 0,
        second: 0,
        millisecond: 0,
    }
}

#[test]
fn us_pacific() {
    let tz = PosixTz::parse("PST8PDT,M3.2.0,M11.1.0/2").unwrap();
    assert_eq!(tz.offset_seconds(&dt(1, 15, 12)), -8 * 3600); // PST
    assert_eq!(tz.offset_seconds(&dt(7, 15, 12)), -7 * 3600); // PDT
    assert!(tz.is_dst(&dt(7, 15, 12)));
    assert!(!tz.is_dst(&dt(1, 15, 12)));
    // DST begins 2nd Sunday of March 2026 = March 8 at 2am.
    assert_eq!(tz.offset_seconds(&dt(3, 8, 1)), -8 * 3600); // before 2am: PST
    assert_eq!(tz.offset_seconds(&dt(3, 8, 3)), -7 * 3600); // after 2am: PDT
}

#[test]
fn no_dst_and_fractional() {
    // India: UTC+5:30, no DST.
    let tz = PosixTz::parse("IST-5:30").unwrap();
    assert_eq!(tz.offset_seconds(&dt(7, 1, 12)), 5 * 3600 + 30 * 60);
    assert!(!tz.is_dst(&dt(7, 1, 12)));
    // Southern hemisphere (DST wraps the year): Central Europe-style sign check.
    let nz = PosixTz::parse("NZST-12NZDT,M9.5.0,M4.1.0/3").unwrap();
    assert_eq!(nz.offset_seconds(&dt(1, 1, 12)), 13 * 3600); // January = DST
    assert_eq!(nz.offset_seconds(&dt(6, 1, 12)), 12 * 3600); // June = standard
}

#[test]
fn rejects_garbage() {
    assert!(PosixTz::parse("").is_none());
    assert!(PosixTz::parse("XYZ").is_none()); // no offset
}

// ---------------------------------------------------------------------------
// Localized time-zone names (UTS #35 §4.8)
//
// Expected values below were checked against V8/ICU (`Intl.DateTimeFormat` with
// `timeZoneName`) unless noted; where CLDR 48 and ICU 77 (CLDR 47) disagree the
// comment says so and the assertion follows the vendored CLDR 48 data.
// ---------------------------------------------------------------------------

use intl::datetime::{
    DateTimeFormatOptions, TimeZoneNameStyle, format_gmt_offset, format_to_parts,
};

/// The `timeZoneName` part for a zone on the 15th of `month` in 2026 (noon
/// local), with a zero fallback offset so a nameless zone still renders.
fn name(lang: &str, zone: &'static str, style: TimeZoneNameStyle, month: u8) -> String {
    let when = DateTime {
        year: 2026,
        month,
        day: 15,
        hour: 12,
        minute: 0,
        second: 0,
        millisecond: 0,
    };
    let mut o = DateTimeFormatOptions::default();
    o.time_zone = Some(zone);
    o.time_zone_name = Some(style);
    o.tz_offset_minutes = Some(0);
    format_to_parts(lang, &when, &o)
        .unwrap()
        .last()
        .unwrap()
        .value
        .clone()
}

#[test]
fn localized_gmt_offset_forms() {
    // The long form pads the hour; UTS #35's short form does not, and drops the
    // minute field entirely when it is zero. `GMT-7` / `GMT+5:30` per ICU.
    assert_eq!(format_gmt_offset("en", 0), "GMT");
    assert_eq!(format_gmt_offset("en", -420), "GMT-07:00");
    assert_eq!(format_gmt_offset("fr", -420), "UTC\u{2212}07:00");
    // `hourFormat` is not always `+HH:mm`: `cs` uses a single `H` and `da` a dot
    // separator. Both used to render literally ("GMT+H:30").
    assert_eq!(format_gmt_offset("cs", 330), "GMT+5:30");
    assert_eq!(format_gmt_offset("da", 330), "GMT+05.30");
    assert_eq!(format_gmt_offset("fi", -480), "UTC-8.00");
}

#[cfg(all(feature = "iana-tz", feature = "tz-names-america"))]
#[test]
fn reference_table_america() {
    use TimeZoneNameStyle::*;
    let la = "America/Los_Angeles";
    // July: the zone is on daylight time, so the specific styles take the
    // daylight names. All six match V8/ICU.
    assert_eq!(name("en", la, Long, 7), "Pacific Daylight Time");
    assert_eq!(name("en", la, Short, 7), "PDT");
    assert_eq!(name("en", la, LongGeneric, 7), "Pacific Time");
    assert_eq!(name("en", la, ShortGeneric, 7), "PT");
    assert_eq!(name("en", la, ShortOffset, 7), "GMT-7");
    assert_eq!(name("en", la, LongOffset, 7), "GMT-07:00");
    // January: standard time.
    assert_eq!(name("en", la, Long, 1), "Pacific Standard Time");
    assert_eq!(name("en", la, Short, 1), "PST");

    // French has long metazone names but no short ones, so `short` falls all the
    // way through to the short localized GMT offset, and `shortGeneric` to the
    // generic location format (`regionFormat` + exemplar city).
    assert_eq!(
        name("fr", la, Long, 7),
        "heure d\u{2019}\u{e9}t\u{e9} du Pacifique nord-am\u{e9}ricain"
    );
    assert_eq!(name("fr", la, Short, 7), "UTC\u{2212}7");
    assert_eq!(
        name("fr", la, LongGeneric, 7),
        "heure du Pacifique nord-am\u{e9}ricain"
    );
    assert_eq!(name("fr", la, ShortGeneric, 7), "heure : Los Angeles");
    assert_eq!(name("fr", la, ShortOffset, 7), "UTC\u{2212}7");
}

#[cfg(all(feature = "iana-tz", feature = "tz-names-etc"))]
#[test]
fn reference_table_utc() {
    use TimeZoneNameStyle::*;
    // `UTC` is a tzdb link; it canonicalizes to `Etc/UTC`, whose *zone-level*
    // entry carries both names. There is no generic form and `Etc/…` has no
    // location, so the generic styles fall through to the GMT zero format.
    assert_eq!(name("en", "UTC", Long, 7), "Coordinated Universal Time");
    assert_eq!(name("en", "UTC", Short, 7), "UTC");
    assert_eq!(name("en", "UTC", LongGeneric, 7), "GMT");
    assert_eq!(name("en", "UTC", ShortGeneric, 7), "GMT");
    assert_eq!(name("en", "UTC", ShortOffset, 7), "GMT");
}

#[cfg(all(feature = "iana-tz", feature = "tz-names-europe"))]
#[test]
fn zone_level_names_beat_metazone() {
    use TimeZoneNameStyle::*;
    // `Europe/London` maps to the `GMT` metazone, which only has a standard
    // name; the daylight name comes from the zone's own entry. Both per ICU.
    assert_eq!(name("en", "Europe/London", Long, 1), "Greenwich Mean Time");
    assert_eq!(name("en", "Europe/London", Long, 7), "British Summer Time");
    // No generic name anywhere and the zone does observe DST, so `vvvv` falls to
    // the generic location format — with the country name, because GB has a
    // single tzdb zone.
    #[cfg(feature = "displaynames")]
    assert_eq!(
        name("en", "Europe/London", LongGeneric, 7),
        "United Kingdom Time"
    );
}

#[cfg(all(feature = "iana-tz", feature = "tz-names-asia"))]
#[test]
fn generic_falls_back_to_standard_without_dst() {
    use TimeZoneNameStyle::*;
    // The `India` metazone has only a long standard name. A zone that never
    // observes daylight time has just the one name, so it answers `vvvv` too
    // (ICU: "India Standard Time"). `v` has no short name to use and lands on
    // the generic location format instead.
    assert_eq!(name("en", "Asia/Kolkata", Long, 1), "India Standard Time");
    assert_eq!(
        name("en", "Asia/Kolkata", LongGeneric, 1),
        "India Standard Time"
    );
    #[cfg(feature = "displaynames")]
    assert_eq!(name("en", "Asia/Kolkata", ShortGeneric, 1), "India Time");
    assert_eq!(name("en", "Asia/Kolkata", Short, 1), "GMT+5:30");
}

#[cfg(all(feature = "iana-tz", feature = "tz-names-america"))]
#[test]
fn zone_links_resolve_to_the_canonical_zone() {
    use TimeZoneNameStyle::*;
    // CLDR keys these by the link (`America/Buenos_Aires`, `Asia/Calcutta`) or
    // by the modern id; either spelling must reach the same names.
    assert_eq!(
        name("en", "US/Pacific", Long, 7),
        name("en", "America/Los_Angeles", Long, 7)
    );
    assert_eq!(
        name("en", "America/Buenos_Aires", Long, 1),
        name("en", "America/Argentina/Buenos_Aires", Long, 1)
    );
    // An unknown zone has no names at all: the caller's offset is all that is
    // left, exactly as UTS #35 prescribes.
    assert_eq!(name("en", "Nowhere/Nothing", Long, 7), "GMT");
}

#[cfg(all(feature = "iana-tz", feature = "tz-names-europe"))]
#[test]
fn metazone_ranges_are_historical() {
    use TimeZoneNameStyle::*;
    let when = DateTime {
        year: 1950,
        month: 1,
        day: 15,
        hour: 12,
        minute: 0,
        second: 0,
        millisecond: 0,
    };
    let mut o = DateTimeFormatOptions::default();
    o.time_zone = Some("Europe/London");
    o.time_zone_name = Some(Long);
    o.tz_offset_minutes = Some(0);
    // Before 1971-10-31 London used the `British` metazone, for which CLDR has
    // no English names — so this is the localized GMT offset, not "Greenwich
    // Mean Time". Matches ICU.
    let v = format_to_parts("en", &when, &o)
        .unwrap()
        .last()
        .unwrap()
        .value
        .clone();
    assert_eq!(v, "GMT");
    assert_eq!(name("en", "Europe/London", Long, 1), "Greenwich Mean Time");
}

// An area that is not compiled in must degrade to the localized GMT offset
// rather than producing a wrong or empty name.
#[cfg(all(feature = "iana-tz", not(feature = "tz-names-europe")))]
#[test]
fn uncompiled_area_falls_back_to_offset() {
    assert_eq!(
        name("en", "Europe/London", TimeZoneNameStyle::Long, 1),
        "GMT"
    );
}
