//! Calendar date conversions (`no_std`, no `alloc`): the Julian Day Number as a
//! pivot between the proleptic Gregorian, civil (tabular) Islamic, Umm al-Qura
//! (Saudi) Islamic, Persian (Solar Hijri), Hebrew, Chinese (lunisolar,
//! 1800–2200), Korean dangi (lunisolar, 1800–2200), and Japanese-era calendars,
//! plus the ISO-8601 week date and day of week. Pure integer arithmetic; the
//! Chinese, dangi, and Umm al-Qura calendars use an embedded month-length table,
//! and the Persian calendar embeds ICU's leap-year correction set.
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

// ---- Persian (Solar Hijri) calendar, ported verbatim from ICU4C. ----
//
// The Persian civil calendar is astronomical: 1 Farvardin is the day whose
// midnight (Tehran, 52.5°E meridian) is nearest the March equinox. That rule
// cannot be reproduced with float-free integer arithmetic, and the older
// arithmetic 2820-year (Birashk) cycle used here diverged from it — and from
// ICU — by a day in some modern years (e.g. Nowruz 1404 AP is 2025-03-21, not
// the 2025-03-20 the Birashk cycle produced). ICU4C solves this with a
// *calibrated* closed form: the arithmetic 33-year leap rule
// `(year·25 + 11) mod 33 < 8`, corrected by a fixed set of years where that
// rule disagrees with the astronomical calendar. We port that closed form
// exactly (no astronomy, no float, pure integers), so month starts / Nowruz
// match ICU — and the official Iranian calendar — across the modern era.
//
// `PERSIAN_NONLEAP` is ICU's `nonLeapYears`, verbatim from ICU4C
// `icu4c/source/i18n/persncal.cpp`: each listed Persian year is forced
// *non-leap* (the bare arithmetic rule would mark it leap), and the year
// immediately after each is forced leap. It spans Persian years 1502..=2987 AP;
// outside that span the bare arithmetic rule applies, exactly as ICU does.

/// JDN of 1 Farvardin 1 in the Persian (Solar Hijri) calendar (ICU's
/// `PERSIAN_EPOCH`; `persian_to_jdn(1, 1, 1) == 1_948_320`).
const PERSIAN_EPOCH: i64 = 1_948_320;

/// First year of ICU's leap-correction set (`PERSIAN_NONLEAP[0]`).
const PERSIAN_MIN_CORRECTION: i64 = 1502;

#[rustfmt::skip]
const PERSIAN_NONLEAP: [i64; 78] = [
    1502, 1601, 1634, 1667, 1700, 1733, 1766, 1799, 1832, 1865, 1898, 1931, 1964, 1997, 2030, 2059,
    2063, 2096, 2129, 2158, 2162, 2191, 2195, 2224, 2228, 2257, 2261, 2290, 2294, 2323, 2327, 2356,
    2360, 2389, 2393, 2422, 2426, 2455, 2459, 2488, 2492, 2521, 2525, 2554, 2558, 2587, 2591, 2620,
    2624, 2653, 2657, 2686, 2690, 2719, 2723, 2748, 2752, 2756, 2781, 2785, 2789, 2818, 2822, 2847,
    2851, 2855, 2880, 2884, 2888, 2913, 2917, 2921, 2946, 2950, 2954, 2979, 2983, 2987,
];

/// Whether `year` is in ICU's forced-non-leap correction set.
fn persian_forced_nonleap(year: i64) -> bool {
    year >= PERSIAN_MIN_CORRECTION && PERSIAN_NONLEAP.contains(&year)
}

/// Days elapsed before `month` (1..=12) within a Persian year: months 1..=6 are
/// 31 days, 7..=11 are 30, month 12 is 29 (30 in leap years).
fn persian_days_before_month(month: i64) -> i64 {
    if month <= 7 {
        (month - 1) * 31
    } else {
        (month - 1) * 30 + 6
    }
}

/// Julian day (Persian-epoch relative) of 1 Farvardin of `year`: ICU's
/// `firstJulianOfYear`. 365 days per prior year plus the arithmetic leap days
/// `floor((8·year + 21) / 33)`, then the astronomical correction.
fn persian_first_julian_of_year(year: i64) -> i64 {
    let mut jd = 365 * (year - 1) + (8 * year + 21).div_euclid(33);
    if persian_forced_nonleap(year - 1) {
        jd -= 1;
    }
    jd
}

/// The Julian Day Number of a Persian (Solar Hijri) date. Ported verbatim from
/// ICU4C `PersianCalendar::handleComputeMonthStart` (calibrated arithmetic
/// 33-year leap rule), so month starts match ICU exactly.
#[must_use]
pub fn persian_to_jdn(year: i64, month: i64, day: i64) -> i64 {
    let year = clamp_component(year);
    let month = clamp_component(month);
    let day = clamp_component(day);
    PERSIAN_EPOCH - 1 + persian_first_julian_of_year(year) + persian_days_before_month(month) + day
}

