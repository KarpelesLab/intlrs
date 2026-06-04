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
//! let jul = DateTime { year: 2026, month: 7, day: 1, hour: 12, minute: 0, second: 0 };
//! let jan = DateTime { year: 2026, month: 1, day: 1, hour: 12, minute: 0, second: 0 };
//! assert_eq!(tz.offset_seconds(&jul), -7 * 3600);
//! assert_eq!(tz.offset_seconds(&jan), -8 * 3600);
//! ```

use crate::datetime::DateTime;

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
    let secs = parts[0] * 3600 + parts[1] * 60 + parts[2];
    // POSIX offsets are seconds *west* of UTC, so negate for an east-positive value.
    Some((if neg { secs } else { -secs }, i))
}

/// Parse an `Mm.w.d[/time]` rule. Only the `M` form is supported.
fn parse_rule(s: &str) -> Option<Rule> {
    let s = s.strip_prefix('M')?;
    let (spec, time) = match s.split_once('/') {
        // The time after '/' is an unsigned h[:mm[:ss]]; take its magnitude.
        Some((a, b)) => (a, parse_offset(b)?.0.unsigned_abs() as i32),
        None => (s, 2 * 3600),
    };
    let mut it = spec.split('.');
    let month: u8 = it.next()?.parse().ok()?;
    let week: u8 = it.next()?.parse().ok()?;
    let dow: u8 = it.next()?.parse().ok()?;
    if it.next().is_some() {
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
            (std_offset + 3600, 0)
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
        if in_dst {
            dst_offset
        } else {
            self.std_offset
        }
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
    if i == 0 {
        None
    } else {
        Some(i)
    }
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
