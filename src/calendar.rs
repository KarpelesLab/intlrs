//! Calendar date conversions (`no_std`, no `alloc`): the Julian Day Number as a
//! pivot between the proleptic Gregorian calendar, the civil (tabular) Islamic
//! calendar, the ISO-8601 week date, and the day of week. Pure integer
//! arithmetic — no data.
//!
//! ```
//! use intl::calendar::{gregorian_to_islamic, islamic_to_gregorian, day_of_week};
//! // 2000-01-01 was a Saturday (ISO weekday 6).
//! assert_eq!(day_of_week(2000, 1, 1), 6);
//! // Round-trips through the Islamic calendar.
//! let (iy, im, id) = gregorian_to_islamic(2024, 7, 7);
//! assert_eq!(islamic_to_gregorian(iy, im, id), (2024, 7, 7));
//! ```

/// JDN of 1 Muharram 1 AH in the civil (tabular) Islamic calendar.
const ISLAMIC_EPOCH: i64 = 1_948_440;

/// The Julian Day Number of a proleptic Gregorian date.
#[must_use]
pub fn gregorian_to_jdn(year: i64, month: i64, day: i64) -> i64 {
    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;
    day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
}

/// The proleptic Gregorian `(year, month, day)` of a Julian Day Number.
#[must_use]
pub fn jdn_to_gregorian(jdn: i64) -> (i64, i64, i64) {
    let a = jdn + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - 146097 * b / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - 1461 * d / 4;
    let m = (5 * e + 2) / 153;
    let day = e - (153 * m + 2) / 5 + 1;
    let month = m + 3 - 12 * (m / 10);
    let year = 100 * b + d - 4800 + m / 10;
    (year, month, day)
}

/// The Julian Day Number of a civil (tabular) Islamic date.
#[must_use]
pub fn islamic_to_jdn(year: i64, month: i64, day: i64) -> i64 {
    // Days before `month`: months alternate 30/29 (ceil(29.5·(m−1))).
    let before = 29 * (month - 1) + month / 2;
    day + before + (year - 1) * 354 + (3 + 11 * year) / 30 + ISLAMIC_EPOCH - 1
}

/// The civil (tabular) Islamic `(year, month, day)` of a Julian Day Number.
#[must_use]
pub fn jdn_to_islamic(jdn: i64) -> (i64, i64, i64) {
    // Estimate the year, then correct using the exact forward function.
    let mut year = (30 * (jdn - ISLAMIC_EPOCH) + 10646) / 10631;
    if year < 1 {
        year = 1;
    }
    while islamic_to_jdn(year, 1, 1) > jdn {
        year -= 1;
    }
    while islamic_to_jdn(year + 1, 1, 1) <= jdn {
        year += 1;
    }
    let mut month = 1;
    while month < 12 && islamic_to_jdn(year, month + 1, 1) <= jdn {
        month += 1;
    }
    let day = jdn - islamic_to_jdn(year, month, 1) + 1;
    (year, month, day)
}

/// JDN of 1 Farvardin 1 in the Persian (Solar Hijri) calendar.
const PERSIAN_EPOCH: i64 = 1_948_321;

/// The Julian Day Number of a Persian (Solar Hijri) date (arithmetic 2820-year
/// cycle).
#[must_use]
pub fn persian_to_jdn(year: i64, month: i64, day: i64) -> i64 {
    let epbase = if year >= 0 { year - 474 } else { year - 473 };
    let epyear = 474 + epbase.rem_euclid(2820);
    let month_days = if month <= 7 {
        (month - 1) * 31
    } else {
        (month - 1) * 30 + 6
    };
    day + month_days
        + (epyear * 682 - 110) / 2816
        + (epyear - 1) * 365
        + epbase.div_euclid(2820) * 1_029_983
        + (PERSIAN_EPOCH - 1)
}

