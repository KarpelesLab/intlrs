//! Locale-aware date and time formatting (CLDR / UTS #35, Gregorian).
//! Requires the `alloc` feature.
//!
//! A [`DateTime`] holds the broken-down fields (no calendar arithmetic or time
//! zones); [`format_date`] / [`format_time`] / [`format_datetime`] render it with
//! the locale's CLDR patterns, month/weekday names, and am/pm markers. The
//! weekday is derived from the proleptic Gregorian date.
//!
//! ```
//! use intl::datetime::{DateTime, format_date, format_time, DateStyle};
//! let dt = DateTime { year: 2026, month: 6, day: 4, hour: 14, minute: 30, second: 5 };
//! assert_eq!(format_date("en", &dt, DateStyle::Long), "June 4, 2026");
//! assert_eq!(format_date("en", &dt, DateStyle::Medium), "Jun 4, 2026");
//! assert_eq!(format_time("en", &dt, DateStyle::Short), "2:30\u{202f}PM");
//! ```

use crate::cldr::{calendar_spec, CalendarSpec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// A broken-down Gregorian date and time. Fields are not validated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    /// Proleptic Gregorian year (e.g. 2026).
    pub year: i32,
    /// Month, 1–12.
    pub month: u8,
    /// Day of month, 1–31.
    pub day: u8,
    /// Hour, 0–23.
    pub hour: u8,
    /// Minute, 0–59.
    pub minute: u8,
    /// Second, 0–59.
    pub second: u8,
}

impl DateTime {
    /// Render as a machine-readable ISO-8601 timestamp
    /// (`YYYY-MM-DDTHH:MM:SS`), independent of locale.
    #[must_use]
    pub fn to_iso8601(&self) -> String {
        alloc::format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second
        )
    }

    /// Parse an ISO-8601 timestamp such as `"2026-06-04T14:30:05"`. Accepts a
    /// space instead of `T`, an omitted time or seconds, and a trailing `Z`.
    /// Returns `None` if malformed. (A time-zone offset, if present, is ignored.)
    #[must_use]
    pub fn parse_iso8601(s: &str) -> Option<DateTime> {
        let s = s.trim().trim_end_matches('Z');
        let (date, time) = match s.split_once(['T', ' ']) {
            Some((d, t)) => (d, Some(t)),
            None => (s, None),
        };
        let mut dp = date.split('-');
        let year: i32 = dp.next()?.parse().ok()?;
        let month: u8 = dp.next()?.parse().ok()?;
        let day: u8 = dp.next()?.parse().ok()?;
        if dp.next().is_some() || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
            return None;
        }
        let (hour, minute, second) = match time {
            None => (0, 0, 0),
            Some(t) => {
                // Drop any zone offset on the time component.
                let t = t.split(['+', '-']).next().unwrap_or(t);
                let mut tp = t.split(':');
                let h: u8 = tp.next()?.parse().ok()?;
                let mi: u8 = tp.next()?.parse().ok()?;
                let se: u8 = match tp.next() {
                    Some(x) => x.parse().ok()?,
                    None => 0,
                };
                (h, mi, se)
            }
        };
        Some(DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }
}

/// One of the four CLDR length styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateStyle {
    /// Full ("Thursday, June 4, 2026").
    Full,
    /// Long ("June 4, 2026").
    Long,
    /// Medium ("Jun 4, 2026").
    Medium,
    /// Short ("6/4/26").
    Short,
}

impl DateStyle {
    fn idx(self) -> usize {
        match self {
            DateStyle::Full => 0,
            DateStyle::Long => 1,
            DateStyle::Medium => 2,
            DateStyle::Short => 3,
        }
    }
}

/// Day of week for a proleptic Gregorian date: 0 = Sunday … 6 = Saturday
/// (Sakamoto's algorithm).
fn weekday(dt: &DateTime) -> usize {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let m = dt.month as i32;
    let d = dt.day as i32;
    let y = if m < 3 { dt.year - 1 } else { dt.year };
    (((y + y / 4 - y / 100 + y / 400 + t[(m - 1) as usize] + d) % 7 + 7) % 7) as usize
}

fn two(n: i64) -> String {
    alloc::format!("{n:02}")
}

