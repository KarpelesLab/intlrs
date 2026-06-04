//! Calendar conversions.
use intl::calendar::*;

#[test]
fn jdn_anchors() {
    assert_eq!(gregorian_to_jdn(2000, 1, 1), 2451545); // well-known epoch
    assert_eq!(jdn_to_gregorian(2451545), (2000, 1, 1));
    assert_eq!(islamic_to_jdn(1, 1, 1), 1948440); // 1 Muharram 1 AH
    assert_eq!(jdn_to_islamic(1948440), (1, 1, 1));
}

#[test]
fn gregorian_roundtrip() {
    // Round-trip a span of dates through JDN.
    for &(y, m, d) in &[
        (1, 1, 1),
        (1582, 10, 15),
        (1970, 1, 1),
        (2026, 6, 4),
        (9999, 12, 31),
    ] {
        let (yy, mm, dd) = jdn_to_gregorian(gregorian_to_jdn(y, m, d));
        assert_eq!((yy, mm, dd), (y, m, d));
    }
}

#[test]
fn islamic_roundtrip_and_known() {
    // Gregorian -> Islamic -> Gregorian round-trips.
    for &(y, m, d) in &[(2000, 1, 1), (2024, 7, 7), (1970, 1, 1), (2026, 6, 4)] {
        let (iy, im, id) = gregorian_to_islamic(y, m, d);
        assert!((1..=12).contains(&im) && (1..=30).contains(&id));
        assert_eq!(islamic_to_gregorian(iy, im, id), (y, m, d));
    }
    // 2024-07-07 falls at the 1445/1446 AH boundary (civil tabular epoch).
    let (iy, ..) = gregorian_to_islamic(2024, 7, 7);
    assert!(iy == 1445 || iy == 1446);
}

#[test]
fn weekdays_and_iso_week() {
    assert_eq!(day_of_week(2000, 1, 1), 6); // Saturday
    assert_eq!(day_of_week(2026, 6, 4), 4); // Thursday
                                            // 2026-01-01 is a Thursday, in ISO week 1 of 2026.
    assert_eq!(iso_week(2026, 1, 1), (2026, 1, 4));
    // 2027-01-01 is a Friday, belonging to ISO week 53 of 2026.
    assert_eq!(iso_week(2027, 1, 1).0, 2026);
}
