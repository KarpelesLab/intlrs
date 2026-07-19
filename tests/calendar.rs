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
fn umalqura_known_and_roundtrip() {
    // Known official Umm al-Qura (Saudi) conversions, both directions.
    for &(iy, im, id, gy, gm, gd) in &[
        (1445, 9, 1, 2024, 3, 11),  // 1 Ramadan 1445
        (1445, 10, 1, 2024, 4, 10), // 1 Shawwal 1445 (Eid al-Fitr)
        (1446, 1, 1, 2024, 7, 7),   // 1 Muharram 1446
        (1443, 9, 1, 2022, 4, 2),   // 1 Ramadan 1443
        (1400, 1, 1, 1979, 11, 21), // 1 Muharram 1400
        (1420, 1, 1, 1999, 4, 17),  // 1 Muharram 1420
    ] {
        assert_eq!(umalqura_to_gregorian(iy, im, id), (gy, gm, gd));
        assert_eq!(gregorian_to_umalqura(gy, gm, gd), (iy, im, id));
    }
    // The embedded epoch: 1 Muharram 1300 AH = 1882-11-12.
    assert_eq!(umalqura_to_gregorian(1300, 1, 1), (1882, 11, 12));

    // Round-trip every first-of-month across the tabulated range, and check
    // month lengths (29 or 30) sum to the year length (354 or 355).
    for iy in 1300..=1600 {
        let mut sum = 0;
        for im in 1..=12 {
            let start = umalqura_to_jdn(iy, im, 1);
            assert_eq!(jdn_to_umalqura(start), (iy, im, 1));
            let next = if im < 12 {
                umalqura_to_jdn(iy, im + 1, 1)
            } else {
                umalqura_to_jdn(iy + 1, 1, 1)
            };
            let ml = next - start;
            assert!(ml == 29 || ml == 30, "AH {iy}-{im} has {ml} days");
            // Round-trip the last day of the month too.
            assert_eq!(jdn_to_umalqura(next - 1), (iy, im, ml));
            sum += ml;
        }
        let ylen = umalqura_to_jdn(iy + 1, 1, 1) - umalqura_to_jdn(iy, 1, 1);
        assert!(ylen == 354 || ylen == 355, "AH {iy} has {ylen} days");
        assert_eq!(sum, ylen);
    }

    // Outside the tabulated range, Umm al-Qura falls back to the civil calendar.
    for &(iy, im, id) in &[(1200, 1, 1), (1299, 12, 29), (1601, 1, 1), (1700, 6, 15)] {
        assert_eq!(umalqura_to_jdn(iy, im, id), islamic_to_jdn(iy, im, id));
    }
    // And the reverse falls back for JDNs outside the range.
    let far_past = islamic_to_jdn(1200, 1, 1);
    assert_eq!(jdn_to_umalqura(far_past), jdn_to_islamic(far_past));
    let far_future = islamic_to_jdn(1700, 1, 1);
    assert_eq!(jdn_to_umalqura(far_future), jdn_to_islamic(far_future));
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

#[test]
fn japanese() {
    use intl::calendar::japanese_era;
    assert_eq!(japanese_era(2026, 6, 4), ("Reiwa", 8));
    assert_eq!(japanese_era(2019, 5, 1), ("Reiwa", 1));
    assert_eq!(japanese_era(2019, 4, 30), ("Heisei", 31));
    assert_eq!(japanese_era(1989, 1, 8), ("Heisei", 1));
    assert_eq!(japanese_era(1989, 1, 7), ("Showa", 64));
    assert_eq!(japanese_era(1868, 10, 23), ("Meiji", 1));
    assert_eq!(japanese_era(1700, 1, 1), ("CE", 1700)); // pre-Meiji
}

#[test]
fn persian() {
    use intl::calendar::*;
    // Nowruz 1404 = 20 March 2025 (vernal equinox).
    assert_eq!(gregorian_to_persian(2025, 3, 20), (1404, 1, 1));
    assert_eq!(persian_to_gregorian(1404, 1, 1), (2025, 3, 20));
    assert_eq!(persian_to_jdn(1, 1, 1), 1948321); // epoch
    // Round-trips.
    for &(y, m, d) in &[(2000, 1, 1), (1970, 1, 1), (2026, 6, 4), (2025, 12, 31)] {
        let (py, pm, pd) = gregorian_to_persian(y, m, d);
        assert!((1..=12).contains(&pm) && (1..=31).contains(&pd));
        assert_eq!(persian_to_gregorian(py, pm, pd), (y, m, d));
    }
}

#[test]
fn hebrew() {
    use intl::calendar::*;
    // 1 Tishrei 5785 (Rosh Hashanah 5785) = 3 October 2024.
    assert_eq!(gregorian_to_hebrew(2024, 10, 3), (5785, 7, 1));
    assert_eq!(hebrew_to_gregorian(5785, 7, 1), (2024, 10, 3));
    // Round-trips across a span of Gregorian dates.
    for &(y, m, d) in &[(2000, 1, 1), (1970, 1, 1), (2026, 6, 4), (2025, 3, 20)] {
        let (hy, hm, hd) = gregorian_to_hebrew(y, m, d);
        assert!((1..=13).contains(&hm) && (1..=30).contains(&hd));
        assert_eq!(hebrew_to_gregorian(hy, hm, hd), (y, m, d));
    }
}

#[test]
fn forward_extremes_do_not_panic() {
    use intl::calendar::*;
    // These inputs previously overflowed the internal integer arithmetic and
    // panicked in debug builds ("attempt to multiply/add with overflow"). After
    // clamping the components they must return a finite value without panicking.
    for &(y, m, d) in &[
        (i64::MAX, 1, 1),
        (2026, i64::MAX, 1),
        (2026, 1, i64::MAX),
        (i64::MIN, 1, 1),
        (2026, i64::MIN, 1),
        (i64::MIN, i64::MIN, i64::MIN),
        (i64::MAX, i64::MAX, i64::MAX),
    ] {
        // Each forward (date -> JDN) function must not panic.
        let _ = gregorian_to_jdn(y, m, d);
        let _ = islamic_to_jdn(y, m, d);
        let _ = persian_to_jdn(y, m, d);
        let _ = hebrew_to_jdn(y, m, d);
        // High-level wrappers and weekday/iso helpers too.
        let _ = gregorian_to_islamic(y, m, d);
        let _ = gregorian_to_persian(y, m, d);
        let _ = gregorian_to_hebrew(y, m, d);
        let _ = day_of_week(y, m, d);
        let _ = iso_week(y, m, d);
        let _ = japanese_era(y, m, d);
        // Chinese forward is range-checked and returns None out of range.
        let _ = gregorian_to_chinese(y, m, d);
        let _ = chinese_to_jdn(y, m, d, false);
    }

    // Normal in-range results must remain byte-for-byte identical to before.
    assert_eq!(gregorian_to_jdn(2000, 1, 1), 2451545);
    assert_eq!(islamic_to_jdn(1, 1, 1), 1948440);
    assert_eq!(persian_to_jdn(1, 1, 1), 1948321);
    assert_eq!(gregorian_to_hebrew(2024, 10, 3), (5785, 7, 1));
}

#[test]
fn chinese() {
    use intl::calendar::*;
    // Chinese New Year anchors (lunar 1/1) -> Gregorian.
    assert_eq!(chinese_to_gregorian(2024, 1, 1, false), Some((2024, 2, 10)));
    assert_eq!(chinese_to_gregorian(2025, 1, 1, false), Some((2025, 1, 29)));
    assert_eq!(chinese_to_gregorian(2000, 1, 1, false), Some((2000, 2, 5)));
    // Gregorian -> Chinese.
    assert_eq!(gregorian_to_chinese(2024, 2, 10), Some((2024, 1, 1, false)));
    // A leap month: Chinese year 2023 had a leap 2nd month.
    assert_eq!(gregorian_to_chinese(2023, 1, 22), Some((2023, 1, 1, false))); // CNY 2023
    let (_, _, _, _) = gregorian_to_chinese(2023, 4, 1).unwrap();
    // Round-trips across the supported range.
    for &(y, m, d) in &[(1950, 6, 1), (2000, 1, 1), (2024, 2, 10), (2099, 12, 31)] {
        let c = gregorian_to_chinese(y, m, d).unwrap();
        assert_eq!(chinese_to_gregorian(c.0, c.1, c.2, c.3), Some((y, m, d)));
    }
    // Out of range -> None (no panic).
    assert_eq!(gregorian_to_chinese(1800, 1, 1), None);
    assert_eq!(gregorian_to_chinese(2200, 1, 1), None);
    assert_eq!(chinese_to_jdn(3000, 1, 1, false), None);
}
