//! POSIX TZ zones.
#![cfg(feature = "alloc")]
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