/// Render one date-field run (`field` repeated `n` times) of a CLDR pattern.
fn field(field: char, n: usize, dt: &DateTime, s: &CalendarSpec) -> String {
    let m = dt.month as usize;
    match field {
        'y' | 'Y' => {
            if n == 2 {
                two((dt.year.rem_euclid(100)) as i64)
            } else {
                dt.year.to_string()
            }
        }
        'M' | 'L' => match n {
            1 => m.to_string(),
            2 => two(m as i64),
            3 => s.months_abbr[m - 1].to_string(),
            _ => s.months_wide[m - 1].to_string(),
        },
        'd' => {
            if n >= 2 {
                two(dt.day as i64)
            } else {
                dt.day.to_string()
            }
        }
        'E' | 'e' | 'c' => {
            let w = weekday(dt);
            if n >= 4 {
                s.days_wide[w].to_string()
            } else {
                s.days_abbr[w].to_string()
            }
        }
        'h' => {
            let h = ((dt.hour + 11) % 12) + 1; // 12-hour clock
            if n >= 2 {
                two(h as i64)
            } else {
                h.to_string()
            }
        }
        'H' => {
            if n >= 2 {
                two(dt.hour as i64)
            } else {
                dt.hour.to_string()
            }
        }
        'm' => {
            if n >= 2 {
                two(dt.minute as i64)
            } else {
                dt.minute.to_string()
            }
        }
        's' => {
            if n >= 2 {
                two(dt.second as i64)
            } else {
                dt.second.to_string()
            }
        }
        'a' | 'b' => if dt.hour < 12 { s.am } else { s.pm }.to_string(),
        _ => String::new(), // unsupported field (e.g. time zone) -> nothing
    }
}

/// Interpret a CLDR date/time pattern, handling quoted literals.
fn render(pattern: &str, dt: &DateTime, s: &CalendarSpec) -> String {
    let c: Vec<char> = pattern.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < c.len() {
        let ch = c[i];
        if ch == '\'' {
            i += 1;
            if i < c.len() && c[i] == '\'' {
                out.push('\'');
                i += 1;
                continue;
            }
            while i < c.len() && c[i] != '\'' {
                out.push(c[i]);
                i += 1;
            }
            i += 1; // closing quote
        } else if ch.is_ascii_alphabetic() {
            let start = i;
            while i < c.len() && c[i] == ch {
                i += 1;
            }
            out.push_str(&field(ch, i - start, dt, s));
        } else {
            out.push(ch);
            i += 1;
        }
    }
    out
}

fn spec(lang: &str) -> CalendarSpec {
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    loop {
        if let Some(s) = calendar_spec(&norm[..end]) {
            return s;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => return calendar_spec("en").expect("root calendar present"),
        }
    }
}

/// Format the date part of `dt` in `lang` at the given `style`.
#[must_use]
pub fn format_date(lang: &str, dt: &DateTime, style: DateStyle) -> String {
    let s = spec(lang);
    render(s.date[style.idx()], dt, &s)
}

/// Format the time part of `dt` in `lang` at the given `style`.
#[must_use]
pub fn format_time(lang: &str, dt: &DateTime, style: DateStyle) -> String {
    let s = spec(lang);
    render(s.time[style.idx()], dt, &s)
}

/// Format `dt` with a CLDR *skeleton* (e.g. `"yMMMd"`, `"Hm"`, `"MMMEd"`) — the
/// modern, flexible date API: the skeleton names the fields you want and the
/// locale supplies their order and punctuation. Falls back through the locale
/// chain (and to English), then to the medium date pattern for an unknown
/// skeleton.
///
/// ```
/// use intl::datetime::{DateTime, format_skeleton};
/// let dt = DateTime { year: 2026, month: 6, day: 4, hour: 14, minute: 30, second: 5 };
/// assert_eq!(format_skeleton("en", &dt, "yMMMd"), "Jun 4, 2026");
/// assert_eq!(format_skeleton("de", &dt, "yMMMd"), "4. Juni 2026");
/// assert_eq!(format_skeleton("en", &dt, "Hm"), "14:30");
/// ```
#[must_use]
pub fn format_skeleton(lang: &str, dt: &DateTime, skeleton: &str) -> String {
    let s = spec(lang);
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    let pattern = loop {
        if let Some(p) = crate::cldr::skeleton_pattern(&norm[..end], skeleton) {
            break p;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => break crate::cldr::skeleton_pattern("en", skeleton).unwrap_or(s.date[2]),
        }
    };
    render(pattern, dt, &s)
}

