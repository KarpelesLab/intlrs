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

// ---- Hebrew calendar (Dershowitz–Reingold arithmetic). ----

/// Rata Die (days since proleptic Gregorian 0001-01-01) of 1 Tishrei AM 1.
const HEBREW_EPOCH_RD: i64 = -1_373_427;
/// RD → JDN offset (JDN of Gregorian 0001-01-01 is 1721426; RD 1 = that day).
const RD_TO_JDN: i64 = 1_721_425;

fn hebrew_leap(year: i64) -> bool {
    (7 * year + 1).rem_euclid(19) < 7
}
fn hebrew_year_months(year: i64) -> i64 {
    if hebrew_leap(year) {
        13
    } else {
        12
    }
}
fn hebrew_elapsed_days(year: i64) -> i64 {
    let months = (235 * year - 234).div_euclid(19);
    let parts = 12084 + 13753 * months;
    let day = 29 * months + parts.div_euclid(25920);
    if (3 * (day + 1)).rem_euclid(7) < 3 {
        day + 1
    } else {
        day
    }
}
fn hebrew_new_year_rd(year: i64) -> i64 {
    let correction = {
        let (a, b, c) = (
            hebrew_elapsed_days(year - 1),
            hebrew_elapsed_days(year),
            hebrew_elapsed_days(year + 1),
        );
        if c - b == 356 {
            2
        } else if b - a == 382 {
            1
        } else {
            0
        }
    };
    HEBREW_EPOCH_RD + hebrew_elapsed_days(year) + correction
}
fn hebrew_year_days(year: i64) -> i64 {
    hebrew_new_year_rd(year + 1) - hebrew_new_year_rd(year)
}
fn hebrew_month_days(year: i64, month: i64) -> i64 {
    match month {
        2 | 4 | 6 | 10 | 13 => 29,
        8 => {
            if matches!(hebrew_year_days(year), 355 | 385) {
                30
            } else {
                29
            }
        } // Marheshvan
        9 => {
            if matches!(hebrew_year_days(year), 353 | 383) {
                29
            } else {
                30
            }
        } // Kislev
        12 => {
            if hebrew_leap(year) {
                30
            } else {
                29
            }
        } // Adar (I)
        _ => 30, // Nisan, Sivan, Av, Tishrei, Shevat
    }
}
fn hebrew_to_rd(year: i64, month: i64, day: i64) -> i64 {
    // The Hebrew year begins at Tishrei (month 7); months 1..6 (Nisan..Elul)
    // fall after the year-number rollover.
    let mut rd = hebrew_new_year_rd(year) + day - 1;
    if month < 7 {
        for m in 7..=hebrew_year_months(year) {
            rd += hebrew_month_days(year, m);
        }
        for m in 1..month {
            rd += hebrew_month_days(year, m);
        }
    } else {
        for m in 7..month {
            rd += hebrew_month_days(year, m);
        }
    }
    rd
}

/// The Julian Day Number of a Hebrew (Anno Mundi) date.
#[must_use]
pub fn hebrew_to_jdn(year: i64, month: i64, day: i64) -> i64 {
    hebrew_to_rd(year, month, day) + RD_TO_JDN
}

/// The Hebrew `(year, month, day)` of a Julian Day Number (month 7 = Tishrei,
/// the start of the year; month 1 = Nisan).
#[must_use]
pub fn jdn_to_hebrew(jdn: i64) -> (i64, i64, i64) {
    let rd = jdn - RD_TO_JDN;
    let mut year = (rd - HEBREW_EPOCH_RD) * 98496 / 35_975_351 + 1;
    while hebrew_new_year_rd(year + 1) <= rd {
        year += 1;
    }
    while hebrew_new_year_rd(year) > rd {
        year -= 1;
    }
    let start = if rd < hebrew_to_rd(year, 1, 1) { 7 } else { 1 };
    let mut month = start;
    while rd > hebrew_to_rd(year, month, hebrew_month_days(year, month)) {
        month += 1;
    }
    let day = rd - hebrew_to_rd(year, month, 1) + 1;
    (year, month, day)
}

/// Convert a Gregorian date to the Hebrew calendar.
#[must_use]
pub fn gregorian_to_hebrew(year: i64, month: i64, day: i64) -> (i64, i64, i64) {
    jdn_to_hebrew(gregorian_to_jdn(year, month, day))
}

/// Convert a Hebrew date to the Gregorian calendar.
#[must_use]
pub fn hebrew_to_gregorian(year: i64, month: i64, day: i64) -> (i64, i64, i64) {
    jdn_to_gregorian(hebrew_to_jdn(year, month, day))
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
