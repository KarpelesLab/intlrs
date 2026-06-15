//! POSIX `TZ`-string time zones (`no_std`, no `alloc`): parse a rule string like
//! `"PST8PDT,M3.2.0,M11.1.0/2"` and compute the UTC offset for any local date
//! using its standard/DST rules. This handles the *current rules* of a zone
//! without the full IANA historical database.
//!
//! ```
//! use intl::timezone::PosixTz;
//! use intl::datetime::DateTime;
//! let tz = PosixTz::parse("PST8PDT,M3.2.0,M11.1.0/2").unwrap();
//! // July is daylight time (UTC−7); January is standard time (UTC−8).
//! let jul = DateTime { year: 2026, month: 7, day: 1, hour: 12, minute: 0, second: 0, millisecond: 0 };
//! let jan = DateTime { year: 2026, month: 1, day: 1, hour: 12, minute: 0, second: 0, millisecond: 0 };
//! assert_eq!(tz.offset_seconds(&jul), -7 * 3600);
//! assert_eq!(tz.offset_seconds(&jan), -8 * 3600);
//! ```

use crate::datetime::DateTime;

#[cfg(feature = "iana-tz")]
pub use iana::{IanaZone, load_zone, zone_names};

/// Full IANA time-zone database support (the `iana-tz` feature), backed by the
/// embedded `timezone-data` crate.
#[cfg(feature = "iana-tz")]
mod iana {
    use crate::datetime::DateTime;

    /// Julian Day Number of the Unix epoch (1970-01-01).
    const UNIX_EPOCH_JDN: i64 = 2_440_588;

    fn to_unix(dt: &DateTime) -> i64 {
        let jdn = crate::calendar::gregorian_to_jdn(dt.year as i64, dt.month as i64, dt.day as i64);
        (jdn - UNIX_EPOCH_JDN) * 86_400
            + dt.hour as i64 * 3600
            + dt.minute as i64 * 60
            + dt.second as i64
    }

    fn from_unix(secs: i64) -> DateTime {
        let (days, sod) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
        let (y, m, d) = crate::calendar::jdn_to_gregorian(UNIX_EPOCH_JDN + days);
        DateTime {
            year: y as i32,
            month: m as u8,
            day: d as u8,
            hour: (sod / 3600) as u8,
            minute: (sod % 3600 / 60) as u8,
            second: (sod % 60) as u8,
            millisecond: 0,
        }
    }

    /// A loaded IANA time zone (e.g. `"America/New_York"`) with its full history
    /// of UTC-offset/DST transitions.
    pub struct IanaZone(timezone_data::Zone<'static>);

    /// Load an IANA zone by name. Returns `None` for an unknown name. Lookups are
    /// case-sensitive (`"America/New_York"`).
    #[must_use]
    pub fn load_zone(name: &str) -> Option<IanaZone> {
        timezone_data::load(name).ok().map(IanaZone)
    }

    impl IanaZone {
        /// UTC offset (seconds east of UTC) in effect at the UTC instant `unix`.
        #[must_use]
        pub fn offset_at(&self, unix: i64) -> i32 {
            self.0.lookup(unix).offset
        }

        /// The zone abbreviation (e.g. `"EST"` / `"EDT"`) at the UTC instant `unix`.
        #[must_use]
        pub fn abbrev_at(&self, unix: i64) -> &'static str {
            self.0.lookup(unix).abbrev
        }

        /// Whether daylight time is in effect at the UTC instant `unix`.
        #[must_use]
        pub fn is_dst_at(&self, unix: i64) -> bool {
            self.0.lookup(unix).is_dst
        }

        /// The local broken-down time in this zone for the UTC instant `unix`.
        #[must_use]
        pub fn to_local(&self, unix: i64) -> DateTime {
            from_unix(unix + self.0.lookup(unix).offset as i64)
        }

        /// The UTC offset (seconds east) for a *local* date-time in this zone.
        /// (At the one ambiguous hour of a DST transition this picks one side.)
        #[must_use]
        pub fn offset_for_local(&self, dt: &DateTime) -> i32 {
            let approx = to_unix(dt);
            let off = self.0.lookup(approx).offset as i64;
            self.0.lookup(approx - off).offset
        }
    }

    /// Iterate over every IANA zone name in the embedded database.
    pub fn zone_names() -> impl Iterator<Item = &'static str> {
        timezone_data::names()
    }
}