/// Format an Islamic (Hijri) date in `lang`, e.g.
/// `format_islamic_date("en", 1445, 9, 1, DateStyle::Long)` →
/// `"Ramadan 1, 1445 AH"`. Uses the localized Islamic month names and era; the
/// weekday name (if the pattern has one) comes from the Gregorian day names.
#[must_use]
pub fn format_islamic_date(
    lang: &str,
    year: i64,
    month: i64,
    day: i64,
    style: DateStyle,
) -> String {
    let isl = islamic_spec(lang);
    let greg = spec(lang); // for weekday names
    let jdn = crate::calendar::islamic_to_jdn(year, month, day);
    // Gregorian day-name index (0 = Sunday).
    let wd = ((jdn.rem_euclid(7) + 1) % 7) as usize;

    let pattern = isl.date[style.idx()];
    let c: Vec<char> = pattern.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < c.len() {
        let ch = c[i];
        if ch == '\'' {
            i += 1;
            if i < c.len() && c[i] == '\'' {
                out.push('\'');
                i += 1;
                continue;
            }
            while i < c.len() && c[i] != '\'' {
                out.push(c[i]);
                i += 1;
            }
            i += 1;
        } else if ch.is_ascii_alphabetic() {
            let start = i;
            while i < c.len() && c[i] == ch {
                i += 1;
            }
            let n = i - start;
            let m = month as usize;
            match ch {
                'y' | 'Y' => out.push_str(&year.to_string()),
                'M' | 'L' => match n {
                    1 => out.push_str(&m.to_string()),
                    2 => out.push_str(&two(m as i64)),
                    3 => out.push_str(isl.months_abbr[m - 1]),
                    _ => out.push_str(isl.months_wide[m - 1]),
                },
                'd' => out.push_str(&day.to_string()),
                'E' | 'e' | 'c' => out.push_str(if n >= 4 {
                    greg.days_wide[wd]
                } else {
                    greg.days_abbr[wd]
                }),
                'G' => out.push_str(isl.era),
                _ => {}
            }
        } else {
            out.push(ch);
            i += 1;
        }
    }
    out
}

fn islamic_spec(lang: &str) -> crate::cldr::IslamicSpec {
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    loop {
        if let Some(s) = crate::cldr::islamic_spec(&norm[..end]) {
            return s;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => return crate::cldr::islamic_spec("en").expect("root islamic present"),
        }
    }
}

/// Format a fixed UTC offset (in minutes) in the localized GMT form, e.g.
/// `"GMT+5:30"`-style output: `format_gmt_offset("en", 330)` → `"GMT+05:30"`,
/// `format_gmt_offset("fr", -480)` → `"UTC−08:00"`, `0` → `"GMT"` / `"UTC"`.
/// This is the data-light part of time-zone support (a concrete offset, not the
/// IANA zone database).
#[must_use]
pub fn format_gmt_offset(lang: &str, offset_minutes: i32) -> String {
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    let tz = loop {
        if let Some(t) = crate::cldr::tz_spec(&norm[..end]) {
            break t;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => break crate::cldr::tz_spec("en").expect("root tz present"),
        }
    };
    if offset_minutes == 0 {
        return String::from(tz.zero);
    }
    let (pos, neg) = tz.hour.split_once(';').unwrap_or((tz.hour, tz.hour));
    let sub = if offset_minutes >= 0 { pos } else { neg };
    let (h, m) = (offset_minutes.abs() / 60, offset_minutes.abs() % 60);
    let body = sub
        .replace("HH", &alloc::format!("{h:02}"))
        .replace("mm", &alloc::format!("{m:02}"));
    tz.gmt.replace("{0}", &body)
}

/// Format both date and time, combined with the locale's date+time pattern.
#[must_use]
pub fn format_datetime(
    lang: &str,
    dt: &DateTime,
    date_style: DateStyle,
    time_style: DateStyle,
) -> String {
    let s = spec(lang);
    let date = render(s.date[date_style.idx()], dt, &s);
    let time = render(s.time[time_style.idx()], dt, &s);
    s.datetime[date_style.idx()]
        .replace("{1}", &date)
        .replace("{0}", &time)
}