/// The Persian (Solar Hijri) `(year, month, day)` of a Julian Day Number.
/// Ported verbatim from ICU4C `PersianCalendar::handleComputeFields`.
#[must_use]
pub fn jdn_to_persian(jdn: i64) -> (i64, i64, i64) {
    let days_since_epoch = jdn - PERSIAN_EPOCH;
    let mut year = (33 * days_since_epoch + 3).div_euclid(12053) + 1;
    let mut day_of_year = days_since_epoch - persian_first_julian_of_year(year); // 0-based
    // A forced-non-leap year has 365 days: its would-be day 366 is 1 Farvardin
    // of the (forced-leap) next year.
    if day_of_year == 365 && persian_forced_nonleap(year) {
        year += 1;
        day_of_year = 0;
    }
    let month = if day_of_year < 216 {
        day_of_year / 31 + 1
    } else {
        (day_of_year - 6) / 30 + 1
    };
    let day = day_of_year + 1 - persian_days_before_month(month);
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

// ---- Umm al-Qura (Saudi) Islamic calendar, AH 1300–1600. ----
//
// The Umm al-Qura calendar is the official calendar of Saudi Arabia. Unlike the
// civil (tabular) Islamic calendar above, its month lengths are not a fixed
// arithmetic rule but a published table (astronomical new-moon calculations,
// then codified). We embed the same month-length table ICU4C ships. Each entry
// packs one Hijri year's 12 months, low 12 bits, one bit per month:
//   * bit (12−m) — month m (1..=12) has 30 days (1) or 29 (0).
// So bit 11 is Muharram (month 1) and bit 0 is Dhu al-Hijja (month 12); there is
// no leap month. Anchored at 1 Muharram 1300 AH = 1882-11-12 (JDN 2 408 762).
// Outside the tabulated range (AH 1300–1600) the conversion falls back to the
// civil tabular Islamic calendar, exactly as ICU4C does. Validated against known
// Saudi dates (1 Ramadan 1445 → 2024-03-11, 1 Shawwal 1445 → 2024-04-10,
// 1 Muharram 1446 → 2024-07-07, 1 Ramadan 1443 → 2022-04-02, …). Source: ICU4C
// `icu4c/source/i18n/islamcal.cpp` (`UMALQURA_MONTHLENGTH`, `UMALQURA_YEAR_START`
// = 1300, `UMALQURA_YEAR_END` = 1600).
const UMALQURA_FIRST_YEAR: i64 = 1300;
const UMALQURA_LAST_YEAR: i64 = 1600;
const UMALQURA_EPOCH_JDN: i64 = 2_408_762; // 1 Muharram 1300 AH = 1882-11-12
#[rustfmt::skip]
const UMALQURA_MONTH_LENGTH: [u16; 301] = [
    0xAAA, 0xD54, 0xEC9, 0x6D4, 0x6EA, 0x36C, 0xAAD, 0x555, 0x6A9, 0x792,
    0xBA9, 0x5D4, 0xADA, 0x55C, 0xD2D, 0x695, 0x74A, 0xB54, 0xB6A, 0x5AD,
    0x4AE, 0xA4F, 0x517, 0x68B, 0x6A5, 0xAD5, 0x2D6, 0x95B, 0x49D, 0xA4D,
    0xD26, 0xD95, 0x5AC, 0x9B6, 0x2BA, 0xA5B, 0x52B, 0xA95, 0x6CA, 0xAE9,
    0x2F4, 0x976, 0x2B6, 0x956, 0xACA, 0xBA4, 0xBD2, 0x5D9, 0x2DC, 0x96D,
    0x54D, 0xAA5, 0xB52, 0xBA5, 0x5B4, 0x9B6, 0x557, 0x297, 0x54B, 0x6A3,
    0x752, 0xB65, 0x56A, 0xAAB, 0x52B, 0xC95, 0xD4A, 0xDA5, 0x5CA, 0xAD6,
    0x957, 0x4AB, 0x94B, 0xAA5, 0xB52, 0xB6A, 0x575, 0x276, 0x8B7, 0x45B,
    0x555, 0x5A9, 0x5B4, 0x9DA, 0x4DD, 0x26E, 0x936, 0xAAA, 0xD54, 0xDB2,
    0x5D5, 0x2DA, 0x95B, 0x4AB, 0xA55, 0xB49, 0xB64, 0xB71, 0x5B4, 0xAB5,
    0xA55, 0xD25, 0xE92, 0xEC9, 0x6D4, 0xAE9, 0x96B, 0x4AB, 0xA93, 0xD49,
    0xDA4, 0xDB2, 0xAB9, 0x4BA, 0xA5B, 0x52B, 0xA95, 0xB2A, 0xB55, 0x55C,
    0x4BD, 0x23D, 0x91D, 0xA95, 0xB4A, 0xB5A, 0x56D, 0x2B6, 0x93B, 0x49B,
    0x655, 0x6A9, 0x754, 0xB6A, 0x56C, 0xAAD, 0x555, 0xB29, 0xB92, 0xBA9,
    0x5D4, 0xADA, 0x55A, 0xAAB, 0x595, 0x749, 0x764, 0xBAA, 0x5B5, 0x2B6,
    0xA56, 0xE4D, 0xB25, 0xB52, 0xB6A, 0x5AD, 0x2AE, 0x92F, 0x497, 0x64B,
    0x6A5, 0x6AC, 0xAD6, 0x55D, 0x49D, 0xA4D, 0xD16, 0xD95, 0x5AA, 0x5B5,
    0x2DA, 0x95B, 0x4AD, 0x595, 0x6CA, 0x6E4, 0xAEA, 0x4F5, 0x2B6, 0x956,
    0xAAA, 0xB54, 0xBD2, 0x5D9, 0x2EA, 0x96D, 0x4AD, 0xA95, 0xB4A, 0xBA5,
    0x5B2, 0x9B5, 0x4D6, 0xA97, 0x547, 0x693, 0x749, 0xB55, 0x56A, 0xA6B,
    0x52B, 0xA8B, 0xD46, 0xDA3, 0x5CA, 0xAD6, 0x4DB, 0x26B, 0x94B, 0xAA5,
    0xB52, 0xB69, 0x575, 0x176, 0x8B7, 0x25B, 0x52B, 0x565, 0x5B4, 0x9DA,
    0x4ED, 0x16D, 0x8B6, 0xAA6, 0xD52, 0xDA9, 0x5D4, 0xADA, 0x95B, 0x4AB,
    0x653, 0x729, 0x762, 0xBA9, 0x5B2, 0xAB5, 0x555, 0xB25, 0xD92, 0xEC9,
    0x6D2, 0xAE9, 0x56B, 0x4AB, 0xA55, 0xD29, 0xD54, 0xDAA, 0x9B5, 0x4BA,
    0xA3B, 0x49B, 0xA4D, 0xAAA, 0xAD5, 0x2DA, 0x95D, 0x45E, 0xA2E, 0xC9A,
    0xD55, 0x6B2, 0x6B9, 0x4BA, 0xA5D, 0x52D, 0xA95, 0xB52, 0xBA8, 0xBB4,
    0x5B9, 0x2DA, 0x95A, 0xB4A, 0xDA4, 0xED1, 0x6E8, 0xB6A, 0x56D, 0x535,
    0x695, 0xD4A, 0xDA8, 0xDD4, 0x6DA, 0x55B, 0x29D, 0x62B, 0xB15, 0xB4A,
    0xB95, 0x5AA, 0xAAE, 0x92E, 0xC8F, 0x527, 0x695, 0x6AA, 0xAD6, 0x55D,
    0x29D,
];

/// The month-length bit pattern for a tabulated Umm al-Qura year, or `None` when
/// the year is outside the embedded range (AH 1300–1600).
fn umq_info(year: i64) -> Option<u16> {
    if (UMALQURA_FIRST_YEAR..=UMALQURA_LAST_YEAR).contains(&year) {
        Some(UMALQURA_MONTH_LENGTH[(year - UMALQURA_FIRST_YEAR) as usize])
    } else {
        None
    }
}
/// Days in `month` (1..=12) of a tabulated year: 30 if the month's bit is set,
/// else 29.
fn umq_month_days(info: u16, month: i64) -> i64 {
    ((info >> (12 - month)) & 1) as i64 + 29
}
/// Total days in a tabulated Umm al-Qura year (sum of its 12 month lengths).
fn umq_year_days(info: u16) -> i64 {
    let mut sum = 0;
    let mut m = 1;
    while m <= 12 {
        sum += umq_month_days(info, m);
        m += 1;
    }
    sum
}

/// The Julian Day Number of an Umm al-Qura (Saudi) Islamic date. Within the
/// tabulated range (AH 1300–1600) the embedded month-length table is used;
/// outside it the conversion falls back to the civil (tabular) Islamic calendar.
#[must_use]
pub fn umalqura_to_jdn(year: i64, month: i64, day: i64) -> i64 {
    let info = match umq_info(year) {
        Some(info) => info,
        None => return islamic_to_jdn(year, month, day),
    };
    // `year` is range-checked by `umq_info`, but `day` is added unguarded
    // (`jdn + day - 1`); a valid Umm al-Qura day is 1..=30, so clamping to the
    // `FWD_LIMIT` band leaves every real date unchanged while preventing an
    // extreme `day` from overflowing.
    let day = clamp_component(day);
    let mut jdn = UMALQURA_EPOCH_JDN;
    let mut y = UMALQURA_FIRST_YEAR;
    while y < year {
        jdn += umq_year_days(UMALQURA_MONTH_LENGTH[(y - UMALQURA_FIRST_YEAR) as usize]);
        y += 1;
    }
    let mut m = 1;
    while m < month && m <= 12 {
        jdn += umq_month_days(info, m);
        m += 1;
    }
    jdn + day - 1
}

/// The Umm al-Qura (Saudi) Islamic `(year, month, day)` of a Julian Day Number.
/// JDNs outside the tabulated range (AH 1300–1600) fall back to the civil
/// (tabular) Islamic calendar.
#[must_use]
pub fn jdn_to_umalqura(jdn: i64) -> (i64, i64, i64) {
    let mut offset = jdn - UMALQURA_EPOCH_JDN;
    if offset < 0 {
        return jdn_to_islamic(jdn);
    }
    let mut year = UMALQURA_FIRST_YEAR;
    loop {
        if year > UMALQURA_LAST_YEAR {
            return jdn_to_islamic(jdn);
        }
        let yd = umq_year_days(UMALQURA_MONTH_LENGTH[(year - UMALQURA_FIRST_YEAR) as usize]);
        if offset < yd {
            break;
        }
        offset -= yd;
        year += 1;
    }
    let info = UMALQURA_MONTH_LENGTH[(year - UMALQURA_FIRST_YEAR) as usize];
    let mut month = 1;
    while month < 12 && offset >= umq_month_days(info, month) {
        offset -= umq_month_days(info, month);
        month += 1;
    }
    (year, month, offset + 1)
}

/// Convert a Gregorian date to the Umm al-Qura (Saudi) Islamic calendar.
#[must_use]
pub fn gregorian_to_umalqura(year: i64, month: i64, day: i64) -> (i64, i64, i64) {
    jdn_to_umalqura(gregorian_to_jdn(year, month, day))
}

/// Convert an Umm al-Qura (Saudi) Islamic date to the Gregorian calendar.
#[must_use]
pub fn umalqura_to_gregorian(year: i64, month: i64, day: i64) -> (i64, i64, i64) {
    jdn_to_gregorian(umalqura_to_jdn(year, month, day))
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

// ---- Chinese (lunisolar) and Korean dangi (lunisolar) calendars, 1800–2200. ----
//
// Lunisolar conversion cannot be derived from CLDR (which carries only the month
// *names*) nor from simple arithmetic; it needs astronomical computation or a
// precomputed table. Each entry packs one lunisolar year (which begins at its
// New Year):
//   * low 4 bits  — the leap-month number (0 = no leap month);
//   * bit (16−m)  — month m (1..=12) has 30 days (1) or 29 (0);
//   * bit 16      — the leap month has 30 days (1) or 29 (0).
// The two tables share this format and the `ls_*` helpers below, differing only
// in table, first year, and epoch.
//
// The Chinese and dangi calendars use the SAME lunisolar rules — a year runs
// between the two new moons that bracket the winter solstice (month 11 always
// contains the solstice); a 13-month year inserts a leap month, the first month
// with no major solar term (zhongqi) — but evaluate them at DIFFERENT meridians:
// China at UTC+8, Korea at its historical offsets (ICU `DangiCalendar`: UTC+8
// through 1911 — save an ad-hoc UTC+7 in 1897 — then UTC+9 from 1912). That one
// meridian hour is why dangi's New Year / leap-month placement diverges from
// Chinese in some years (e.g. 1997: Chinese New Year 1997-02-07, dangi Seollal
// 1997-02-08).
//
// The tables were produced OFFLINE by a throwaway astronomical generator that
// ports Jean Meeus, "Astronomical Algorithms" (new-moon conjunction, apparent
// solar longitude) and applies ICU's ChineseCalendar leap rule at each meridian;
// the runtime here is pure integer table lookup. For the Chinese years 1900–2099
// the table is the previously-committed almanac (Hong Kong Observatory) data,
// kept verbatim: HKO-precise ephemerides resolve ~7 new moons that fall within
// minutes of local midnight differently from any independent truncated model,
// so those years are trusted to HKO rather than regenerated. Outside 1900–2099
// (Chinese) and across the whole span (dangi) the Meeus generator is used; its
// New-Year dates match ICU at every sampled anchor from 1800 to 2200, and the
// 1900 and 2100 seams line up exactly with the HKO block. Validated New Years:
// Chinese 2000-02-05, 2024-02-10, 2025-01-29, 1850-02-12, 2150-01-29.
const CHINESE_FIRST_YEAR: i64 = 1800; // last year 2200 = first + table length − 1
const CHINESE_EPOCH_JDN: i64 = 2_378_521; // New Year 1800 = 1800-01-25
#[rustfmt::skip]
const CHINESE_YEAR_INFO: [u32; 401] = [
    0x0baa4, 0x06b50, 0x02ba0, 0x0ab62, 0x09570, 0x150e7, 0x0d160, 0x0e4b0, 0x06d25, 0x0da90,
    0x05b50, 0x036d3, 0x02ae0, 0x0a2e0, 0x0e2d2, 0x0c950, 0x0d556, 0x0b520, 0x0b690, 0x05da4,
    0x055d0, 0x025d0, 0x0a5b3, 0x0a2b0, 0x1a8b7, 0x0a950, 0x0b4a0, 0x1b2a5, 0x0ad50, 0x055b0,
    0x02b74, 0x04570, 0x052f9, 0x052b0, 0x06950, 0x06d56, 0x05aa0, 0x0ab50, 0x056d4, 0x04ae0,
    0x0a570, 0x14563, 0x0d2a0, 0x1e8a7, 0x0d550, 0x05aa0, 0x0ada5, 0x095d0, 0x04ae0, 0x0aab4,
    0x0a4d0, 0x0d2b8, 0x0b290, 0x0b550, 0x05757, 0x02da0, 0x095d0, 0x04d75, 0x049b0, 0x0a4b0,
    0x1a4b3, 0x06a90, 0x0ada8, 0x06b50, 0x02b60, 0x19365, 0x09370, 0x04970, 0x06964, 0x0e4a0,
    0x0ea6a, 0x0da90, 0x05ad0, 0x12ad6, 0x02ae0, 0x092e0, 0x0cad5, 0x0c950, 0x0d4a0, 0x1d4a3,
    0x0b650, 0x057a7, 0x055b0, 0x025d0, 0x095b5, 0x092b0, 0x0a950, 0x0d954, 0x0b4a0, 0x0b55c,
    0x0ad50, 0x055b0, 0x02776, 0x02570, 0x052b0, 0x0aab5, 0x06950, 0x06aa0, 0x0baa3, 0x0ab50,
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
    0x0d520, 0x0db27, 0x0b5a0, 0x055d0, 0x04db5, 0x049b0, 0x0a4b0, 0x0d4b4, 0x0aa50, 0x0b559,
    0x06d20, 0x0ad60, 0x05766, 0x09370, 0x04970, 0x06974, 0x054b0, 0x06a50, 0x07a53, 0x06aa0,
    0x1aaa7, 0x0aad0, 0x052e0, 0x0cae5, 0x0a960, 0x0d4a0, 0x1e4a4, 0x0d950, 0x05abb, 0x056a0,
    0x0a6d0, 0x151d6, 0x052d0, 0x0a8d0, 0x1d155, 0x0b2a0, 0x0b550, 0x06d52, 0x055a0, 0x1a5a7,
    0x0a5b0, 0x052b0, 0x0a975, 0x068b0, 0x07290, 0x0baa4, 0x06b50, 0x02dbb, 0x04b60, 0x0a570,
    0x052e6, 0x0d160, 0x0e8b0, 0x06d25, 0x0da90, 0x05b50, 0x036d3, 0x02ae0, 0x0a3d7, 0x0a2d0,
    0x0d150, 0x0d556, 0x0b520, 0x0d690, 0x155a4, 0x055b0, 0x02afa, 0x045b0, 0x0a2b0, 0x0aab6,
    0x0a950, 0x0b4a0, 0x1b2a5, 0x0ad50, 0x055b0, 0x02b73, 0x04570, 0x06377, 0x052b0, 0x06950,
    0x06d56, 0x05aa0, 0x0ab50, 0x056d4, 0x04ae0, 0x0a570, 0x06562, 0x0d2a0, 0x0eaa6, 0x0d550,
    0x05aa0, 0x0aea5, 0x0a6d0, 0x04ae0, 0x0aab3, 0x0a4d0, 0x0d2b7, 0x0b290, 0x0b550, 0x15556,
    0x02da0,
];

const DANGI_FIRST_YEAR: i64 = 1800; // last year 2200 = first + table length − 1
const DANGI_EPOCH_JDN: i64 = 2_378_521; // New Year 1800 = 1800-01-25 (same as Chinese)
#[rustfmt::skip]
const DANGI_YEAR_INFO: [u32; 401] = [
    0x0baa4, 0x06b50, 0x02ba0, 0x0ab62, 0x09570, 0x150e7, 0x0d160, 0x0e4b0, 0x06d25, 0x0da90,
    0x05b50, 0x036d3, 0x02ae0, 0x0a2e0, 0x0e2d2, 0x0c950, 0x0d556, 0x0b520, 0x0b690, 0x05da4,
    0x055d0, 0x025d0, 0x0a5b3, 0x0a2b0, 0x1a8b7, 0x0a950, 0x0b4a0, 0x1b2a5, 0x0ad50, 0x055b0,
    0x02b74, 0x04570, 0x052f9, 0x052b0, 0x06950, 0x06d56, 0x05aa0, 0x0ab50, 0x056d4, 0x04ae0,
    0x0a570, 0x14563, 0x0d2a0, 0x1e8a7, 0x0d550, 0x05aa0, 0x0ada5, 0x095d0, 0x04ae0, 0x0aab4,
    0x0a4d0, 0x0d2b8, 0x0b290, 0x0b550, 0x05757, 0x02da0, 0x095d0, 0x04d75, 0x049b0, 0x0a4b0,
    0x1a4b3, 0x06a90, 0x0ada8, 0x06b50, 0x02b60, 0x19365, 0x09370, 0x04970, 0x06964, 0x0e4a0,
    0x0ea6a, 0x0da90, 0x05ad0, 0x12ad6, 0x02ae0, 0x092e0, 0x0cad5, 0x0c950, 0x0d4a0, 0x1d4a3,
    0x0b650, 0x057a7, 0x055b0, 0x025d0, 0x095b5, 0x092b0, 0x0a950, 0x0d954, 0x0b4a0, 0x0b55c,
    0x0ad50, 0x055b0, 0x02776, 0x02570, 0x052b0, 0x0aab5, 0x06950, 0x06aa0, 0x0baa3, 0x0ab50,
    0x04bd8, 0x04ae0, 0x0a570, 0x054d5, 0x0d260, 0x0d950, 0x16554, 0x056a0, 0x09ad0, 0x055d2,
    0x04ae0, 0x0a5b6, 0x0a4d0, 0x0d250, 0x0da95, 0x0b550, 0x056a0, 0x0ada2, 0x095d0, 0x04bb7,
    0x049b0, 0x0a4b0, 0x0b4b5, 0x06a90, 0x0ad40, 0x0bb54, 0x02b60, 0x095b0, 0x05372, 0x04970,
    0x06566, 0x0e4a0, 0x0ea50, 0x16a95, 0x05b50, 0x02b60, 0x18ae3, 0x092e0, 0x1c8d7, 0x0c950,
    0x0d4a0, 0x1d8a6, 0x0b690, 0x056d0, 0x125b4, 0x025d0, 0x092d0, 0x0d2b2, 0x0a950, 0x0d557,
    0x0b4a0, 0x0b550, 0x15555, 0x04db0, 0x025b0, 0x18573, 0x052b0, 0x0a9b8, 0x06950, 0x06aa0,
    0x0aea6, 0x0ab50, 0x04b60, 0x0aae4, 0x0a570, 0x05270, 0x07263, 0x0d950, 0x06b57, 0x056a0,
    0x09ad0, 0x04dd5, 0x04ae0, 0x0a4e0, 0x0d4d4, 0x0d250, 0x0d598, 0x0b540, 0x0d6a0, 0x195a6,
    0x095b0, 0x049b0, 0x0a9b4, 0x0a4b0, 0x0b27a, 0x06a50, 0x06d40, 0x0b756, 0x02b60, 0x095b0,
    0x04b75, 0x04970, 0x064b0, 0x074a3, 0x0ea50, 0x06d98, 0x05ad0, 0x02b60, 0x096e5, 0x092e0,
    0x0c960, 0x0e954, 0x0d4a0, 0x0da50, 0x07552, 0x056c0, 0x0abb7, 0x025d0, 0x092d0, 0x0cab5,
    0x0a950, 0x0b4a0, 0x1b4a3, 0x0b550, 0x055d9, 0x04ba0, 0x0a5b0, 0x05575, 0x052b0, 0x0a950,
    0x0b954, 0x06aa0, 0x0ad50, 0x06b52, 0x04b60, 0x0a6e6, 0x0a570, 0x05270, 0x06a65, 0x0d930,
    0x05aa0, 0x0b6a3, 0x096d0, 0x04afb, 0x04ae0, 0x0a4d0, 0x1d0d6, 0x0d250, 0x0d520, 0x0dd45,
    0x0b6a0, 0x096d0, 0x055b2, 0x049b0, 0x0a577, 0x0a4b0, 0x0b250, 0x1b255, 0x06d40, 0x0ada0,
    0x18b63, 0x09570, 0x14978, 0x04970, 0x064b0, 0x168a6, 0x0ea50, 0x06b20, 0x1aac4, 0x0ab60,
    0x09370, 0x052e3, 0x0c960, 0x0d557, 0x0d4a0, 0x0da50, 0x05d55, 0x056a0, 0x0aad0, 0x095d4,
    0x092d0, 0x0c9b8, 0x0a950, 0x0b4a0, 0x0b6a6, 0x0ad50, 0x055a0, 0x0aba4, 0x0a5b0, 0x052b0,
    0x0b2b3, 0x0a930, 0x07557, 0x06aa0, 0x0ad50, 0x14b55, 0x04b60, 0x0a570, 0x054f4, 0x05260,
    0x0e968, 0x0d530, 0x05aa0, 0x1aaa6, 0x096d0, 0x04ae0, 0x0aad4, 0x0a4d0, 0x0d260, 0x0f253,
    0x0d520, 0x0db47, 0x0b5a0, 0x096d0, 0x04db5, 0x049b0, 0x0a4b0, 0x0d4b4, 0x0aa50, 0x0b559,
    0x06d20, 0x0ada0, 0x05766, 0x09370, 0x04970, 0x0a974, 0x064b0, 0x06a50, 0x16a53, 0x06b20,
    0x1aaa7, 0x0ab60, 0x09370, 0x04ae5, 0x0c960, 0x0d4a0, 0x1e4a4, 0x0da50, 0x05ad9, 0x056a0,
    0x0a6d0, 0x151d6, 0x092d0, 0x0a950, 0x1d155, 0x0b4a0, 0x0b550, 0x06d52, 0x055a0, 0x1a5a7,
    0x0a5b0, 0x052b0, 0x0aa75, 0x06930, 0x07290, 0x0baa4, 0x0ad50, 0x04dbb, 0x04b60, 0x0a570,
    0x150f6, 0x05160, 0x0e930, 0x06d25, 0x0daa0, 0x06b50, 0x036d3, 0x04ae0, 0x0a5d7, 0x0a2d0,
    0x0d150, 0x0da56, 0x0d520, 0x0da90, 0x155a4, 0x055d0, 0x04afa, 0x049b0, 0x0a2b0, 0x1d0b6,
    0x0aa50, 0x0b520, 0x0bd24, 0x0ada0, 0x055b0, 0x05373, 0x04570, 0x0a377, 0x054b0, 0x06a50,
    0x06d56, 0x06aa0, 0x0ab50, 0x05ad4, 0x052e0, 0x0c570, 0x06962, 0x0d4a0, 0x0eaa7, 0x0d950,
    0x05aa0, 0x0aea5, 0x0a6d0, 0x052e0, 0x0b2d3, 0x0a950, 0x0d557, 0x0b2a0, 0x0b550, 0x15556,
    0x055a0,
];

/// The packed year info for `year`, or `None` when it lies outside `table`'s
/// range (`first_year ..= first_year + table.len() − 1`).
fn ls_info(table: &[u32], first_year: i64, year: i64) -> Option<u32> {
    let last = first_year + table.len() as i64 - 1;
    if (first_year..=last).contains(&year) {
        Some(table[(year - first_year) as usize])
    } else {
        None
    }
}
fn ls_month_days(info: u32, month: i64) -> i64 {
    ((info >> (16 - month)) & 1) as i64 + 29
}
fn ls_leap_days(info: u32) -> i64 {
    ((info >> 16) & 1) as i64 + 29
}
fn ls_year_days(info: u32) -> i64 {
    let leap = (info % 16) as i64;
    let mut sum = 0;
    let mut m = 1;
    while m <= 12 {
        sum += ls_month_days(info, m);
        if leap == m {
            sum += ls_leap_days(info);
        }
        m += 1;
    }
    sum
}

/// The Julian Day Number of a lunisolar date in the given table. Shared by the
/// Chinese and dangi front ends; see [`chinese_to_jdn`] for the argument meaning.
fn ls_to_jdn(
    table: &[u32],
    first_year: i64,
    epoch: i64,
    year: i64,
    month: i64,
    day: i64,
    leap: bool,
) -> Option<i64> {
    let info = ls_info(table, first_year, year)?;
    if leap && (info % 16) as i64 != month {
        return None;
    }
    // `year` is already range-checked by `ls_info`, but `day` is added to the
    // accumulated JDN unguarded (`jdn + day - 1`); an extreme `day` would
    // overflow. A valid lunisolar day is 1..=30, so clamping to the `FWD_LIMIT`
    // band leaves every real date unchanged while preventing the overflow.
    let day = clamp_component(day);
    let mut jdn = epoch;
    let mut y = first_year;
    while y < year {
        jdn += ls_year_days(ls_info(table, first_year, y)?);
        y += 1;
    }
    let leap_m = (info % 16) as i64;
    let mut m = 1;
    while m <= 12 {
        if m == month && !leap {
            return Some(jdn + day - 1);
        }
        jdn += ls_month_days(info, m);
        if leap_m == m {
            if m == month && leap {
                return Some(jdn + day - 1);
            }
            jdn += ls_leap_days(info);
        }
        m += 1;
    }
    None
}

/// The lunisolar `(year, month, day, is_leap_month)` of a Julian Day Number in
/// the given table, or `None` if it falls outside the table's range.
fn jdn_to_ls(
    table: &[u32],
    first_year: i64,
    epoch: i64,
    jdn: i64,
) -> Option<(i64, i64, i64, bool)> {
    let mut offset = jdn - epoch;
    if offset < 0 {
        return None;
    }
    let mut year = first_year;
    loop {
        let yd = ls_year_days(ls_info(table, first_year, year)?);
        if offset < yd {
            break;
        }
        offset -= yd;
        year += 1;
    }
    let info = ls_info(table, first_year, year)?;
    let leap_m = (info % 16) as i64;
    let mut m = 1;
    while m <= 12 {
        let d = ls_month_days(info, m);
        if offset < d {
            return Some((year, m, offset + 1, false));
        }
        offset -= d;
        if leap_m == m {
            let dl = ls_leap_days(info);
            if offset < dl {
                return Some((year, m, offset + 1, true));
            }
            offset -= dl;
        }
        m += 1;
    }
    None
}

/// The Julian Day Number of a Chinese (lunisolar) date. `month` is 1–12; set
/// `leap` for the intercalary month that follows `month`. Returns `None` for a
/// year outside the supported range (1800–2200) or a `leap` month that does not
/// occur that year.
#[must_use]
pub fn chinese_to_jdn(year: i64, month: i64, day: i64, leap: bool) -> Option<i64> {
    ls_to_jdn(
        &CHINESE_YEAR_INFO,
        CHINESE_FIRST_YEAR,
        CHINESE_EPOCH_JDN,
        year,
        month,
        day,
        leap,
    )
}

/// The Chinese `(year, month, day, is_leap_month)` of a Julian Day Number, or
/// `None` if it falls outside the supported range (Chinese years 1800–2200).
#[must_use]
pub fn jdn_to_chinese(jdn: i64) -> Option<(i64, i64, i64, bool)> {
    jdn_to_ls(
        &CHINESE_YEAR_INFO,
        CHINESE_FIRST_YEAR,
        CHINESE_EPOCH_JDN,
        jdn,
    )
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

/// The Julian Day Number of a Korean dangi (lunisolar) date. Same month/`leap`
/// convention as [`chinese_to_jdn`]; the dangi calendar uses the same lunisolar
/// rules computed at the Korean meridian, so its months can differ from the
/// Chinese calendar's in some years. Returns `None` outside the supported range
/// (1800–2200) or for a `leap` month that does not occur that year.
///
/// Note dangi years are conventionally numbered from 2333 BC (Chinese year + 2333);
/// this function takes the equivalent Gregorian-aligned `year` (the same numbering
/// [`chinese_to_jdn`] uses), not the traditional dangi era number.
#[must_use]
pub fn dangi_to_jdn(year: i64, month: i64, day: i64, leap: bool) -> Option<i64> {
    ls_to_jdn(
        &DANGI_YEAR_INFO,
        DANGI_FIRST_YEAR,
        DANGI_EPOCH_JDN,
        year,
        month,
        day,
        leap,
    )
}

/// The dangi `(year, month, day, is_leap_month)` of a Julian Day Number, or
/// `None` if it falls outside the supported range (1800–2200).
#[must_use]
pub fn jdn_to_dangi(jdn: i64) -> Option<(i64, i64, i64, bool)> {
    jdn_to_ls(&DANGI_YEAR_INFO, DANGI_FIRST_YEAR, DANGI_EPOCH_JDN, jdn)
}

/// Convert a Gregorian date to the Korean dangi calendar (`None` if out of range).
#[must_use]
pub fn gregorian_to_dangi(year: i64, month: i64, day: i64) -> Option<(i64, i64, i64, bool)> {
    jdn_to_dangi(gregorian_to_jdn(year, month, day))
}

/// Convert a Korean dangi date to the Gregorian calendar (`None` if out of range).
#[must_use]
pub fn dangi_to_gregorian(year: i64, month: i64, day: i64, leap: bool) -> Option<(i64, i64, i64)> {
    Some(jdn_to_gregorian(dangi_to_jdn(year, month, day, leap)?))
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