/// A daylight-saving transition rule in the `Mm.w.d[/time]` form: the `d`th
/// weekday of week `w` in month `m`, at `time` seconds after local midnight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rule {
    month: u8, // 1..=12
    week: u8,  // 1..=5 (5 = last)
    dow: u8,   // 0 = Sunday .. 6 = Saturday
    time: i32, // seconds after midnight (default 2:00)
}

/// A parsed POSIX `TZ` zone: a standard offset and optional daylight rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PosixTz {
    /// UTC offset of standard time, in seconds (east positive).
    std_offset: i32,
    /// `Some((dst_offset, start, end))` if the zone observes daylight time.
    dst: Option<(i32, Rule, Rule)>,
}

/// Parse a signed `h[:mm[:ss]]` offset (POSIX-positive = west of UTC) into the
/// east-positive UTC offset in seconds.
fn parse_offset(s: &str) -> Option<(i32, usize)> {
    let bytes = s.as_bytes();
    let mut i = 0;
    let neg = match bytes.first() {
        Some(b'-') => {
            i += 1;
            true
        }
        Some(b'+') => {
            i += 1;
            false
        }
        _ => false,
    };
    let start = i;
    let mut parts = [0i32; 3];
    let mut p = 0;
    while p < 3 {
        let d0 = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i == d0 {
            return None;
        }
        parts[p] = s[d0..i].parse().ok()?;
        p += 1;
        if i < bytes.len() && bytes[i] == b':' {
            i += 1;
        } else {
            break;
        }
    }
    if i == start {
        return None;
    }
    let secs = parts[0]
        .checked_mul(3600)?
        .checked_add(parts[1].checked_mul(60)?)?
        .checked_add(parts[2])?;
    // POSIX offsets are seconds *west* of UTC, so negate for an east-positive value.
    Some((if neg { secs } else { -secs }, i))
}

/// Parse an `Mm.w.d[/time]` rule. Only the `M` form is supported.
fn parse_rule(s: &str) -> Option<Rule> {
    let s = s.strip_prefix('M')?;
    let (spec, time) = match s.split_once('/') {
        // The time after '/' is an unsigned h[:mm[:ss]]; take its magnitude.
        // `parse_offset` returns the east-positive value (the negated magnitude for
        // an unsigned input), so negate once more to recover the original magnitude
        // as an i32 directly, avoiding a u32->i32 wrap.
        Some((a, b)) => (a, parse_offset(b)?.0.checked_neg()?),
        None => (s, 2 * 3600),
    };
    let mut it = spec.split('.');
    let month: u8 = it.next()?.parse().ok()?;
    let week: u8 = it.next()?.parse().ok()?;
    let dow: u8 = it.next()?.parse().ok()?;
    if it.next().is_some() {
        return None;
    }
    // POSIX requires month 1–12, week 1–5, and day-of-week 0–6. Reject
    // out-of-range fields rather than silently producing wrong transitions.
    if !((1..=12).contains(&month) && (1..=5).contains(&week) && dow <= 6) {
        return None;
    }
    Some(Rule {
        month,
        week,
        dow,
        time,
    })
}

impl PosixTz {
    /// Parse a POSIX `TZ` string. Returns `None` if it is malformed or uses an
    /// unsupported (non-`M`) DST rule form.
    #[must_use]
    pub fn parse(tz: &str) -> Option<PosixTz> {
        // std-abbr std-offset [dst-abbr [dst-offset] , start , end]
        let after_std_name = skip_name(tz)?;
        let (std_offset, n) = parse_offset(&tz[after_std_name..])?;
        let mut rest = &tz[after_std_name + n..];
        if rest.is_empty() {
            return Some(PosixTz {
                std_offset,
                dst: None,
            });
        }
        // DST abbreviation, then optional offset (default = std + 1h).
        let after_dst_name = skip_name(rest)?;
        let dst_off_str = &rest[after_dst_name..];
        let (dst_offset, used) = if dst_off_str.starts_with(',') {
            (std_offset.checked_add(3600)?, 0)
        } else {
            parse_offset(dst_off_str)?
        };
        rest = &dst_off_str[used..];
        let rules = rest.strip_prefix(',')?;
        let (start, end) = rules.split_once(',')?;
        Some(PosixTz {
            std_offset,
            dst: Some((dst_offset, parse_rule(start)?, parse_rule(end)?)),
        })
    }

