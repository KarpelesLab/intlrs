//! Calendar date conversions (`no_std`, no `alloc`): the Julian Day Number as a
//! pivot between the proleptic Gregorian, civil (tabular) Islamic, Persian
//! (Solar Hijri), Hebrew, Chinese (lunisolar, 1900–2099), and Japanese-era
//! calendars, plus the ISO-8601 week date and day of week. Pure integer
//! arithmetic; only the Chinese calendar uses an embedded lunar table.
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

/// Bound for the raw `year`/`month`/`day` components of forward (date → JDN)
/// conversions. The internal arithmetic multiplies these inputs by small
/// constants (the largest effective factor is ≈ 1.7·10⁵ in the Hebrew code:
/// `13753 · months` with `months ≈ 12.4 · year`); clamping every component to
/// ±10⁹ keeps all intermediate products and sums well within `i64`
/// (≈9.2·10¹⁸), so out-of-range inputs saturate to a finite JDN instead of
/// overflowing (a debug panic) or silently wrapping (release). The bound is far
/// larger than any real-world calendar date, so normal-range results are
/// unchanged.
const FWD_LIMIT: i64 = 1_000_000_000;

#[inline]
fn clamp_component(v: i64) -> i64 {
    v.clamp(-FWD_LIMIT, FWD_LIMIT)
}

/// The Julian Day Number of a proleptic Gregorian date.
#[must_use]
pub fn gregorian_to_jdn(year: i64, month: i64, day: i64) -> i64 {
    let year = clamp_component(year);
    let month = clamp_component(month);
    let day = clamp_component(day);
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
    let year = clamp_component(year);
    let month = clamp_component(month);
    let day = clamp_component(day);
    // Days before `month`: months alternate 30/29 (ceil(29.5·(m−1))).
    let before = 29 * (month - 1) + month / 2;
    day + before + (year - 1) * 354 + (3 + 11 * year) / 30 + ISLAMIC_EPOCH - 1
}

