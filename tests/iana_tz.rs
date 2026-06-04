//! Full IANA time-zone lookups (the `iana-tz` feature).
#![cfg(feature = "iana-tz")]
use intl::datetime::DateTime;
use intl::timezone::{load_zone, zone_names};

fn dt(m: u8, d: u8) -> DateTime {
    DateTime {
        year: 2026,
        month: m,
        day: d,
        hour: 12,
        minute: 0,
        second: 0,
    }
}

#[test]
fn new_york_dst() {
    let ny = load_zone("America/New_York").expect("zone exists");
    assert_eq!(ny.offset_for_local(&dt(7, 1)), -4 * 3600); // EDT (summer)
    assert_eq!(ny.offset_for_local(&dt(1, 1)), -5 * 3600); // EST (winter)
}

#[test]
fn no_dst_zones() {
    let kolkata = load_zone("Asia/Kolkata").expect("zone");
    assert_eq!(kolkata.offset_for_local(&dt(7, 1)), 5 * 3600 + 30 * 60); // +05:30
    assert_eq!(kolkata.offset_for_local(&dt(1, 1)), 5 * 3600 + 30 * 60);
    // UTC.
    let utc = load_zone("UTC")
        .or_else(|| load_zone("Etc/UTC"))
        .expect("utc");
    assert_eq!(utc.offset_for_local(&dt(6, 4)), 0);
}

#[test]
fn unix_lookups_and_names() {
    let ny = load_zone("America/New_York").unwrap();
    // 2021-07-01 12:00 UTC = unix 1625140800 -> EDT.
    assert_eq!(ny.offset_at(1625140800), -4 * 3600);
    assert!(ny.is_dst_at(1625140800));
    assert_eq!(ny.abbrev_at(1625140800), "EDT");
    // Round-trip a UTC instant to local and back is consistent.
    let local = ny.to_local(1625140800);
    assert_eq!(local.hour, 8); // 12:00 UTC - 4h = 08:00 EDT
                               // The database has the usual canonical zones.
    let count = zone_names().count();
    assert!(count > 300, "expected many zones, got {count}");
    assert!(load_zone("Bogus/Nowhere").is_none());
}