    /// The UTC offset (seconds, east positive) in effect at local date-time `dt`.
    #[must_use]
    pub fn offset_seconds(&self, dt: &DateTime) -> i32 {
        let Some((dst_offset, start, end)) = self.dst else {
            return self.std_offset;
        };
        let now = local_seconds(dt);
        let s = rule_seconds(start, dt.year as i64);
        let e = rule_seconds(end, dt.year as i64);
        let in_dst = if s < e {
            now >= s && now < e // northern hemisphere
        } else {
            now >= s || now < e // southern hemisphere (DST wraps the year)
        };
        if in_dst { dst_offset } else { self.std_offset }
    }

    /// `true` if daylight time is in effect at `dt`.
    #[must_use]
    pub fn is_dst(&self, dt: &DateTime) -> bool {
        self.dst.is_some_and(|(d, ..)| self.offset_seconds(dt) == d)
    }
}

/// Skip a zone abbreviation: either `<...>` quoted or a run of letters.
fn skip_name(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    if b.first() == Some(&b'<') {
        return s.find('>').map(|i| i + 1);
    }
    let mut i = 0;
    while i < b.len() && b[i].is_ascii_alphabetic() {
        i += 1;
    }
    if i == 0 { None } else { Some(i) }
}

/// Seconds since the start of `dt`'s year for the date-time `dt`.
fn local_seconds(dt: &DateTime) -> i64 {
    let jan1 = crate::calendar::gregorian_to_jdn(dt.year as i64, 1, 1);
    let jdn = crate::calendar::gregorian_to_jdn(dt.year as i64, dt.month as i64, dt.day as i64);
    (jdn - jan1) * 86_400 + dt.hour as i64 * 3600 + dt.minute as i64 * 60 + dt.second as i64
}

/// Seconds since the start of `year` for the instant named by `rule`.
fn rule_seconds(rule: Rule, year: i64) -> i64 {
    let jan1 = crate::calendar::gregorian_to_jdn(year, 1, 1);
    // First day-of-`month`, and its weekday (0 = Sunday).
    let first = crate::calendar::gregorian_to_jdn(year, rule.month as i64, 1);
    let first_dow = (first.rem_euclid(7) + 1) % 7; // 0 = Sunday
    let mut day = 1 + (rule.dow as i64 - first_dow).rem_euclid(7) + (rule.week as i64 - 1) * 7;
    // Week 5 means "last": clamp into the month.
    let dim = days_in_month(year, rule.month as i64);
    while day > dim {
        day -= 7;
    }
    let jdn = crate::calendar::gregorian_to_jdn(year, rule.month as i64, day);
    (jdn - jan1) * 86_400 + rule.time as i64
}

fn days_in_month(year: i64, month: i64) -> i64 {
    let next = crate::calendar::gregorian_to_jdn(
        if month == 12 { year + 1 } else { year },
        if month == 12 { 1 } else { month + 1 },
        1,
    );
    next - crate::calendar::gregorian_to_jdn(year, month, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_overflow_returns_none() {
        // `parts[0] * 3600` overflows i32 for values > ~596523; checked arithmetic
        // must yield `None` rather than panicking (debug) or silently wrapping.
        assert!(parse_offset("600000").is_none());
        assert!(PosixTz::parse("X600000").is_none());
    }

    #[test]
    fn default_dst_offset_overflow_returns_none() {
        // A near-i32::MAX std offset with an implicit DST offset (default std+1h)
        // must not overflow when adding 3600.
        assert!(PosixTz::parse("STD-596523DST,M3.2.0,M11.1.0").is_none());
    }

    #[test]
    fn rule_field_ranges_are_validated() {
        // The common US DST rules (month 1–12, week 1–5, dow 0–6) must parse.
        assert!(parse_rule("M3.2.0").is_some());
        assert!(parse_rule("M11.1.0").is_some());
        assert!(parse_rule("M12.5.6").is_some());
        // Out-of-range month/week/dow must be rejected rather than yielding a
        // silently-wrong DST transition.
        assert!(parse_rule("M0.0.0").is_none());
        assert!(parse_rule("M13.9.9").is_none());
        assert!(parse_rule("M99.99.255").is_none());
        assert!(parse_rule("M3.0.0").is_none()); // week 0
        assert!(parse_rule("M3.6.0").is_none()); // week 6
        assert!(parse_rule("M3.2.7").is_none()); // dow 7
        // A full TZ string carrying a malformed rule must fail to parse.
        assert!(PosixTz::parse("PST8PDT,M13.2.0,M11.1.0/2").is_none());
        assert!(PosixTz::parse("PST8PDT,M3.2.0,M11.1.0/2").is_some());
    }
}