/// The civil (tabular) Islamic `(year, month, day)` of a Julian Day Number.
#[must_use]
pub fn jdn_to_islamic(jdn: i64) -> (i64, i64, i64) {
    // Estimate the year, then correct using the exact forward function. The
    // estimate is accurate to ±1 for any in-domain JDN, so the correction loops
    // below run O(1) times. Keep the estimate itself (rather than forcing it up
    // to 1) so an extreme, pre-epoch JDN does not trigger a linear walk of up to
    // `FWD_LIMIT` steps; the loops are also bounded by `±FWD_LIMIT` as a final
    // backstop. For any real (≥ epoch) JDN the estimate is ≥ 1, so the result is
    // unchanged.
    let mut year = ((30 * (jdn - ISLAMIC_EPOCH) + 10646) / 10631).clamp(-FWD_LIMIT, FWD_LIMIT);
    while year > -FWD_LIMIT && islamic_to_jdn(year, 1, 1) > jdn {
        year -= 1;
    }
    while year < FWD_LIMIT && islamic_to_jdn(year + 1, 1, 1) <= jdn {
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
    let year = clamp_component(year);
    let month = clamp_component(month);
    let day = clamp_component(day);
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
    while year > -FWD_LIMIT && persian_to_jdn(year, 1, 1) > jdn {
        year -= 1;
    }
    while year < FWD_LIMIT && persian_to_jdn(year + 1, 1, 1) <= jdn {
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
    if hebrew_leap(year) { 13 } else { 12 }
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
    // fall after the year-number rollover. A Hebrew year has at most 13 months,
    // so cap the second accumulation loop's bound at the real month count: every
    // valid `month` (1..=13) is unaffected, while an out-of-range `month` can no
    // longer drive a billion-iteration loop.
    let mut rd = hebrew_new_year_rd(year) + day - 1;
    if month < 7 {
        for m in 7..=hebrew_year_months(year) {
            rd += hebrew_month_days(year, m);
        }
        let upper = month.min(hebrew_year_months(year) + 1);
        for m in 1..upper {
            rd += hebrew_month_days(year, m);
        }
    } else {
        let upper = month.min(hebrew_year_months(year) + 1);
        for m in 7..upper {
            rd += hebrew_month_days(year, m);
        }
    }
    rd
}

/// The Julian Day Number of a Hebrew (Anno Mundi) date.
#[must_use]
pub fn hebrew_to_jdn(year: i64, month: i64, day: i64) -> i64 {
    // Clamp before any arithmetic: `hebrew_elapsed_days` multiplies the year by
    // ≈1.7·10⁵ (`13753 · months`, `months ≈ 12.4 · year`), and `hebrew_to_rd`
    // iterates month-by-month, so an extreme `month` would otherwise spin for a
    // very long time. A Hebrew month is always 1..=13, so clamping `month` to
    // the `FWD_LIMIT` band leaves every valid month untouched while bounding the
    // loop. `year`/`day` use the same wide band as the other calendars.
    let year = clamp_component(year);
    let month = clamp_component(month);
    let day = clamp_component(day);
    hebrew_to_rd(year, month, day) + RD_TO_JDN
}

/// The Hebrew `(year, month, day)` of a Julian Day Number (month 7 = Tishrei,
/// the start of the year; month 1 = Nisan).
#[must_use]
pub fn jdn_to_hebrew(jdn: i64) -> (i64, i64, i64) {
    let rd = jdn - RD_TO_JDN;
    let mut year = ((rd - HEBREW_EPOCH_RD) * 98496 / 35_975_351 + 1).clamp(-FWD_LIMIT, FWD_LIMIT);
    while year < FWD_LIMIT && hebrew_new_year_rd(year + 1) <= rd {
        year += 1;
    }
    while year > -FWD_LIMIT && hebrew_new_year_rd(year) > rd {
        year -= 1;
    }
    let start = if rd < hebrew_to_rd(year, 1, 1) { 7 } else { 1 };
    let mut month = start;
    // Hebrew month numbers never exceed 13, so `month < 13` is a backstop that an
    // in-range `rd` reaches naturally (the loop stops earlier on real dates) but
    // that prevents an out-of-domain, clamped `rd` from spinning forever.
    while month < 13 && rd > hebrew_to_rd(year, month, hebrew_month_days(year, month)) {
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

// ---- Chinese (lunisolar) calendar, 1900–2099. ----
//
// Lunisolar conversion cannot be derived from CLDR (which carries only the month
// *names*) nor from simple arithmetic; it needs astronomical computation or a
// precomputed table. We embed the well-established lunar table for years
// 1900–2099. Each entry packs one Chinese year (which begins at its New Year):
//   * low 4 bits  — the leap-month number (0 = no leap month);
//   * bit (16−m)  — month m (1..=12) has 30 days (1) or 29 (0);
//   * bit 16      — the leap month has 30 days (1) or 29 (0).
// Anchored at the 1900 New Year = 1900-01-31 (JDN 2 415 051). Validated against
// known New-Year dates (2024-02-10, 2025-01-29, 2000-02-05, …). Source: the
// `lunardate` table (MIT), itself derived from astronomical almanac data.
const CHINESE_FIRST_YEAR: i64 = 1900;
const CHINESE_LAST_YEAR: i64 = 2099;
const CHINESE_EPOCH_JDN: i64 = 2_415_051; // 1900-01-31
#[rustfmt::skip]
const CHINESE_YEAR_INFO: [u32; 200] = [
    0x04bd8, 0x04ae0, 0x0a570, 0x054d5, 0x0d260, 0x0d950, 0x16554, 0x056a0, 0x09ad0, 0x055d2,
    0x04ae0, 0x0a5b6, 0x0a4d0, 0x0d250, 0x1d255, 0x0b540, 0x0d6a0, 0x0ada2, 0x095b0, 0x14977,
    0x04970, 0x0a4b0, 0x0b4b5, 0x06a50, 0x06d40, 0x1ab54, 0x02b60, 0x09570, 0x052f2, 0x04970,
    0x06566, 0x0d4a0, 0x0ea50, 0x06e95, 0x05ad0, 0x02b60, 0x186e3, 0x092e0, 0x1c8d7, 0x0c950,
    0x0d4a0, 0x1d8a6, 0x0b550, 0x056a0, 0x1a5b4, 0x025d0, 0x092d0, 0x0d2b2, 0x0a950, 0x0b557,
    0x06ca0, 0x0b550, 0x15355, 0x04da0, 0x0a5d0, 0x14573, 0x052b0, 0x0a9a8, 0x0e950, 0x06aa0,
    0x0aea6, 0x0ab50, 0x04b60, 0x0aae4, 0x0a570, 0x05260, 0x0f263, 0x0d950, 0x05b57, 0x056a0,
    0x096d0, 0x04dd5, 0x04ad0, 0x0a4d0, 0x0d4d4, 0x0d250, 0x0d558, 0x0b540, 0x0b5a0, 0x195a6,
    0x095b0, 0x049b0, 0x0a974, 0x0a4b0, 0x0b27a, 0x06a50, 0x06d40, 0x0af46, 0x0ab60, 0x09570,
    0x04af5, 0x04970, 0x064b0, 0x074a3, 0x0ea50, 0x06b58, 0x05ac0, 0x0ab60, 0x096d5, 0x092e0,
    0x0c960, 0x0d954, 0x0d4a0, 0x0da50, 0x07552, 0x056a0, 0x0abb7, 0x025d0, 0x092d0, 0x0cab5,
    0x0a950, 0x0b4a0, 0x0baa4, 0x0ad50, 0x055d9, 0x04ba0, 0x0a5b0, 0x15176, 0x052b0, 0x0a930,
    0x07954, 0x06aa0, 0x0ad50, 0x05b52, 0x04b60, 0x0a6e6, 0x0a4e0, 0x0d260, 0x0ea65, 0x0d530,
    0x05aa0, 0x076a3, 0x096d0, 0x04afb, 0x04ad0, 0x0a4d0, 0x1d0b6, 0x0d250, 0x0d520, 0x0dd45,
    0x0b5a0, 0x056d0, 0x055b2, 0x049b0, 0x0a577, 0x0a4b0, 0x0aa50, 0x1b255, 0x06d20, 0x0ada0,
    0x14b63, 0x09370, 0x049f8, 0x04970, 0x064b0, 0x168a6, 0x0ea50, 0x06aa0, 0x1a6c4, 0x0aae0,
    0x092e0, 0x0d2e3, 0x0c960, 0x0d557, 0x0d4a0, 0x0da50, 0x05d55, 0x056a0, 0x0a6d0, 0x055d4,
    0x052d0, 0x0a9b8, 0x0a950, 0x0b4a0, 0x0b6a6, 0x0ad50, 0x055a0, 0x0aba4, 0x0a5b0, 0x052b0,
    0x0b273, 0x06930, 0x07337, 0x06aa0, 0x0ad50, 0x14b55, 0x04b60, 0x0a570, 0x054e4, 0x0d160,
    0x0e968, 0x0d520, 0x0daa0, 0x16aa6, 0x056d0, 0x04ae0, 0x0a9d4, 0x0a2d0, 0x0d150, 0x0f252,
];

fn cn_info(year: i64) -> Option<u32> {
    if (CHINESE_FIRST_YEAR..=CHINESE_LAST_YEAR).contains(&year) {
        Some(CHINESE_YEAR_INFO[(year - CHINESE_FIRST_YEAR) as usize])
    } else {
        None
    }
}
fn cn_month_days(info: u32, month: i64) -> i64 {
    ((info >> (16 - month)) & 1) as i64 + 29
}
fn cn_leap_days(info: u32) -> i64 {
    ((info >> 16) & 1) as i64 + 29
}
fn cn_year_days(info: u32) -> i64 {
    let leap = (info % 16) as i64;
    let mut sum = 0;
    let mut m = 1;
    while m <= 12 {
        sum += cn_month_days(info, m);
        if leap == m {
            sum += cn_leap_days(info);
        }
        m += 1;
    }
    sum
}

/// The Julian Day Number of a Chinese (lunisolar) date. `month` is 1–12; set
/// `leap` for the intercalary month that follows `month`. Returns `None` for a
/// year outside the supported range (1900–2099) or a `leap` month that does not
/// occur that year.
#[must_use]
pub fn chinese_to_jdn(year: i64, month: i64, day: i64, leap: bool) -> Option<i64> {
    let info = cn_info(year)?;
    if leap && (info % 16) as i64 != month {
        return None;
    }
    // `year` is already range-checked by `cn_info`, but `day` is added to the
    // accumulated JDN unguarded (`jdn + day - 1`); an extreme `day` would
    // overflow. A valid Chinese day is 1..=30, so clamping to the `FWD_LIMIT`
    // band leaves every real date unchanged while preventing the overflow.
    let day = clamp_component(day);
    let mut jdn = CHINESE_EPOCH_JDN;
    let mut y = CHINESE_FIRST_YEAR;
    while y < year {
        jdn += cn_year_days(cn_info(y)?);
        y += 1;
    }
    let leap_m = (info % 16) as i64;
    let mut m = 1;
    while m <= 12 {
        if m == month && !leap {
            return Some(jdn + day - 1);
        }
        jdn += cn_month_days(info, m);
        if leap_m == m {
            if m == month && leap {
                return Some(jdn + day - 1);
            }
            jdn += cn_leap_days(info);
        }
        m += 1;
    }
    None
}

/// The Chinese `(year, month, day, is_leap_month)` of a Julian Day Number, or
/// `None` if it falls outside the supported range (Chinese years 1900–2099).
#[must_use]
pub fn jdn_to_chinese(jdn: i64) -> Option<(i64, i64, i64, bool)> {
    let mut offset = jdn - CHINESE_EPOCH_JDN;
    if offset < 0 {
        return None;
    }
    let mut year = CHINESE_FIRST_YEAR;
    loop {
        let yd = cn_year_days(cn_info(year)?);
        if offset < yd {
            break;
        }
        offset -= yd;
        year += 1;
    }
    let info = cn_info(year)?;
    let leap_m = (info % 16) as i64;
    let mut m = 1;
    while m <= 12 {
        let d = cn_month_days(info, m);
        if offset < d {
            return Some((year, m, offset + 1, false));
        }
        offset -= d;
        if leap_m == m {
            let dl = cn_leap_days(info);
            if offset < dl {
                return Some((year, m, offset + 1, true));
            }
            offset -= dl;
        }
        m += 1;
    }
    None
}

/// Convert a Gregorian date to the Chinese calendar (`None` if out of range).
#[must_use]
pub fn gregorian_to_chinese(year: i64, month: i64, day: i64) -> Option<(i64, i64, i64, bool)> {
    jdn_to_chinese(gregorian_to_jdn(year, month, day))
}

/// Convert a Chinese date to the Gregorian calendar (`None` if out of range).
#[must_use]
pub fn chinese_to_gregorian(
    year: i64,
    month: i64,
    day: i64,
    leap: bool,
) -> Option<(i64, i64, i64)> {
    Some(jdn_to_gregorian(chinese_to_jdn(year, month, day, leap)?))
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