/// The Persian (Solar Hijri) `(year, month, day)` of a Julian Day Number.
#[must_use]
pub fn jdn_to_persian(jdn: i64) -> (i64, i64, i64) {
    // The Persian year of a Gregorian date is roughly `gregorian - 621`.
    let mut year = jdn_to_gregorian(jdn).0 - 621;
    while persian_to_jdn(year, 1, 1) > jdn {
        year -= 1;
    }
    while persian_to_jdn(year + 1, 1, 1) <= jdn {
        year += 1;
    }
    let mut month = 1;
    while month < 12 && persian_to_jdn(year, month + 1, 1) <= jdn {
        month += 1;
    }
    let day = jdn - persian_to_jdn(year, month, 1) + 1;
    (year, month, day)
}

/// Convert a Gregorian date to the Persian (Solar Hijri) calendar.
#[must_use]
pub fn gregorian_to_persian(year: i64, month: i64, day: i64) -> (i64, i64, i64) {
    jdn_to_persian(gregorian_to_jdn(year, month, day))
}

/// Convert a Persian (Solar Hijri) date to the Gregorian calendar.
#[must_use]
pub fn persian_to_gregorian(year: i64, month: i64, day: i64) -> (i64, i64, i64) {
    jdn_to_gregorian(persian_to_jdn(year, month, day))
}

/// Convert a Gregorian date to the civil Islamic calendar.
#[must_use]
pub fn gregorian_to_islamic(year: i64, month: i64, day: i64) -> (i64, i64, i64) {
    jdn_to_islamic(gregorian_to_jdn(year, month, day))
}

/// Convert a civil Islamic date to the Gregorian calendar.
#[must_use]
pub fn islamic_to_gregorian(year: i64, month: i64, day: i64) -> (i64, i64, i64) {
    jdn_to_gregorian(islamic_to_jdn(year, month, day))
}

/// The Japanese era and year-within-era for a Gregorian date, e.g.
/// `(2026, 6, 4)` → `("Reiwa", 8)`. Dates before the Meiji era return
/// `("CE", year)`.
///
/// ```
/// use intl::calendar::japanese_era;
/// assert_eq!(japanese_era(2026, 6, 4), ("Reiwa", 8));
/// assert_eq!(japanese_era(2019, 4, 30), ("Heisei", 31)); // day before Reiwa
/// assert_eq!(japanese_era(2019, 5, 1), ("Reiwa", 1));
/// ```
#[must_use]
pub fn japanese_era(year: i64, month: i64, day: i64) -> (&'static str, i64) {
    // Modern era boundaries (Gregorian start dates).
    const ERAS: [(i64, i64, i64, &str); 5] = [
        (1868, 10, 23, "Meiji"),
        (1912, 7, 30, "Taisho"),
        (1926, 12, 25, "Showa"),
        (1989, 1, 8, "Heisei"),
        (2019, 5, 1, "Reiwa"),
    ];
    for &(sy, sm, sd, name) in ERAS.iter().rev() {
        if (year, month, day) >= (sy, sm, sd) {
            return (name, year - sy + 1);
        }
    }
    ("CE", year)
}

/// ISO-8601 weekday of a Gregorian date: 1 = Monday … 7 = Sunday.
#[must_use]
pub fn day_of_week(year: i64, month: i64, day: i64) -> u8 {
    (gregorian_to_jdn(year, month, day).rem_euclid(7) + 1) as u8
}

/// The ISO-8601 week date of a Gregorian date: `(iso_year, iso_week, weekday)`,
/// where `weekday` is 1 = Monday … 7 = Sunday and weeks belong to the year
/// containing their Thursday.
#[must_use]
pub fn iso_week(year: i64, month: i64, day: i64) -> (i64, u8, u8) {
    let jdn = gregorian_to_jdn(year, month, day);
    let weekday = jdn.rem_euclid(7) + 1; // 1..7
                                         // The Thursday of this week determines the ISO year.
    let thursday = jdn - (weekday - 4);
    let (iso_year, _, _) = jdn_to_gregorian(thursday);
    // Week 1 is the week containing 4 January (the first Thursday's week).
    let jan4 = gregorian_to_jdn(iso_year, 1, 4);
    let jan4_weekday = jan4.rem_euclid(7) + 1;
    let week1_monday = jan4 - (jan4_weekday - 1);
    let week = ((jdn - week1_monday) / 7 + 1) as u8;
    (iso_year, week, weekday as u8)
}
